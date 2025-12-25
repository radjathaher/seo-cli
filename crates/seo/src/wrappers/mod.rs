pub mod keyword;
pub mod serp;
pub mod website;

use anyhow::Result;
use clap::{ArgMatches, Command};
use serde_json::Value;

use crate::client::DataForSeoClient;
use crate::output::OutputFormat;

pub fn commands() -> Vec<Command> {
    vec![website::command(), keyword::command(), serp::command()]
}

pub fn handle(root: &ArgMatches, client: &DataForSeoClient, format: OutputFormat) -> Result<bool> {
    match root.subcommand() {
        Some(("website", sub)) => {
            website::run(root, sub, client, format)?;
            Ok(true)
        }
        Some(("keyword", sub)) => {
            keyword::run(root, sub, client, format)?;
            Ok(true)
        }
        Some(("serp", sub)) => {
            serp::run(root, sub, client, format)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub fn dig<'a>(mut v: &'a Value, path: &[&str]) -> Option<&'a Value> {
    for k in path {
        v = v.get(*k)?;
    }
    Some(v)
}

pub fn items_from_tasks(resp: &Value) -> Vec<&Value> {
    let Some(tasks) = resp.get("tasks").and_then(|v| v.as_array()) else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for task in tasks {
        let Some(results) = task.get("result").and_then(|v| v.as_array()) else {
            continue;
        };
        for res in results {
            let Some(items) = res.get("items").and_then(|v| v.as_array()) else {
                continue;
            };
            for item in items {
                out.push(item);
            }
        }
    }
    out
}

pub fn normalize_domain(input: &str) -> String {
    let mut s = input.trim().to_string();
    if let Some(idx) = s.find("://") {
        s = s[(idx + 3)..].to_string();
    }
    if let Some(idx) = s.find('/') {
        s = s[..idx].to_string();
    }
    if let Some(stripped) = s.strip_prefix("www.") {
        s = stripped.to_string();
    }
    s
}
