mod cli;
mod client;
mod config;
mod generated;
mod output;
mod wrappers;

use crate::client::DataForSeoClient;
use crate::output::print_output;
use anyhow::{Context, Result};
use clap::ArgMatches;

fn main() -> Result<()> {
    let cmd = cli::build_cli();
    let matches = cmd.get_matches();

    if let Some(("auth", sub)) = matches.subcommand() {
        return handle_auth(sub);
    }

    let auth = config::resolve_credentials(&matches)?;
    let base_url = cli::resolve_base_url(&matches)?;
    let timeout = cli::resolve_timeout(&matches)?;
    let format = cli::resolve_format(&matches)?;

    let client = DataForSeoClient::new(base_url, auth.creds.clone(), timeout, auth.curl_auth);

    if wrappers::handle(&matches, &client, format)? {
        return Ok(());
    }

    let resolved = cli::resolve_operation(&matches)?;

    let response = if resolved.dry_run {
        let curl = client.build_curl(&resolved.path, &resolved.method, resolved.body.as_ref())?;
        println!("{curl}");
        if matches.get_flag("curl") {
            match auth.curl_auth {
                config::CurlAuthHint::BasicEnv => {
                    eprintln!(
                        "note: uses $DATAFORSEO_AUTH (secrets not embedded in --curl output)"
                    );
                }
                config::CurlAuthHint::LoginPasswordEnv => {
                    eprintln!(
                        "note: uses $DATAFORSEO_LOGIN/$DATAFORSEO_PASSWORD (secrets not embedded in --curl output)"
                    );
                }
            }
        }
        return Ok(());
    } else {
        match resolved.method.as_str() {
            "get" => client.get_json(&resolved.path).context("GET failed")?,
            "post" => {
                let body = resolved.body.as_ref().context("missing request body")?;
                client
                    .post_json(&resolved.path, body)
                    .context("POST failed")?
            }
            other => anyhow::bail!("unsupported method: {other}"),
        }
    };

    print_output(&response, format)?;

    Ok(())
}

fn handle_auth(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("login", _sub)) => {
            let creds = config::prompt_credentials()?;
            config::write_credentials(&creds)?;
            println!(
                "saved credentials to {}",
                config::credentials_path().display()
            );
            Ok(())
        }
        Some(("logout", _sub)) => {
            config::clear_credentials()?;
            println!(
                "cleared credentials in {}",
                config::credentials_path().display()
            );
            Ok(())
        }
        _ => anyhow::bail!("unknown auth command"),
    }
}
