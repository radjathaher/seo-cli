use anyhow::{Context, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};
use serde_json::{json, Map, Value};

use crate::client::DataForSeoClient;
use crate::output::{print_output, print_rows_tsv, OutputFormat};
use crate::wrappers::{dig, items_from_tasks, normalize_domain};

const DEFAULT_LOC: &str = "2840";
const DEFAULT_LANG: &str = "en";

const TRAFFIC_COLS: &[&str] = &[
    "target",
    "organic_count",
    "organic_etv",
    "paid_count",
    "paid_etv",
    "featured_snippet_count",
    "featured_snippet_etv",
    "local_pack_count",
    "local_pack_etv",
];

const KEYWORDS_COLS: &[&str] = &[
    "keyword",
    "search_volume",
    "keyword_difficulty",
    "competition_level",
    "cpc",
    "intent",
    "rank",
    "etv",
    "url",
    "title",
];

pub fn command() -> Command {
    Command::new("website")
        .about("Website workflows (opinionated; uses DataForSEO)")
        .subcommand_required(true)
        .subcommand(traffic_cmd())
        .subcommand(keywords_cmd())
        .subcommand(wins_cmd())
}

pub fn run(
    root: &ArgMatches,
    matches: &ArgMatches,
    client: &DataForSeoClient,
    format: OutputFormat,
) -> Result<()> {
    match matches.subcommand() {
        Some(("traffic", sub)) => run_traffic(root, sub, client, format),
        Some(("keywords", sub)) => run_keywords(root, sub, client, format),
        Some(("wins", sub)) => run_wins(root, sub, client, format),
        _ => anyhow::bail!("unknown website subcommand"),
    }
}

fn traffic_cmd() -> Command {
    Command::new("traffic")
        .about("Estimate total monthly traffic (DataForSEO Labs bulk_traffic_estimation)")
        .arg(site_arg())
        .arg(loc_arg())
        .arg(lang_arg())
}

fn keywords_cmd() -> Command {
    Command::new("keywords")
        .about("Top ranking keywords for a domain (DataForSEO Labs ranked_keywords)")
        .arg(site_arg())
        .arg(loc_arg())
        .arg(lang_arg())
        .arg(
            Arg::new("limit")
                .long("limit")
                .value_name("N")
                .default_value("100")
                .help("Max rows returned (API limit; default 100)"),
        )
}

fn wins_cmd() -> Command {
    Command::new("wins")
        .about("Quick-win keywords (rank range + volume filter) from ranked_keywords")
        .arg(site_arg())
        .arg(loc_arg())
        .arg(lang_arg())
        .arg(
            Arg::new("limit")
                .long("limit")
                .value_name("N")
                .default_value("500")
                .help("Fetch this many ranked keywords before filtering (default 500)"),
        )
        .arg(
            Arg::new("rank")
                .long("rank")
                .value_name("A..B")
                .default_value("4..20")
                .help("Rank range inclusive (e.g. 4..20)"),
        )
        .arg(
            Arg::new("min-volume")
                .long("min-volume")
                .value_name("N")
                .default_value("100")
                .help("Minimum search volume (default 100)"),
        )
        .arg(
            Arg::new("sort")
                .long("sort")
                .value_name("KEY")
                .default_value("search_volume")
                .value_parser(["search_volume", "etv"])
                .help("Sort wins by this field (default search_volume)"),
        )
}

