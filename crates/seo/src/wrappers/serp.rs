use anyhow::{Context, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};
use serde_json::{json, Map, Value};

use crate::client::DataForSeoClient;
use crate::output::{print_output, print_rows_tsv, OutputFormat};
use crate::wrappers::{dig, items_from_tasks};

const DEFAULT_LOC: &str = "2840";
const DEFAULT_LANG: &str = "en";

const TOP_COLS: &[&str] = &[
    "rank",
    "title",
    "domain",
    "url",
    "description",
    "breadcrumb",
];

pub fn command() -> Command {
    Command::new("serp")
        .about("SERP checks (opinionated; uses DataForSEO)")
        .subcommand_required(true)
        .subcommand(top_cmd())
}

pub fn run(
    root: &ArgMatches,
    matches: &ArgMatches,
    client: &DataForSeoClient,
    format: OutputFormat,
) -> Result<()> {
    match matches.subcommand() {
        Some(("top", sub)) => run_top(root, sub, client, format),
        _ => anyhow::bail!("unknown serp subcommand"),
    }
}

fn top_cmd() -> Command {
    Command::new("top")
        .about("Top organic results for a keyword (SERP Google organic live advanced)")
        .arg(
            Arg::new("kw")
                .long("kw")
                .value_name("KEYWORD")
                .required(true)
                .help("Keyword to query in Google SERP")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("loc")
                .long("loc")
                .value_name("LOCATION_CODE")
                .default_value(DEFAULT_LOC)
                .help("Location code (default 2840 = US)")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("lang")
                .long("lang")
                .value_name("LANGUAGE_CODE")
                .default_value(DEFAULT_LANG)
                .help("Language code (default en)")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("depth")
                .long("depth")
                .value_name("N")
                .default_value("10")
                .help("Number of results to fetch (default 10)")
                .action(ArgAction::Set),
        )
}

fn dry_run(root: &ArgMatches) -> bool {
    root.get_flag("dry-run") || root.get_flag("curl")
}

fn run_top(
    root: &ArgMatches,
    m: &ArgMatches,
    client: &DataForSeoClient,
    format: OutputFormat,
) -> Result<()> {
    let kw = m
        .get_one::<String>("kw")
        .context("missing --kw")?
        .to_string();
    let loc: i64 = m
        .get_one::<String>("loc")
        .map(|s| s.as_str())
        .unwrap_or(DEFAULT_LOC)
        .parse()
        .context("invalid --loc")?;
    let lang = m
        .get_one::<String>("lang")
        .map(|s| s.as_str())
        .unwrap_or(DEFAULT_LANG)
        .to_string();
    let depth: i64 = m
        .get_one::<String>("depth")
        .map(|s| s.as_str())
        .unwrap_or("10")
        .parse()
        .context("invalid --depth")?;

    let path = "/v3/serp/google/organic/live/advanced";
    let body = json!([{
        "keyword": kw,
        "location_code": loc,
        "language_code": lang,
        "depth": depth
    }]);

    if dry_run(root) {
        let curl = client.build_curl(path, "post", Some(&body))?;
        println!("{curl}");
        return Ok(());
    }

    let raw = client.post_json(path, &body)?;
    let rows = serp_rows(&raw);
    print_wrapper_output(
        format,
        json!({"kw": kw, "loc": loc, "lang": lang, "depth": depth}),
        raw,
        rows,
        TOP_COLS,
    )
}

fn serp_rows(raw: &Value) -> Vec<Map<String, Value>> {
    let mut rows = Vec::new();
    for item in items_from_tasks(raw) {
        if dig(item, &["type"]).and_then(|v| v.as_str()) != Some("organic") {
            continue;
        }
        let mut row = Map::new();
        row.insert(
            "rank".to_string(),
            item.get("rank_absolute").cloned().unwrap_or(Value::Null),
        );
        row.insert(
            "title".to_string(),
            item.get("title").cloned().unwrap_or(Value::Null),
        );
        row.insert(
            "domain".to_string(),
            item.get("domain").cloned().unwrap_or(Value::Null),
        );
        row.insert(
            "url".to_string(),
            item.get("url").cloned().unwrap_or(Value::Null),
        );
        row.insert(
            "description".to_string(),
            item.get("description").cloned().unwrap_or(Value::Null),
        );
        row.insert(
            "breadcrumb".to_string(),
            item.get("breadcrumb").cloned().unwrap_or(Value::Null),
        );
        rows.push(row);
    }
    rows
}

fn print_wrapper_output(
    format: OutputFormat,
    inputs: Value,
    raw: Value,
    rows: Vec<Map<String, Value>>,
    cols: &[&str],
) -> Result<()> {
    match format {
        OutputFormat::Json => {
            let row_values = Value::Array(rows.into_iter().map(Value::Object).collect());
            print_output(
                &json!({ "inputs": inputs, "rows": row_values, "raw": raw }),
                OutputFormat::Json,
            )
        }
        OutputFormat::Tsv => print_rows_tsv(&rows, Some(cols)),
    }
}
