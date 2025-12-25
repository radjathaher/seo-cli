use anyhow::{Context, Result};
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

pub fn credentials_path() -> PathBuf {
    let mut base = home_dir().expect("home dir");
    base.push(".seo-cli-rs");
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

pub fn resolve_credentials(matches: &ArgMatches) -> Result<Credentials> {
    if let (Some(l), Some(p)) = (
        matches.get_one::<String>("login"),
        matches.get_one::<String>("password"),
    ) {
        return Ok(Credentials {
            login: l.clone(),
            password: p.clone(),
        });
    }

    let env_login = std::env::var("DATAFORSEO_LOGIN").ok();
    let env_password = std::env::var("DATAFORSEO_PASSWORD").ok();
    if let (Some(l), Some(p)) = (env_login, env_password) {
        return Ok(Credentials {
            login: l,
            password: p,
        });
    }

    if let Some(c) = read_credentials_file()? {
        return Ok(c);
    }

    let no_interactive = matches.get_flag("no-interactive");
    if no_interactive {
        anyhow::bail!("missing credentials: set flags, env, or credentials file");
    }

    let creds = prompt_credentials()?;
    write_credentials(&creds).ok();
    Ok(creds)
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
