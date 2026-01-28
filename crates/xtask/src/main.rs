use anyhow::{Context, Result};
use serde_json::Value as JsonValue;
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        anyhow::bail!("usage: xtask gen");
    }
    match args.remove(0).as_str() {
        "gen" => generate(),
        other => anyhow::bail!("unknown xtask command: {other}"),
    }
}

fn generate() -> Result<()> {
    let root = repo_root()?;
    let spec_path = root.join("openapi.yaml");
    let raw = fs::read_to_string(&spec_path).context("read openapi.yaml")?;
    let doc: Value = serde_yaml::from_str(&raw).context("parse openapi.yaml")?;

    let schemas = doc
        .get("components")
        .and_then(|v| v.get("schemas"))
        .and_then(|v| v.as_mapping())
        .context("missing components.schemas")?;

    let mut schema_map: BTreeMap<String, Value> = BTreeMap::new();
    for (k, v) in schemas {
        if let Some(name) = k.as_str() {
            schema_map.insert(name.to_string(), v.clone());
        }
    }

    let paths = doc
        .get("paths")
        .and_then(|v| v.as_mapping())
        .context("missing paths")?;

    let mut ops = collect_operations(paths)?;
    assign_unique_idents(&mut ops);

    let gen_dir = root.join("crates/seo/src/generated");
    fs::create_dir_all(&gen_dir).context("create generated dir")?;

    let models = render_models(&schema_map)?;
    fs::write(gen_dir.join("models.rs"), models).context("write models.rs")?;

    let meta = render_meta(&ops, &schema_map)?;
    fs::write(gen_dir.join("meta.rs"), meta).context("write meta.rs")?;

    let docs = render_docs(&root.join("docs/docs_map.json"))?;
    fs::write(gen_dir.join("docs.rs"), docs).context("write docs.rs")?;

    let ops_rs = render_ops(&ops)?;
    fs::write(gen_dir.join("ops.rs"), ops_rs).context("write ops.rs")?;

    Ok(())
}

fn repo_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("openapi.yaml").exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    anyhow::bail!("repo root not found")
}

#[derive(Clone, Debug)]
struct Operation {
    operation_id: String,
    ident: String,
    method: String,
    path: String,
    description: String,
    request_schema: Option<String>,
    response_schema: Option<String>,
    path_params: Vec<String>,
    request_example: Option<JsonValue>,
}

fn collect_operations(paths: &serde_yaml::Mapping) -> Result<Vec<Operation>> {
    let mut ops = Vec::new();
    for (path_key, path_val) in paths {
        let path = path_key.as_str().context("path key")?.to_string();
        let item = path_val.as_mapping().context("path item")?;
        for (method_key, op_val) in item {
            let method = method_key.as_str().unwrap_or("").to_lowercase();
            if method != "get" && method != "post" {
                continue;
            }
            let op = op_val.as_mapping().context("op mapping")?;
            let operation_id = op
                .get(&Value::String("operationId".into()))
                .and_then(|v| v.as_str())
                .context("operationId")?
                .to_string();
            let ident = operation_id.clone();
            let description = op
                .get(&Value::String("description".into()))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let path_params = extract_path_params(op);

            let (request_schema, request_example) = if method == "post" {
                extract_request_schema(op)
            } else {
                (None, None)
            };
            let response_schema = extract_response_schema(op);

            ops.push(Operation {
                operation_id,
                ident,
                method,
                path: path.clone(),
                description,
                request_schema,
                response_schema,
                path_params,
                request_example,
            });
        }
    }
    Ok(ops)
}

fn extract_path_params(op: &serde_yaml::Mapping) -> Vec<String> {
    let mut params = Vec::new();
    if let Some(params_val) = op.get(&Value::String("parameters".into())) {
        if let Some(seq) = params_val.as_sequence() {
            for p in seq {
                if let Some(m) = p.as_mapping() {
                    let loc = m.get(&Value::String("in".into())).and_then(|v| v.as_str());
                    if loc != Some("path") {
                        continue;
                    }
                    if let Some(name) = m
                        .get(&Value::String("name".into()))
                        .and_then(|v| v.as_str())
                    {
                        params.push(name.to_string());
                    }
                }
            }
        }
    }
    params
}

