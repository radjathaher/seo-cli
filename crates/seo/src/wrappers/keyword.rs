use anyhow::{Context, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};
use serde_json::{json, Map, Value};

use crate::client::DataForSeoClient;
use crate::output::{print_output, print_rows_tsv, OutputFormat};
use crate::wrappers::{dig, items_from_tasks};

const DEFAULT_LOC: &str = "2840";
const DEFAULT_LANG: &str = "en";

const IDEAS_COLS: &[&str] = &[
    "keyword",
    "search_volume",
    "keyword_difficulty",
    "competition_level",
    "competition",
    "cpc",
    "intent",
];

const EVAL_COLS: &[&str] = &[
    "keyword",
    "search_volume",
    "keyword_difficulty",
    "competition_level",
    "competition",
    "cpc",
    "intent",
    "intent_probability",
];

pub fn command() -> Command {
    Command::new("keyword")
        .about("Keyword workflows (opinionated; uses DataForSEO)")
        .subcommand_required(true)
        .subcommand(ideas_cmd())
        .subcommand(eval_cmd())
}

pub fn run(
    root: &ArgMatches,
    matches: &ArgMatches,
    client: &DataForSeoClient,
    format: OutputFormat,
) -> Result<()> {
    match matches.subcommand() {
        Some(("ideas", sub)) => run_ideas(root, sub, client, format),
        Some(("eval", sub)) => run_eval(root, sub, client, format),
        _ => anyhow::bail!("unknown keyword subcommand"),
    }
}

fn ideas_cmd() -> Command {
    Command::new("ideas")
        .about("Keyword ideas (DataForSEO Labs keyword_suggestions)")
        .arg(
            Arg::new("seed")
                .long("seed")
                .value_name("TEXT")
                .required(true)
                .help("Seed keyword / phrase")
                .action(ArgAction::Set),
        )
        .arg(loc_arg())
        .arg(lang_arg())
        .arg(
            Arg::new("limit")
                .long("limit")
                .value_name("N")
                .default_value("200")
                .help("Max rows returned (API limit; default 200)")
                .action(ArgAction::Set),
        )
}

fn eval_cmd() -> Command {
    Command::new("eval")
        .about("Evaluate a keyword (volume, difficulty, CPC, intent)")
        .arg(
            Arg::new("kw")
                .long("kw")
                .value_name("KEYWORD")
                .required(true)
                .help("Keyword to evaluate")
                .action(ArgAction::Set),
        )
        .arg(loc_arg())
        .arg(lang_arg())
}

fn loc_arg() -> Arg {
    Arg::new("loc")
        .long("loc")
        .value_name("LOCATION_CODE")
        .default_value(DEFAULT_LOC)
        .help("Location code (default 2840 = US)")
        .action(ArgAction::Set)
}

fn lang_arg() -> Arg {
    Arg::new("lang")
        .long("lang")
        .value_name("LANGUAGE_CODE")
        .default_value(DEFAULT_LANG)
        .help("Language code (default en)")
        .action(ArgAction::Set)
}

fn dry_run(root: &ArgMatches) -> bool {
    root.get_flag("dry-run") || root.get_flag("curl")
}

fn run_ideas(
    root: &ArgMatches,
    m: &ArgMatches,
    client: &DataForSeoClient,
    format: OutputFormat,
) -> Result<()> {
    let seed = m
        .get_one::<String>("seed")
        .context("missing --seed")?
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
    let limit: i64 = m
        .get_one::<String>("limit")
        .map(|s| s.as_str())
        .unwrap_or("200")
        .parse()
        .context("invalid --limit")?;

    let path = "/v3/dataforseo_labs/google/keyword_suggestions/live";
    let body = json!([{
        "keyword": seed,
        "location_code": loc,
        "language_code": lang,
        "limit": limit
    }]);

    if dry_run(root) {
        let curl = client.build_curl(path, "post", Some(&body))?;
        println!("{curl}");
        return Ok(());
    }

    let raw = client.post_json(path, &body)?;
    let rows = idea_rows(&raw);
    print_wrapper_output(
        format,
        json!({"seed": seed, "loc": loc, "lang": lang, "limit": limit}),
        raw,
        rows,
        IDEAS_COLS,
    )
}

