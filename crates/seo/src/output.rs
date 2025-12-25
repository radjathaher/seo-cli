use anyhow::Result;
use indexmap::IndexSet;
use serde_json::Value;
use std::io::{self, Write};

#[derive(Clone, Copy, Debug)]
pub enum OutputFormat {
    Tsv,
    Json,
}

pub fn print_output(value: &Value, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => {
            let out = serde_json::to_string_pretty(value)?;
            write_stdout(&out)
        }
        OutputFormat::Tsv => print_tsv(value),
    }
}

pub fn print_rows_tsv(
    rows: &[serde_json::Map<String, Value>],
    columns: Option<&[&str]>,
) -> Result<()> {
    match columns {
        Some(cols) => print_rows_as_tsv_fixed(rows, cols),
        None => print_rows_as_tsv_union(rows),
    }
}

fn print_tsv(value: &Value) -> Result<()> {
    let mut out = io::BufWriter::new(io::stdout());
    let res: io::Result<()> = (|| {
        if let Some(rows) = extract_rows(value) {
            return print_rows_tsv_to(&mut out, &rows, None);
        }

        let mut kv = Vec::new();
        flatten_json(value, "$".to_string(), &mut kv);
        writeln!(out, "path\tvalue")?;
        for (k, v) in kv {
            writeln!(out, "{k}\t{v}")?;
        }
        Ok(())
    })();
    handle_stdout_result(res)
}

fn extract_rows(value: &Value) -> Option<Vec<serde_json::Map<String, Value>>> {
    let tasks = value.get("tasks")?.as_array()?;
    let mut results = Vec::new();

    let mut item_rows = Vec::new();
    for task in tasks {
        if let Some(arr) = task.get("result").and_then(|v| v.as_array()) {
            for item in arr {
                if let Some(items) = item.get("items").and_then(|v| v.as_array()) {
                    for it in items {
                        if let Some(obj) = it.as_object() {
                            item_rows.push(obj.clone());
                        }
                    }
                }
            }
        }
    }
    if !item_rows.is_empty() {
        return Some(flatten_rows(&item_rows));
    }

    let mut result_rows = Vec::new();
    for task in tasks {
        if let Some(arr) = task.get("result").and_then(|v| v.as_array()) {
            for item in arr {
                if let Some(obj) = item.as_object() {
                    result_rows.push(obj.clone());
                }
            }
        }
    }
    if !result_rows.is_empty() {
        return Some(flatten_rows(&result_rows));
    }

    for task in tasks {
        if let Some(obj) = task.as_object() {
            results.push(obj.clone());
        }
    }

    if results.is_empty() {
        None
    } else {
        Some(flatten_rows(&results))
    }
}

fn print_rows_as_tsv_union(rows: &[serde_json::Map<String, Value>]) -> Result<()> {
    let mut out = io::BufWriter::new(io::stdout());
    let res: io::Result<()> = print_rows_tsv_to(&mut out, rows, None);
    handle_stdout_result(res)
}

fn print_rows_as_tsv_fixed(rows: &[serde_json::Map<String, Value>], cols: &[&str]) -> Result<()> {
    let mut out = io::BufWriter::new(io::stdout());
    let res: io::Result<()> = print_rows_tsv_to(&mut out, rows, Some(cols));
    handle_stdout_result(res)
}

fn value_to_cell(v: &Value) -> String {
    match v {
        Value::Null => "".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.replace('\n', " "),
        _ => serde_json::to_string(v).unwrap_or_default(),
    }
}

fn flatten_rows(rows: &[serde_json::Map<String, Value>]) -> Vec<serde_json::Map<String, Value>> {
    rows.iter().map(flatten_row_map).collect()
}

fn flatten_row_map(row: &serde_json::Map<String, Value>) -> serde_json::Map<String, Value> {
    let mut out = serde_json::Map::new();
    for (k, v) in row {
        flatten_value(&mut out, k, v);
    }
    out
}

fn flatten_value(out: &mut serde_json::Map<String, Value>, prefix: &str, value: &Value) {
    match value {
        Value::Object(map) => {
            if map.is_empty() {
                out.insert(prefix.to_string(), Value::Null);
                return;
            }
            for (k, v) in map {
                let key = format!("{prefix}.{k}");
                flatten_value(out, &key, v);
            }
        }
        Value::Array(arr) => {
            let s = serde_json::to_string(arr).unwrap_or_default();
            out.insert(prefix.to_string(), Value::String(s));
        }
        _ => {
            out.insert(prefix.to_string(), value.clone());
        }
    }
}

fn flatten_json(value: &Value, prefix: String, out: &mut Vec<(String, String)>) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let next = format!("{prefix}.{}", k);
                flatten_json(v, next, out);
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let next = format!("{prefix}[{i}]");
                flatten_json(v, next, out);
            }
        }
        _ => {
            let val = value_to_cell(value);
            out.push((prefix, val));
        }
    }
}

fn print_rows_tsv_to(
    out: &mut impl Write,
    rows: &[serde_json::Map<String, Value>],
    columns: Option<&[&str]>,
) -> io::Result<()> {
    let cols: Vec<String> = match columns {
        Some(cols) => cols.iter().map(|s| s.to_string()).collect(),
        None => {
            let mut columns: IndexSet<String> = IndexSet::new();
            for row in rows {
                for key in row.keys() {
                    columns.insert(key.clone());
                }
            }
            columns.into_iter().collect()
        }
    };

    writeln!(out, "{}", cols.join("\t"))?;
    for row in rows {
        let mut values = Vec::with_capacity(cols.len());
        for col in cols.iter() {
            let val = row.get(col).unwrap_or(&Value::Null);
            values.push(value_to_cell(val));
        }
        writeln!(out, "{}", values.join("\t"))?;
    }
    Ok(())
}

fn write_stdout(s: &str) -> Result<()> {
    let mut out = io::BufWriter::new(io::stdout());
    let res: io::Result<()> = (|| {
        out.write_all(s.as_bytes())?;
        out.write_all(b"\n")?;
        out.flush()?;
        Ok(())
    })();
    handle_stdout_result(res)
}

fn handle_stdout_result(res: io::Result<()>) -> Result<()> {
    match res {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(e) => Err(e.into()),
    }
}