fn extract_request_schema(op: &serde_yaml::Mapping) -> (Option<String>, Option<JsonValue>) {
    let schema = op
        .get(&Value::String("requestBody".into()))
        .and_then(|v| v.get(&Value::String("content".into())))
        .and_then(|v| v.get(&Value::String("application/json".into())))
        .and_then(|v| v.get(&Value::String("schema".into())));

    let example = schema
        .and_then(|s| s.get(&Value::String("example".into())))
        .and_then(yaml_to_json);

    let item_ref = schema
        .and_then(|s| s.get(&Value::String("items".into())))
        .and_then(|items| items.as_mapping())
        .and_then(|items| {
            items
                .get(&Value::String("oneOf".into()))
                .and_then(|v| v.as_sequence())
                .and_then(|seq| seq.first())
                .or_else(|| items.get(&Value::String("$ref".into())))
        })
        .and_then(extract_ref_from_value);

    (item_ref, example)
}

fn extract_response_schema(op: &serde_yaml::Mapping) -> Option<String> {
    let schema = op
        .get(&Value::String("responses".into()))
        .and_then(|v| v.get(&Value::String("200".into())))
        .and_then(|v| v.get(&Value::String("content".into())))
        .and_then(|v| v.get(&Value::String("application/json".into())))
        .and_then(|v| v.get(&Value::String("schema".into())));

    schema
        .and_then(|s| s.as_mapping())
        .and_then(|s| {
            s.get(&Value::String("oneOf".into()))
                .and_then(|v| v.as_sequence())
                .and_then(|seq| seq.first())
                .or_else(|| s.get(&Value::String("$ref".into())))
        })
        .and_then(extract_ref_from_value)
}

fn extract_ref_from_value(v: &Value) -> Option<String> {
    if let Some(s) = v.as_str() {
        return Some(ref_name(s));
    }
    if let Some(map) = v.as_mapping() {
        if let Some(r) = map
            .get(&Value::String("$ref".into()))
            .and_then(|v| v.as_str())
        {
            return Some(ref_name(r));
        }
    }
    None
}

fn ref_name(r: &str) -> String {
    r.trim_start_matches("#/components/schemas/").to_string()
}

fn yaml_to_json(v: &Value) -> Option<JsonValue> {
    match v {
        Value::Null => Some(JsonValue::Null),
        Value::Bool(b) => Some(JsonValue::Bool(*b)),
        Value::Number(n) => n
            .as_i64()
            .map(|i| JsonValue::Number(i.into()))
            .or_else(|| {
                n.as_f64()
                    .and_then(|f| serde_json::Number::from_f64(f).map(JsonValue::Number))
            })
            .or_else(|| n.as_u64().map(|u| JsonValue::Number(u.into()))),
        Value::String(s) => Some(JsonValue::String(s.clone())),
        Value::Sequence(seq) => {
            let mut out = Vec::new();
            for item in seq {
                out.push(yaml_to_json(item)?);
            }
            Some(JsonValue::Array(out))
        }
        Value::Mapping(map) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in map {
                let key = k.as_str()?.to_string();
                obj.insert(key, yaml_to_json(v)?);
            }
            Some(JsonValue::Object(obj))
        }
        _ => None,
    }
}

fn render_models(schemas: &BTreeMap<String, Value>) -> Result<String> {
    let mut out = String::new();
    out.push_str("// AUTO-GENERATED. DO NOT EDIT.\n");
    out.push_str("#![allow(dead_code)]\n");
    out.push_str("use serde::{Serialize, Deserialize};\n");
    out.push_str("use std::collections::HashMap;\n\n");

    for (name, schema) in schemas {
        render_schema(name, schema, &mut out, schemas)?;
    }

    Ok(out)
}

