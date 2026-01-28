use anyhow::{Context, Result};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

#[derive(Clone, Debug)]
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

pub fn resolve_credentials() -> Result<ResolvedCredentials> {
    if let Some(raw) = std::env::var("DATAFORSEO_AUTH").ok().map(|v| v.trim().to_string()) {
        if !raw.is_empty() {
            let creds = decode_basic_auth(&raw)?;
            return Ok(ResolvedCredentials {
                creds,
                curl_auth: CurlAuthHint::BasicEnv,
            });
        }
    }

    let env_login = std::env::var("DATAFORSEO_LOGIN").ok();
    let env_password = std::env::var("DATAFORSEO_PASSWORD").ok();
    if let (Some(l), Some(p)) = (env_login, env_password) {
        if l.trim().is_empty() || p.is_empty() {
            anyhow::bail!("missing credentials: set DATAFORSEO_AUTH or DATAFORSEO_LOGIN/DATAFORSEO_PASSWORD");
        }
        return Ok(ResolvedCredentials {
            creds: Credentials {
                login: l,
                password: p,
            },
            curl_auth: CurlAuthHint::LoginPasswordEnv,
        });
    }

    anyhow::bail!("missing credentials: set DATAFORSEO_AUTH or DATAFORSEO_LOGIN/DATAFORSEO_PASSWORD")
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