fn site_arg() -> Arg {
    Arg::new("site")
        .long("site")
        .value_name("DOMAIN")
        .required(true)
        .help("Target domain (accepts domain or full URL)")
        .action(ArgAction::Set)
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

fn run_traffic(
    root: &ArgMatches,
    m: &ArgMatches,
    client: &DataForSeoClient,
    format: OutputFormat,
) -> Result<()> {
    let site_raw = m.get_one::<String>("site").context("missing --site")?;
    let site = normalize_domain(site_raw);
    let loc: i64 = m
        .get_one::<String>("loc")
        .unwrap_or(&DEFAULT_LOC.to_string())
        .parse()
        .context("invalid --loc")?;
    let lang = m
        .get_one::<String>("lang")
        .cloned()
        .unwrap_or_else(|| DEFAULT_LANG.to_string());

    let path = "/v3/dataforseo_labs/google/bulk_traffic_estimation/live";
    let body = json!([{
        "targets": [site],
        "location_code": loc,
        "language_code": lang,
        "item_types": ["organic", "paid", "featured_snippet", "local_pack"]
    }]);

    if dry_run(root) {
        let curl = client.build_curl(path, "post", Some(&body))?;
        println!("{curl}");
        return Ok(());
    }

    let raw = client.post_json(path, &body)?;
    let rows = traffic_rows(&raw);
    print_wrapper_output(
        format,
        json!({"site": site_raw, "loc": loc, "lang": lang}),
        raw,
        rows,
        TRAFFIC_COLS,
    )
}

fn traffic_rows(raw: &Value) -> Vec<Map<String, Value>> {
    let mut rows = Vec::new();
    for item in items_from_tasks(raw) {
        let mut row = Map::new();
        row.insert(
            "target".to_string(),
            item.get("target").cloned().unwrap_or(Value::Null),
        );

        row.insert(
            "organic_count".to_string(),
            dig(item, &["metrics", "organic", "count"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "organic_etv".to_string(),
            dig(item, &["metrics", "organic", "etv"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "paid_count".to_string(),
            dig(item, &["metrics", "paid", "count"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "paid_etv".to_string(),
            dig(item, &["metrics", "paid", "etv"])
                .cloned()
                .unwrap_or(Value::Null),
        );

        row.insert(
            "featured_snippet_count".to_string(),
            dig(item, &["metrics", "featured_snippet", "count"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "featured_snippet_etv".to_string(),
            dig(item, &["metrics", "featured_snippet", "etv"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "local_pack_count".to_string(),
            dig(item, &["metrics", "local_pack", "count"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        row.insert(
            "local_pack_etv".to_string(),
            dig(item, &["metrics", "local_pack", "etv"])
                .cloned()
                .unwrap_or(Value::Null),
        );
        rows.push(row);
    }
    rows
}

fn run_keywords(
    root: &ArgMatches,
    m: &ArgMatches,
    client: &DataForSeoClient,
    format: OutputFormat,
) -> Result<()> {
    let site_raw = m.get_one::<String>("site").context("missing --site")?;
    let site = normalize_domain(site_raw);
    let loc: i64 = m
        .get_one::<String>("loc")
        .unwrap_or(&DEFAULT_LOC.to_string())
        .parse()
        .context("invalid --loc")?;
    let lang = m
        .get_one::<String>("lang")
        .cloned()
        .unwrap_or_else(|| DEFAULT_LANG.to_string());
    let limit: i64 = m
        .get_one::<String>("limit")
        .unwrap_or(&"100".to_string())
        .parse()
        .context("invalid --limit")?;

    let path = "/v3/dataforseo_labs/google/ranked_keywords/live";
    let body = json!([{
        "target": site,
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
    let rows = ranked_keyword_rows(&raw);
    print_wrapper_output(
        format,
        json!({"site": site_raw, "loc": loc, "lang": lang, "limit": limit}),
        raw,
        rows,
        KEYWORDS_COLS,
    )
}

fn run_wins(
    root: &ArgMatches,
    m: &ArgMatches,
    client: &DataForSeoClient,
    format: OutputFormat,
) -> Result<()> {
    let site_raw = m.get_one::<String>("site").context("missing --site")?;
    let site = normalize_domain(site_raw);
    let loc: i64 = m
        .get_one::<String>("loc")
        .unwrap_or(&DEFAULT_LOC.to_string())
        .parse()
        .context("invalid --loc")?;
    let lang = m
        .get_one::<String>("lang")
        .cloned()
        .unwrap_or_else(|| DEFAULT_LANG.to_string());
    let limit: i64 = m
        .get_one::<String>("limit")
        .unwrap_or(&"500".to_string())
        .parse()
        .context("invalid --limit")?;
    let min_volume: i64 = m
        .get_one::<String>("min-volume")
        .unwrap_or(&"100".to_string())
        .parse()
        .context("invalid --min-volume")?;
    let (rank_min, rank_max) = parse_range(
        m.get_one::<String>("rank")
            .map(|s| s.as_str())
            .unwrap_or("4..20"),
    )?;
    let sort_key = m
        .get_one::<String>("sort")
        .map(|s| s.as_str())
        .unwrap_or("search_volume");

    let path = "/v3/dataforseo_labs/google/ranked_keywords/live";
    let body = json!([{
        "target": site,
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
    let rows = ranked_keyword_wins(&raw, rank_min, rank_max, min_volume, sort_key);
    print_wrapper_output(
        format,
        json!({"site": site_raw, "loc": loc, "lang": lang, "limit": limit, "rank": format!("{rank_min}..{rank_max}"), "min_volume": min_volume, "sort": sort_key}),
        raw,
        rows,
        KEYWORDS_COLS,
    )
}

fn ranked_keyword_rows(raw: &Value) -> Vec<Map<String, Value>> {
    let mut rows = Vec::new();
    for item in items_from_tasks(raw) {
        rows.push(ranked_row(item));
    }
    rows
}

fn ranked_keyword_wins(
    raw: &Value,
    rank_min: i64,
    rank_max: i64,
    min_volume: i64,
    sort_key: &str,
) -> Vec<Map<String, Value>> {
    let mut picked: Vec<(i64, i64, Map<String, Value>)> = Vec::new();
    for item in items_from_tasks(raw) {
        let rank = dig(item, &["ranked_serp_element", "serp_item", "rank_absolute"])
            .and_then(|v| v.as_i64())
            .unwrap_or(i64::MAX);
        if rank < rank_min || rank > rank_max {
            continue;
        }
        let volume = dig(item, &["keyword_data", "keyword_info", "search_volume"])
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        if volume < min_volume {
            continue;
        }
        let etv = dig(item, &["ranked_serp_element", "serp_item", "etv"])
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let primary = match sort_key {
            "etv" => (etv * 1000.0) as i64,
            _ => volume,
        };

        picked.push((primary, rank, ranked_row(item)));
    }

    picked.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    picked.into_iter().map(|t| t.2).collect()
}

fn ranked_row(item: &Value) -> Map<String, Value> {
    let mut row = Map::new();
    row.insert(
        "keyword".to_string(),
        dig(item, &["keyword_data", "keyword"])
            .cloned()
            .unwrap_or(Value::Null),
    );
    row.insert(
        "search_volume".to_string(),
        dig(item, &["keyword_data", "keyword_info", "search_volume"])
            .cloned()
            .unwrap_or(Value::Null),
    );
    row.insert(
        "keyword_difficulty".to_string(),
        dig(
            item,
            &["keyword_data", "keyword_properties", "keyword_difficulty"],
        )
        .cloned()
        .unwrap_or(Value::Null),
    );
    row.insert(
        "competition_level".to_string(),
        dig(item, &["keyword_data", "keyword_info", "competition_level"])
            .cloned()
            .unwrap_or(Value::Null),
    );
    row.insert(
        "cpc".to_string(),
        dig(item, &["keyword_data", "keyword_info", "cpc"])
            .cloned()
            .unwrap_or(Value::Null),
    );
    row.insert(
        "intent".to_string(),
        dig(item, &["keyword_data", "search_intent_info", "main_intent"])
            .cloned()
            .unwrap_or(Value::Null),
    );
    row.insert(
        "rank".to_string(),
        dig(item, &["ranked_serp_element", "serp_item", "rank_absolute"])
            .cloned()
            .unwrap_or(Value::Null),
    );
    row.insert(
        "etv".to_string(),
        dig(item, &["ranked_serp_element", "serp_item", "etv"])
            .cloned()
            .unwrap_or(Value::Null),
    );
    row.insert(
        "url".to_string(),
        dig(item, &["ranked_serp_element", "serp_item", "url"])
            .cloned()
            .unwrap_or(Value::Null),
    );
    row.insert(
        "title".to_string(),
        dig(item, &["ranked_serp_element", "serp_item", "title"])
            .cloned()
            .unwrap_or(Value::Null),
    );
    row
}

fn parse_range(raw: &str) -> Result<(i64, i64)> {
    let (a, b) = if let Some((a, b)) = raw.split_once("..=") {
        (a, b)
    } else if let Some((a, b)) = raw.split_once("..") {
        (a, b)
    } else {
        anyhow::bail!("invalid range: {raw}");
    };
    let a: i64 = a.trim().parse().context("parse range start")?;
    let b: i64 = b.trim().parse().context("parse range end")?;
    if a > b {
        anyhow::bail!("invalid range: start > end");
    }
    Ok((a, b))
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