fn render_schema(
    name: &str,
    schema: &Value,
    out: &mut String,
    schemas: &BTreeMap<String, Value>,
) -> Result<()> {
    let map = schema.as_mapping().context("schema map")?;

    if let Some(all_of) = map
        .get(&Value::String("allOf".into()))
        .and_then(|v| v.as_sequence())
    {
        return render_all_of(name, all_of, out, schemas);
    }

    let typ = map
        .get(&Value::String("type".into()))
        .and_then(|v| v.as_str());
    let props = map
        .get(&Value::String("properties".into()))
        .and_then(|v| v.as_mapping());

    if typ == Some("object") || props.is_some() {
        let additional = map.get(&Value::String("additionalProperties".into()));
        if let Some(props) = props {
            render_object_struct(name, props, out, schemas)?;
        } else if let Some(add) = additional {
            let val_ty = map_type(add, schemas);
            out.push_str(&format!("pub type {name} = HashMap<String, {val_ty}>;\n\n"));
        } else {
            out.push_str(&format!("pub type {name} = serde_json::Value;\n\n"));
        }
        return Ok(());
    }

    if let Some(t) = typ {
        let rust_ty = map_primitive(t);
        out.push_str(&format!("pub type {name} = {rust_ty};\n\n"));
        return Ok(());
    }

    out.push_str(&format!("pub type {name} = serde_json::Value;\n\n"));
    Ok(())
}

fn render_all_of(
    name: &str,
    all_of: &[Value],
    out: &mut String,
    _schemas: &BTreeMap<String, Value>,
) -> Result<()> {
    out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    out.push_str(&format!("pub struct {name} {{\n"));

    let mut has_inline = false;
    for item in all_of {
        if let Some(r) = item
            .get(&Value::String("$ref".into()))
            .and_then(|v| v.as_str())
        {
            let ref_name = ref_name(r);
            out.push_str(&format!(
                "    #[serde(flatten)]\n    pub {0}: {1},\n",
                snake_ident(&ref_name),
                ref_name
            ));
        } else if item.get(&Value::String("properties".into())).is_some()
            || item
                .get(&Value::String("additionalProperties".into()))
                .is_some()
        {
            has_inline = true;
        }
    }

    if has_inline {
        out.push_str("    #[serde(flatten)]\n    pub extra: serde_json::Value,\n");
    }

    out.push_str("}\n\n");
    Ok(())
}

fn render_object_struct(
    name: &str,
    props: &serde_yaml::Mapping,
    out: &mut String,
    schemas: &BTreeMap<String, Value>,
) -> Result<()> {
    out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    out.push_str(&format!("pub struct {name} {{\n"));

    for (k, v) in props {
        let key = k.as_str().context("prop name")?;
        let field_name = safe_ident(key);
        let ty = map_type(v, schemas);
        let ty = format!("Option<{ty}>");
        if field_name != key {
            out.push_str(&format!("    #[serde(rename = \"{key}\")]\n"));
        }
        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
        out.push_str(&format!("    pub {field_name}: {ty},\n"));
    }

    out.push_str("}\n\n");
    Ok(())
}

fn map_type(schema: &Value, schemas: &BTreeMap<String, Value>) -> String {
    let map = match schema.as_mapping() {
        Some(m) => m,
        None => return "serde_json::Value".to_string(),
    };

    if let Some(r) = map
        .get(&Value::String("$ref".into()))
        .and_then(|v| v.as_str())
    {
        return ref_name(r);
    }

    if let Some(t) = map
        .get(&Value::String("type".into()))
        .and_then(|v| v.as_str())
    {
        match t {
            "string" => "String".to_string(),
            "integer" => "i64".to_string(),
            "number" => "f64".to_string(),
            "boolean" => "bool".to_string(),
            "array" => {
                let items = map.get(&Value::String("items".into()));
                if let Some(it) = items {
                    let inner = if let Some(oneof) = it
                        .get(&Value::String("oneOf".into()))
                        .and_then(|v| v.as_sequence())
                    {
                        if let Some(first) = oneof.first() {
                            map_type(first, schemas)
                        } else {
                            "serde_json::Value".to_string()
                        }
                    } else {
                        map_type(it, schemas)
                    };
                    format!("Vec<{inner}>")
                } else {
                    "Vec<serde_json::Value>".to_string()
                }
            }
            "object" => {
                if let Some(add) = map.get(&Value::String("additionalProperties".into())) {
                    let val_ty = map_type(add, schemas);
                    format!("HashMap<String, {val_ty}>")
                } else {
                    "serde_json::Value".to_string()
                }
            }
            _ => "serde_json::Value".to_string(),
        }
    } else if map.get(&Value::String("properties".into())).is_some() {
        "serde_json::Value".to_string()
    } else {
        "serde_json::Value".to_string()
    }
}

fn map_primitive(t: &str) -> &'static str {
    match t {
        "string" => "String",
        "integer" => "i64",
        "number" => "f64",
        "boolean" => "bool",
        _ => "serde_json::Value",
    }
}

