use anyhow::{Context, Result};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use clap::ArgMatches;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

#[derive(Clone, Copy, Debug)]
pub enum CurlAuthHint {
    BasicEnv,
    LoginPasswordEnv,
}

#[derive(Clone, Debug)]
pub struct ResolvedCredentials {
    pub creds: Credentials,
    pub curl_auth: CurlAuthHint,
}

pub fn credentials_path() -> PathBuf {
    let mut base = home_dir().expect("home dir");
    base.push(".seo-cli");
    base.push("credentials.toml");
    base
}

pub fn read_credentials_file() -> Result<Option<Credentials>> {
    let path = credentials_path();
    if !path.exists() {
        return Ok(None);
    }
    let raw = fs::read_to_string(&path).context("read credentials file")?;
    if raw.trim().is_empty() {
        return Ok(None);
    }
    let creds: Credentials = toml::from_str(&raw).context("parse credentials file")?;
    if creds.login.is_empty() || creds.password.is_empty() {
        return Ok(None);
    }
    Ok(Some(creds))
}

pub fn write_credentials(creds: &Credentials) -> Result<()> {
    let path = credentials_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("create config dir")?;
    }
    let raw = toml::to_string(creds).context("serialize credentials")?;
    fs::write(&path, raw).context("write credentials")?;
    set_private_perms(&path).ok();
    Ok(())
}

pub fn clear_credentials() -> Result<()> {
    let path = credentials_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }
    fs::write(&path, "").context("clear credentials")?;
    set_private_perms(&path).ok();
    Ok(())
}

pub fn prompt_credentials() -> Result<Credentials> {
    let mut login = String::new();
    print!("DataForSEO login: ");
    io::stdout().flush().ok();
    io::stdin().read_line(&mut login).context("read login")?;
    let login = login.trim().to_string();
    let password = rpassword::prompt_password("DataForSEO password: ")?;
    Ok(Credentials { login, password })
}

pub fn resolve_credentials(matches: &ArgMatches) -> Result<ResolvedCredentials> {
    if let Some(raw) = std::env::var("DATAFORSEO_AUTH").ok().map(|v| v.trim().to_string()) {
        if !raw.is_empty() {
            let creds = decode_basic_auth(&raw)?;
            return Ok(ResolvedCredentials {
                creds,
                curl_auth: CurlAuthHint::BasicEnv,
            });
        }
    }

    if let (Some(l), Some(p)) = (
        matches.get_one::<String>("login"),
        matches.get_one::<String>("password"),
    ) {
        return Ok(ResolvedCredentials {
            creds: Credentials {
                login: l.clone(),
                password: p.clone(),
            },
            curl_auth: CurlAuthHint::LoginPasswordEnv,
        });
    }

    let env_login = std::env::var("DATAFORSEO_LOGIN").ok();
    let env_password = std::env::var("DATAFORSEO_PASSWORD").ok();
    if let (Some(l), Some(p)) = (env_login, env_password) {
        return Ok(ResolvedCredentials {
            creds: Credentials {
                login: l,
                password: p,
            },
            curl_auth: CurlAuthHint::LoginPasswordEnv,
        });
    }

    if let Some(c) = read_credentials_file()? {
        return Ok(ResolvedCredentials {
            creds: c,
            curl_auth: CurlAuthHint::LoginPasswordEnv,
        });
    }

    let no_interactive = matches.get_flag("no-interactive");
    if no_interactive {
        anyhow::bail!("missing credentials: set flags, env, or credentials file");
    }

    let creds = prompt_credentials()?;
    write_credentials(&creds).ok();
    Ok(ResolvedCredentials {
        creds,
        curl_auth: CurlAuthHint::LoginPasswordEnv,
    })
}

fn set_private_perms(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(path, perms)?;
    }
    Ok(())
}

fn decode_basic_auth(raw: &str) -> Result<Credentials> {
    let trimmed = raw.trim();
    let value = trimmed
        .strip_prefix("Basic ")
        .or_else(|| trimmed.strip_prefix("basic "))
        .unwrap_or(trimmed)
        .trim();

    if value.is_empty() {
        anyhow::bail!("DATAFORSEO_AUTH is empty");
    }

    let decoded = BASE64
        .decode(value)
        .context("decode DATAFORSEO_AUTH")?;
    let decoded = String::from_utf8(decoded).context("DATAFORSEO_AUTH must be valid UTF-8")?;
    let mut parts = decoded.splitn(2, ':');
    let login = parts.next().unwrap_or("").trim().to_string();
    let password = parts.next().unwrap_or("").to_string();

    if login.is_empty() || password.is_empty() {
        anyhow::bail!("DATAFORSEO_AUTH must decode to login:password");
    }

    Ok(Credentials { login, password })
}
