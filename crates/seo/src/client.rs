use anyhow::Result;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

use crate::config::Credentials;

pub struct DataForSeoClient {
    base_url: String,
    creds: Credentials,
    http: reqwest::Client,
}

impl DataForSeoClient {
    pub fn new(base_url: String, creds: Credentials, timeout: Duration) -> Self {
        let http = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("reqwest client");
        Self {
            base_url,
            creds,
            http,
        }
    }

    pub fn build_curl(&self, path: &str, method: &str, body: Option<&Value>) -> Result<String> {
        let url = format!("{}{}", self.base_url, path);
        let mut parts: Vec<String> = Vec::new();
        parts.push("curl".to_string());
        parts.push("-sS".to_string());
        parts.push("-u".to_string());
        parts.push(r#"$DATAFORSEO_LOGIN:$DATAFORSEO_PASSWORD"#.to_string());
        parts.push("-X".to_string());
        parts.push(method.to_uppercase());
        parts.push(url);
        if let Some(b) = body {
            let json = serde_json::to_string(b)?;
            parts.push("-H".to_string());
            parts.push("Content-Type: application/json".to_string());
            parts.push("--data-binary".to_string());
            parts.push(json);
        }
        Ok(parts
            .into_iter()
            .map(|p| sh_quote(&p))
            .collect::<Vec<_>>()
            .join(" "))
    }

    pub fn build_request(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        self.http
            .request(method, url)
            .basic_auth(&self.creds.login, Some(&self.creds.password))
    }

    #[allow(dead_code)]
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let res = self.build_request(Method::GET, path).send().await?;
        let res = res.error_for_status()?;
        Ok(res.json::<T>().await?)
    }

    #[allow(dead_code)]
    pub async fn post<TReq: Serialize, TRes: DeserializeOwned>(
        &self,
        path: &str,
        body: &TReq,
    ) -> Result<TRes> {
        let res = self
            .build_request(Method::POST, path)
            .json(body)
            .send()
            .await?;
        let res = res.error_for_status()?;
        Ok(res.json::<TRes>().await?)
    }

    pub fn get_json(&self, path: &str) -> Result<Value> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let res = self.build_request(Method::GET, path).send().await?;
            let res = res.error_for_status()?;
            Ok::<_, anyhow::Error>(res.json::<Value>().await?)
        })
    }

    pub fn post_json(&self, path: &str, body: &Value) -> Result<Value> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let res = self
                .build_request(Method::POST, path)
                .json(body)
                .send()
                .await?;
            let res = res.error_for_status()?;
            Ok::<_, anyhow::Error>(res.json::<Value>().await?)
        })
    }
}

fn sh_quote(s: &str) -> String {
    let safe = s
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "-._~/:=@".contains(c));
    if safe {
        return s.to_string();
    }
    if s.contains('$') && !s.contains(' ') && !s.contains('\'') && !s.contains('"') {
        return format!("\"{s}\"");
    }
    let escaped = s.replace('\'', r#"'\''"#);
    format!("'{escaped}'")
}