fn safe_ident(name: &str) -> String {
    let mut out = String::new();
    for (i, ch) in name.chars().enumerate() {
        if (i == 0 && !ch.is_ascii_alphabetic() && ch != '_')
            || (!ch.is_ascii_alphanumeric() && ch != '_')
        {
            out.push('_');
        } else {
            out.push(ch);
        }
    }
    if is_keyword(&out) {
        out.push('_');
    }
    out
}

fn snake_ident(name: &str) -> String {
    let mut out = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else if !ch.is_ascii_alphanumeric() && ch != '_' {
            out.push('_');
        } else {
            out.push(ch);
        }
    }
    if is_keyword(&out) {
        out.push('_');
    }
    out
}

fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
    )
}

fn render_meta(ops: &[Operation], schemas: &BTreeMap<String, Value>) -> Result<String> {
    let mut out = String::new();
    out.push_str("// AUTO-GENERATED. DO NOT EDIT.\n");
    out.push_str("#![allow(dead_code, non_upper_case_globals)]\n\n");

    out.push_str("#[derive(Clone, Copy, Debug)]\n");
    out.push_str("pub enum FieldKind {\n    String, Integer, Number, Boolean, Json, StringArray, IntegerArray, NumberArray, BooleanArray, JsonArray,\n}\n\n");
    out.push_str("impl FieldKind {\n    pub fn is_array(&self) -> bool { matches!(self, FieldKind::StringArray|FieldKind::IntegerArray|FieldKind::NumberArray|FieldKind::BooleanArray|FieldKind::JsonArray) }\n}\n\n");

    out.push_str("#[derive(Clone, Copy, Debug)]\n");
    out.push_str("pub struct FieldMeta {\n    pub name: &'static str,\n    pub cli_name: &'static str,\n    pub description: &'static str,\n    pub kind: FieldKind,\n}\n\n");

    out.push_str("#[derive(Clone, Copy, Debug)]\n");
    out.push_str("pub struct OperationMeta {\n    pub operation_id: &'static str,\n    pub method: &'static str,\n    pub path: &'static str,\n    pub description: &'static str,\n    pub request_schema: Option<&'static str>,\n    pub response_schema: Option<&'static str>,\n    pub path_params: &'static [&'static str],\n    pub fields: &'static [FieldMeta],\n    pub request_example: Option<&'static str>,\n    pub cli_path: &'static [&'static str],\n}\n\n");

    let multi = find_multi_method_paths(ops);
    out.push_str("pub static MULTI_METHOD_PATHS: &[&str] = &[");
    for path in &multi {
        out.push_str(&format!("\"{}\",", path));
    }
    out.push_str("];\n\n");

    let mut field_defs = String::new();
    let mut op_defs = String::new();

    for op in ops {
        let field_ident = format!("FIELDS_{}", op.ident);
        let fields = render_fields_for_operation(op, &field_ident, schemas)?;
        field_defs.push_str(&fields);

        let example_str = if let Some(ex) = &op.request_example {
            let s = serde_json::to_string_pretty(ex).unwrap_or_default();
            format!("Some({:?})", s)
        } else {
            "None".to_string()
        };

        let cli_path = cli_path_for(&op.path, &op.method, &multi);
        let cli_vec = format!(
            "&[{}]",
            cli_path
                .iter()
                .map(|s| format!("{:?}", s))
                .collect::<Vec<_>>()
                .join(", ")
        );

        op_defs.push_str(&format!(
            "pub static OP_{ident}: OperationMeta = OperationMeta {{\n    operation_id: \"{id}\",\n    method: \"{method}\",\n    path: \"{path}\",\n    description: {desc:?},\n    request_schema: {req},\n    response_schema: {resp},\n    path_params: &{params:?},\n    fields: {fields_name},\n    request_example: {example},\n    cli_path: {cli_vec},\n}};\n\n",
            ident = op.ident,
            id = op.operation_id,
            method = op.method,
            path = op.path,
            desc = op.description,
            req = op.request_schema.as_ref().map(|s| format!("Some(\"{}\")", s)).unwrap_or_else(|| "None".to_string()),
            resp = op.response_schema.as_ref().map(|s| format!("Some(\"{}\")", s)).unwrap_or_else(|| "None".to_string()),
            params = op.path_params,
            fields_name = field_ident,
            example = example_str,
            cli_vec = cli_vec,
        ));
    }

    out.push_str(&field_defs);
    out.push_str(&op_defs);

    out.push_str("pub static OPERATIONS: &[OperationMeta] = &[");
    for op in ops {
        out.push_str(&format!("OP_{},", op.ident));
    }
    out.push_str("];\n");

    Ok(out)
}

