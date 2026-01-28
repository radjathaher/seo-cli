use anyhow::{Context, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};
use serde_json::{Map, Value};
use std::fs;
use std::time::Duration;

use crate::generated::docs;
use crate::generated::meta::{FieldKind, OperationMeta, OPERATIONS};
use crate::output::OutputFormat;
use crate::wrappers;

pub struct ResolvedOperation {
    pub method: String,
    pub path: String,
    pub body: Option<Value>,
    pub dry_run: bool,
}

pub fn build_cli() -> Command {
    let mut root = Command::new("seo")
        .about("DataForSEO CLI (OpenAPI-driven)")
        .arg(
            Arg::new("base-url")
                .long("base-url")
                .value_name("URL")
                .help("Override API base URL (default: https://api.dataforseo.com)")
                .global(true),
        )
        .arg(
            Arg::new("sandbox")
                .long("sandbox")
                .action(ArgAction::SetTrue)
                .help("Use DataForSEO sandbox base URL (https://sandbox.dataforseo.com)")
                .global(true),
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .value_name("SECONDS")
                .help("HTTP timeout in seconds")
                .global(true),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Resolve request but do not call API")
                .global(true),
        )
        .arg(
            Arg::new("curl")
                .long("curl")
                .action(ArgAction::SetTrue)
                .help("Print curl command (uses $DATAFORSEO_LOGIN/$DATAFORSEO_PASSWORD)")
                .global(true),
        )
        .arg(
            Arg::new("format")
                .long("format")
                .value_name("FORMAT")
                .value_parser(["tsv", "json"])
                .default_value("tsv")
                .help("Output format")
                .global(true),
        )
        ;

    for cmd in wrappers::commands() {
        root = root.subcommand(cmd);
    }

    let mut dfs = Command::new("dfs")
        .about("Raw DataForSEO API (OpenAPI-driven)")
        .after_help("Mirrors DataForSEO v3 endpoints. Use `seo dfs ...` to access the full OpenAPI command tree.");

    let mut tree = build_tree();
    apply_docs(&mut tree, &mut Vec::new());
    for child in tree.subcommands {
        dfs = dfs.subcommand(child.to_clap());
    }
    root = root.subcommand(dfs);

    root
}

pub fn resolve_operation(matches: &ArgMatches) -> Result<ResolvedOperation> {
    let mut path = Vec::new();
    let mut current = matches;

    loop {
        match current.subcommand() {
            Some(("dfs", sub)) => {
                current = sub;
                continue;
            }
            Some((name, sub)) => {
                path.push(name.to_string());
                current = sub;
            }
            _ => break,
        }
    }

    let op = find_operation(&path).context("unknown command path")?;

    let mut full_path = op.path.to_string();
    for param in op.path_params {
        let val = current
            .get_one::<String>(param)
            .context(format!("missing --{param}"))?;
        full_path = full_path.replace(&format!("{{{param}}}"), val);
    }

    let body = if op.method == "post" {
        if let Some(file) = current.get_one::<String>("json-file") {
            let raw = fs::read_to_string(file).context("read json file")?;
            let parsed: Value = serde_json::from_str(&raw).context("parse json file")?;
            Some(parsed)
        } else {
            Some(build_body_from_flags(op, current)?)
        }
    } else {
        None
    };

    let dry_run = matches.get_flag("dry-run") || matches.get_flag("curl");

    Ok(ResolvedOperation {
        method: op.method.to_string(),
        path: full_path,
        body,
        dry_run,
    })
}

pub fn resolve_base_url(matches: &ArgMatches) -> Result<String> {
    if matches.get_flag("sandbox") {
        return Ok("https://sandbox.dataforseo.com".to_string());
    }
    if let Some(url) = matches.get_one::<String>("base-url") {
        return Ok(url.to_string());
    }
    Ok("https://api.dataforseo.com".to_string())
}

pub fn resolve_timeout(matches: &ArgMatches) -> Result<Duration> {
    if let Some(raw) = matches.get_one::<String>("timeout") {
        let secs: u64 = raw.parse().context("parse timeout")?;
        return Ok(Duration::from_secs(secs));
    }
    Ok(Duration::from_secs(120))
}

pub fn resolve_format(matches: &ArgMatches) -> Result<OutputFormat> {
    match matches.get_one::<String>("format").map(|s| s.as_str()) {
        Some("json") => Ok(OutputFormat::Json),
        _ => Ok(OutputFormat::Tsv),
    }
}