fn idea_rows(raw: &Value) -> Vec<Map<String, Value>> {
    let mut rows = Vec::new();
    for item in items_from_tasks(raw) {
        let mut row = Map::new();
        row.insert(
            "keyword".to_string(),
            item.get("keyword").cloned().unwrap_or(Value::Null),
        );
        row.insert(
            "search_volume".to_string(),
            dig(item, &["keyword_info", "search_volume"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "keyword_difficulty".to_string(),
            dig(item, &["keyword_properties", "keyword_difficulty"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "competition_level".to_string(),
            dig(item, &["keyword_info", "competition_level"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "competition".to_string(),
            dig(item, &["keyword_info", "competition"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "cpc".to_string(),
            dig(item, &["keyword_info", "cpc"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "intent".to_string(),
            dig(item, &["search_intent_info", "main_intent"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        rows.push(row);
    }
    rows
}

fn run_eval(
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

    let path_overview = "/v3/dataforseo_labs/google/keyword_overview/live";
    let body_overview = json!([{
        "keywords": [kw.clone()],
        "location_code": loc,
        "language_code": lang.clone()
    }]);

    let path_intent = "/v3/dataforseo_labs/google/search_intent/live";
    let body_intent = json!([{
        "keywords": [kw.clone()],
        "language_code": lang.clone()
    }]);

    if dry_run(root) {
        println!(
            "{}",
            client.build_curl(path_overview, "post", Some(&body_overview))?
        );
        println!(
            "{}",
            client.build_curl(path_intent, "post", Some(&body_intent))?
        );
        return Ok(());
    }

    let raw_overview = client.post_json(path_overview, &body_overview)?;
    let raw_intent = client.post_json(path_intent, &body_intent)?;

    let rows = eval_rows(&raw_overview, &raw_intent);
    match format {
        OutputFormat::Json => {
            let row_values = Value::Array(rows.into_iter().map(Value::Object).collect());
            print_output(
                &json!({
                    "inputs": { "kw": kw, "loc": loc, "lang": lang },
                    "rows": row_values,
                    "raw": { "keyword_overview": raw_overview, "search_intent": raw_intent }
                }),
                OutputFormat::Json,
            )
        }
        OutputFormat::Tsv => print_rows_tsv(&rows, Some(EVAL_COLS)),
    }
}

fn eval_rows(raw_overview: &Value, raw_intent: &Value) -> Vec<Map<String, Value>> {
    let mut intent_probability: Option<Value> = None;
    for item in items_from_tasks(raw_intent) {
        if dig(item, &["keyword"]).and_then(|v| v.as_str()).is_some() {
            intent_probability = dig(item, &["keyword_intent", "probability"]).cloned();
            break;
        }
    }

    let mut rows = Vec::new();
    for item in items_from_tasks(raw_overview) {
        let mut row = Map::new();
        row.insert(
            "keyword".to_string(),
            item.get("keyword").cloned().unwrap_or(Value::Null),
        );
        row.insert(
            "search_volume".to_string(),
            dig(item, &["keyword_info", "search_volume"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "keyword_difficulty".to_string(),
            dig(item, &["keyword_properties", "keyword_difficulty"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "competition_level".to_string(),
            dig(item, &["keyword_info", "competition_level"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "competition".to_string(),
            dig(item, &["keyword_info", "competition"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "cpc".to_string(),
            dig(item, &["keyword_info", "cpc"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "intent".to_string(),
            dig(item, &["search_intent_info", "main_intent"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "intent_probability".to_string(),
            intent_probability.clone().unwrap_or(Value::Null),
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