fn render_fields_for_operation(
    op: &Operation,
    ident: &str,
    schemas: &BTreeMap<String, Value>,
) -> Result<String> {
    let mut out = String::new();
    let mut fields = Vec::new();

    if let Some(schema_name) = &op.request_schema {
        let schema = schemas.get(schema_name).context("schema")?;
        if let Some(map) = schema.as_mapping() {
            if let Some(props) = map
                .get(&Value::String("properties".into()))
                .and_then(|v| v.as_mapping())
            {
                for (k, v) in props {
                    let name = k.as_str().context("prop name")?;
                    let desc = v
                        .get(&Value::String("description".into()))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let kind = field_kind(v);
                    let cli = name.replace('_', "-");
                    fields.push((name.to_string(), cli, desc.to_string(), kind));
                }
            }
        }
    }

    out.push_str(&format!("pub static {ident}: &[FieldMeta] = &[\n"));
    for (name, cli, desc, kind) in fields {
        out.push_str(&format!("    FieldMeta {{ name: \"{name}\", cli_name: \"{cli}\", description: {desc:?}, kind: {kind} }},\n"));
    }
    out.push_str("];\n\n");

    Ok(out)
}

fn field_kind(schema: &Value) -> String {
    let map = match schema.as_mapping() {
        Some(m) => m,
        None => return "FieldKind::Json".to_string(),
    };
    let typ = map
        .get(&Value::String("type".into()))
        .and_then(|v| v.as_str());

    match typ {
        Some("string") => "FieldKind::String".to_string(),
        Some("integer") => "FieldKind::Integer".to_string(),
        Some("number") => "FieldKind::Number".to_string(),
        Some("boolean") => "FieldKind::Boolean".to_string(),
        Some("object") => "FieldKind::Json".to_string(),
        Some("array") => {
            let items = map.get(&Value::String("items".into()));
            if let Some(it) = items.and_then(|v| v.as_mapping()) {
                let it_type = it
                    .get(&Value::String("type".into()))
                    .and_then(|v| v.as_str());
                match it_type {
                    Some("string") => "FieldKind::StringArray".to_string(),
                    Some("integer") => "FieldKind::IntegerArray".to_string(),
                    Some("number") => "FieldKind::NumberArray".to_string(),
                    Some("boolean") => "FieldKind::BooleanArray".to_string(),
                    _ => "FieldKind::JsonArray".to_string(),
                }
            } else {
                "FieldKind::JsonArray".to_string()
            }
        }
        _ => "FieldKind::Json".to_string(),
    }
}

fn find_multi_method_paths(ops: &[Operation]) -> BTreeSet<String> {
    let mut map: HashMap<String, BTreeSet<String>> = HashMap::new();
    for op in ops {
        map.entry(op.path.clone())
            .or_default()
            .insert(op.method.clone());
    }
    let mut multi = BTreeSet::new();
    for (path, methods) in map {
        if methods.len() > 1 {
            multi.insert(path);
        }
    }
    multi
}

fn cli_path_for(path: &str, method: &str, multi: &BTreeSet<String>) -> Vec<String> {
    let mut parts = path
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if parts.first().map(|s| s.starts_with('v')).unwrap_or(false) {
        parts.remove(0);
    }
    let mut segments = parts
        .into_iter()
        .filter(|p| !p.starts_with('{'))
        .map(|s| s.replace('_', "-"))
        .collect::<Vec<_>>();

    if multi.contains(path) {
        segments.push(method.to_string());
    }
    segments
}

fn assign_unique_idents(ops: &mut [Operation]) {
    let mut used = HashSet::new();
    for op in ops {
        let base = op.operation_id.clone();
        if used.insert(base.clone()) {
            op.ident = base;
            continue;
        }

        let suffix = ident_suffix_from_path(&op.path, &op.method);
        let mut candidate = format!("{base}{suffix}");
        let mut idx = 2;
        while used.contains(&candidate) {
            candidate = format!("{base}{suffix}{idx}");
            idx += 1;
        }
        op.ident = candidate.clone();
        used.insert(candidate);
    }
}