fn build_body_from_flags(op: &OperationMeta, matches: &ArgMatches) -> Result<Value> {
    let mut obj = Map::new();

    for field in op.fields {
        let key = field.name;
        let cli = field.cli_name;

        match field.kind {
            FieldKind::String => {
                if let Some(v) = matches.get_one::<String>(cli) {
                    obj.insert(key.to_string(), Value::String(v.to_string()));
                }
            }
            FieldKind::Integer => {
                if let Some(v) = matches.get_one::<String>(cli) {
                    let n: i64 = v.parse().context(format!("invalid integer for {cli}"))?;
                    obj.insert(key.to_string(), Value::Number(n.into()));
                }
            }
            FieldKind::Number => {
                if let Some(v) = matches.get_one::<String>(cli) {
                    let n: f64 = v.parse().context(format!("invalid number for {cli}"))?;
                    let num = serde_json::Number::from_f64(n).context("invalid float")?;
                    obj.insert(key.to_string(), Value::Number(num));
                }
            }
            FieldKind::Boolean => {
                if let Some(v) = matches.get_one::<String>(cli) {
                    let b: bool = v.parse().context(format!("invalid bool for {cli}"))?;
                    obj.insert(key.to_string(), Value::Bool(b));
                }
            }
            FieldKind::StringArray => {
                if let Some(vals) = matches.get_many::<String>(cli) {
                    let arr = vals.map(|v| Value::String(v.to_string())).collect();
                    obj.insert(key.to_string(), Value::Array(arr));
                }
            }
            FieldKind::IntegerArray => {
                if let Some(vals) = matches.get_many::<String>(cli) {
                    let mut arr = Vec::new();
                    for v in vals {
                        let n: i64 = v.parse().context(format!("invalid integer for {cli}"))?;
                        arr.push(Value::Number(n.into()));
                    }
                    obj.insert(key.to_string(), Value::Array(arr));
                }
            }
            FieldKind::NumberArray => {
                if let Some(vals) = matches.get_many::<String>(cli) {
                    let mut arr = Vec::new();
                    for v in vals {
                        let n: f64 = v.parse().context(format!("invalid number for {cli}"))?;
                        let num = serde_json::Number::from_f64(n).context("invalid float")?;
                        arr.push(Value::Number(num));
                    }
                    obj.insert(key.to_string(), Value::Array(arr));
                }
            }
            FieldKind::BooleanArray => {
                if let Some(vals) = matches.get_many::<String>(cli) {
                    let mut arr = Vec::new();
                    for v in vals {
                        let b: bool = v.parse().context(format!("invalid bool for {cli}"))?;
                        arr.push(Value::Bool(b));
                    }
                    obj.insert(key.to_string(), Value::Array(arr));
                }
            }
            FieldKind::Json => {
                if let Some(v) = matches.get_one::<String>(cli) {
                    let parsed: Value =
                        serde_json::from_str(v).context(format!("invalid JSON for {cli}"))?;
                    obj.insert(key.to_string(), parsed);
                }
            }
            FieldKind::JsonArray => {
                if let Some(vals) = matches.get_many::<String>(cli) {
                    let mut arr = Vec::new();
                    for v in vals {
                        let parsed: Value =
                            serde_json::from_str(v).context(format!("invalid JSON for {cli}"))?;
                        arr.push(parsed);
                    }
                    obj.insert(key.to_string(), Value::Array(arr));
                }
            }
        }
    }

    Ok(Value::Array(vec![Value::Object(obj)]))
}

fn find_operation(path: &[String]) -> Option<&'static OperationMeta> {
    for op in OPERATIONS {
        if op.cli_path.len() != path.len() {
            continue;
        }
        if op.cli_path.iter().zip(path.iter()).all(|(a, b)| a == b) {
            return Some(op);
        }
    }
    None
}

#[derive(Clone, Debug)]
struct Node {
    name: String,
    about: Option<String>,
    subcommands: Vec<Node>,
    op: Option<&'static OperationMeta>,
}

impl Node {
    fn to_clap(&self) -> Command {
        let mut cmd = Command::new(&self.name);
        if let Some(a) = &self.about {
            cmd = cmd.about(a);
        }

        if let Some(op) = self.op {
            cmd = add_op_args(cmd, op);
        }

        for child in &self.subcommands {
            cmd = cmd.subcommand(child.to_clap());
        }
        cmd
    }
}

fn build_tree() -> Node {
    let mut root = Node {
        name: "root".to_string(),
        about: None,
        subcommands: Vec::new(),
        op: None,
    };

    for op in OPERATIONS {
        let mut segments = op
            .cli_path
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        insert_op(&mut root, &mut segments, op);
    }

    root
}

fn apply_docs(node: &mut Node, path: &mut Vec<String>) {
    let pushed = node.name != "root";
    if pushed {
        path.push(node.name.clone());
    }

    if node.op.is_none() && node.about.is_none() {
        node.about = Some(
            docs::lookup(path)
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("DataForSEO command group: {}", path.join(" "))),
        );
    }

    for child in &mut node.subcommands {
        apply_docs(child, path);
    }

    if pushed {
        path.pop();
    } else {
        path.clear();
    }
}

fn insert_op(root: &mut Node, segments: &mut Vec<String>, op: &'static OperationMeta) {
    if segments.is_empty() {
        root.op = Some(op);
        return;
    }

    let name = segments.remove(0);
    let idx = match root.subcommands.iter().position(|n| n.name == name) {
        Some(i) => i,
        None => {
            root.subcommands.push(Node {
                name: name.clone(),
                about: None,
                subcommands: Vec::new(),
                op: None,
            });
            root.subcommands.len() - 1
        }
    };
    let child = &mut root.subcommands[idx];

    insert_op(child, segments, op);
}

fn add_op_args(mut cmd: Command, op: &OperationMeta) -> Command {
    let about = format!(
        "{}\n\n{} {}\noperationId: {}",
        op.description,
        op.method.to_uppercase(),
        op.path,
        op.operation_id
    );
    cmd = cmd.about(about);

    if op.method == "post" {
        cmd = cmd.arg(
            Arg::new("json-file")
                .long("json-file")
                .value_name("PATH")
                .help("Use JSON array file instead of flags"),
        );
    }

    for param in op.path_params {
        cmd = cmd.arg(
            Arg::new(param)
                .long(param)
                .value_name(param.to_uppercase())
                .required(true),
        );
    }

    for field in op.fields {
        let mut arg = Arg::new(field.cli_name)
            .long(field.cli_name)
            .value_name(field.cli_name.to_uppercase())
            .help(field.description)
            .action(ArgAction::Set);

        if field.kind.is_array() {
            arg = arg.action(ArgAction::Append);
        }

        cmd = cmd.arg(arg);
    }

    if let Some(ex) = op.request_example {
        cmd = cmd.after_help(format!("Example body:\n{ex}"));
    }

    cmd
}
