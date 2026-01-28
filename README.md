# seo-cli

Single-binary Rust CLI for DataForSEO.

- `seo dfs ...` mirrors the full DataForSEO API v3 OpenAPI command tree (discoverable via `--help` at every level).
- Top-level wrapper commands provide “jobs-to-be-done” workflows for common SEO tasks (human-friendly TSV by default).

## Requirements

- DataForSEO account (`login` + `password`)

## Install

### From this repo (local)

```bash
cargo install --path crates/seo
```

### Install script (macOS arm64 + Linux x86_64)

```bash
curl -fsSL https://raw.githubusercontent.com/radjathaher/seo-cli/main/scripts/install.sh | bash
```

Pin a version:

```bash
SEO_CLI_VERSION="v0.1.3" curl -fsSL https://raw.githubusercontent.com/radjathaher/seo-cli/main/scripts/install.sh | bash
```

### From Git (requires Rust toolchain)

```bash
cargo install --git https://github.com/radjathaher/seo-cli.git --package seo
```

### Build a release binary

```bash
cargo build -p seo --release
./target/release/seo -h
```

## Auth

Credential precedence (highest → lowest):

1) env `DATAFORSEO_AUTH` (Base64 `login:password`, optional `Basic ...`)
2) env `DATAFORSEO_LOGIN` / `DATAFORSEO_PASSWORD`

## Output

- Default: TSV (`--format tsv`)
- JSON: `--format json`

Tip (pretty TSV in terminal):

```bash
seo website keywords --site soreno.ai --limit 20 | column -t -s $'\t'
```

## Explore the full DataForSEO API (OpenAPI-driven)

Start at the root, then drill down:

```bash
seo dfs -h
seo dfs dataforseo-labs -h
seo dfs dataforseo-labs google -h
```

Example (raw endpoint call; exact path/flags come from OpenAPI):

```bash
seo dfs dataforseo-labs google ranked-keywords live --target soreno.ai --location-code 2840 --language-code en --limit 20
```

Generate a curl command instead of calling the API:

```bash
seo dfs dataforseo-labs google ranked-keywords live --target soreno.ai --location-code 2840 --language-code en --limit 20 --curl
```

`--curl` never embeds secrets; it uses `$DATAFORSEO_AUTH` when set, otherwise `$DATAFORSEO_LOGIN/$DATAFORSEO_PASSWORD`.

## Wrapper commands (JTBD)

Common workflows:

```bash
seo website traffic  --site soreno.ai
seo website keywords --site soreno.ai --limit 50
seo website wins     --site soreno.ai --rank 4..20 --min-volume 100

seo keyword ideas --seed "case interview" --limit 50
seo keyword eval  --kw "leadership strengths"

seo serp top --kw "leadership strengths" --depth 20
```

## Updating the generated API surface (OpenAPI → CLI)

`openapi.yaml` is a snapshot of DataForSEO OpenApiDocumentation. If it changes:

```bash
cargo run -p xtask -- gen
```

## License

Licensed under either of:

- Apache License, Version 2.0 (`LICENSE-APACHE`)
- MIT license (`LICENSE-MIT`)