fn ident_suffix_from_path(path: &str, method: &str) -> String {
    let mut parts = path
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if parts.first().map(|s| s.starts_with('v')).unwrap_or(false) {
        parts.remove(0);
    }
    let mut out = String::new();
    for part in parts {
        if part.starts_with('{') {
            continue;
        }
        out.push_str(&pascal_case(part));
    }
    if out.is_empty() {
        out.push_str(&pascal_case(method));
    }
    out
}

fn pascal_case(s: &str) -> String {
    let mut out = String::new();
    let mut next_upper = true;
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            if next_upper {
                out.push(ch.to_ascii_uppercase());
                next_upper = false;
            } else {
                out.push(ch.to_ascii_lowercase());
            }
        } else {
            next_upper = true;
        }
    }
    if out.is_empty() {
        "Op".to_string()
    } else {
        out
    }
}

fn render_ops(ops: &[Operation]) -> Result<String> {
    let mut out = String::new();
    out.push_str("// AUTO-GENERATED. DO NOT EDIT.\n");
    out.push_str("#![allow(dead_code, unused_imports)]\n");
    out.push_str("use anyhow::Result;\n");
    out.push_str("use crate::client::DataForSeoClient;\n");
    out.push_str("use crate::generated::models::*;\n\n");

    for op in ops {
        let fn_name = snake_case(&op.ident);
        let req = op.request_schema.as_deref().unwrap_or("serde_json::Value");
        let resp = op.response_schema.as_deref().unwrap_or("serde_json::Value");
        let path = &op.path;
        let method = &op.method;

        if method == "get" {
            let params = render_path_params(&op.path_params);
            out.push_str(&format!(
                "pub async fn {fn_name}(client: &DataForSeoClient{params}) -> Result<{resp}> {{\n{path_build}    client.get::<{resp}>(\n        &path\n    ).await\n}}\n\n",
                fn_name = fn_name,
                params = params,
                resp = resp,
                path_build = render_path_build(path, &op.path_params),
            ));
        } else {
            out.push_str(&format!(
                "pub async fn {fn_name}(client: &DataForSeoClient, body: &Vec<{req}>) -> Result<{resp}> {{\n    client.post::<Vec<{req}>, {resp}>(\"{path}\", body).await\n}}\n\n",
                fn_name = fn_name,
                req = req,
                resp = resp,
                path = path,
            ));
        }
    }

    Ok(out)
}

fn render_docs(path: &PathBuf) -> Result<String> {
    let raw = fs::read_to_string(path).context("read docs_map.json")?;
    let map: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(&raw).context("parse docs_map.json")?;

    let mut out = String::new();
    out.push_str("// AUTO-GENERATED. DO NOT EDIT.\n");
    out.push_str("#![allow(dead_code)]\n\n");

    out.push_str("pub static DOCS_MAP: &[(&str, &str)] = &[\n");
    for (k, v) in map.iter() {
        let val = v.as_str().unwrap_or("");
        out.push_str(&format!("    ({:?}, {:?}),\n", k, val));
    }
    out.push_str("];\n\n");

    out.push_str("pub fn lookup(path: &[String]) -> Option<&'static str> {\n");
    out.push_str("    let mut end = path.len();\n");
    out.push_str("    while end > 0 {\n");
    out.push_str("        let key = path[..end].join(\"/\");\n");
    out.push_str("        if let Some((_, v)) = DOCS_MAP.iter().find(|(k, _)| *k == key) {\n");
    out.push_str("            return Some(*v);\n");
    out.push_str("        }\n");
    out.push_str("        end -= 1;\n");
    out.push_str("    }\n");
    out.push_str("    None\n");
    out.push_str("}\n");

    Ok(out)
}

fn render_path_params(params: &[String]) -> String {
    let mut s = String::new();
    for p in params {
        s.push_str(&format!(", {p}: &str"));
    }
    s
}

fn render_path_build(path: &str, params: &[String]) -> String {
    if params.is_empty() {
        return format!("    let path = \"{path}\".to_string();\n");
    }
    let args = params
        .iter()
        .map(|p| format!("{p} = {p}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("    let path = format!(\"{path}\", {args});\n")
}

fn snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}
