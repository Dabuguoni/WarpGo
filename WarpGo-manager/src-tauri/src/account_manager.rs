use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::database::{AccountData, Database, TokenManager};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitInfo {
    #[serde(rename = "requestsUsedSinceLastRefresh")]
    pub requests_used: i32,
    #[serde(rename = "requestLimit")]
    pub request_limit: i32,
    #[serde(rename = "isUnlimited")]
    pub is_unlimited: bool,
}

pub struct AccountManager {
    db: Database,
    client: Client,
}

impl AccountManager {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        }
    }

    pub fn refresh_token(&self, email: &str) -> Result<TokenManager> {
        let account = self
            .db
            .get_account_by_email(email)?
            .context("Account not found")?;

        let account_data: AccountData = serde_json::from_str(&account.account_data)?;

        let url = format!(
            "https://securetoken.googleapis.com/v1/token?key={}",
            account_data.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "WarpAccountManager/1.0")
            .json(&json!({
                "grant_type": "refresh_token",
                "refresh_token": account_data.sts_token_manager.refresh_token
            }))
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!("Token refresh failed: {}", response.status());
        }

        let token_response: RefreshTokenResponse = response.json()?;
        let expires_in: i64 = token_response.expires_in.parse().unwrap_or(3600);

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let new_token = TokenManager {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expiration_time: current_time + (expires_in * 1000),
        };

        self.db.update_account_token(email, &new_token)?;

        Ok(new_token)
    }

    pub fn get_limit_info(&self, email: &str) -> Result<String> {
        let account = self
            .db
            .get_account_by_email(email)?
            .context("Account not found")?;

        let account_data: AccountData = serde_json::from_str(&account.account_data)?;

        // Check token expiration
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let token = if current_time >= account_data.sts_token_manager.expiration_time {
            self.refresh_token(email)?
        } else {
            account_data.sts_token_manager
        };

        let os_info = get_os_info();
        let url = "https://app.warp.dev/graphql/v2?op=GetRequestLimitInfo";

        let query = r#"
            query GetRequestLimitInfo($requestContext: RequestContext!) {
              user(requestContext: $requestContext) {
                __typename
                ... on UserOutput {
                  user {
                    requestLimitInfo {
                      requestLimit
                      requestsUsedSinceLastRefresh
                      isUnlimited
                    }
                  }
                }
              }
            }
        "#;

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token.access_token))
            .header("X-Warp-Client-Version", "v0.2025.08.27.08.11.stable_04")
            .header("X-Warp-Os-Category", &os_info.category)
            .header("X-Warp-Os-Name", &os_info.name)
            .header("X-Warp-Os-Version", &os_info.version)
            .json(&json!({
                "query": query,
                "variables": {
                    "requestContext": {
                        "clientContext": {
                            "version": "v0.2025.08.27.08.11.stable_04"
                        },
                        "osContext": {
                            "category": os_info.category,
                            "name": os_info.category,
                            "version": os_info.version,
                            "linuxKernelVersion": serde_json::Value::Null
                        }
                    }
                },
                "operationName": "GetRequestLimitInfo"
            }))
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get limit info: {}", response.status());
        }

        let json_response: serde_json::Value = response.json()?;

        if let Some(limit_info) = json_response
            .get("data")
            .and_then(|d| d.get("user"))
            .and_then(|u| u.get("user"))
            .and_then(|u| u.get("requestLimitInfo"))
        {
            let used = limit_info
                .get("requestsUsedSinceLastRefresh")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let limit = limit_info
                .get("requestLimit")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);

            return Ok(format!("{}/{}", used, limit));
        }

        Ok("N/A".to_string())
    }

    pub fn check_token_expiration(&self, email: &str) -> Result<bool> {
        let account = self
            .db
            .get_account_by_email(email)?
            .context("Account not found")?;

        let account_data: AccountData = serde_json::from_str(&account.account_data)?;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // Token expires in less than 1 minute
        Ok(current_time >= (account_data.sts_token_manager.expiration_time - 60000))
    }
}

#[derive(Debug)]
pub struct OsInfo {
    pub category: String,
    pub name: String,
    pub version: String,
}

pub fn get_os_info() -> OsInfo {
    #[cfg(target_os = "windows")]
    {
        OsInfo {
            category: "Windows".to_string(),
            name: "Windows".to_string(),
            version: std::env::var("OS").unwrap_or_else(|_| "Unknown".to_string()),
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let version = Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_else(|| "Unknown".to_string())
            .trim()
            .to_string();

        OsInfo {
            category: "Darwin".to_string(),
            name: "macOS".to_string(),
            version,
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        OsInfo {
            category: "Linux".to_string(),
            name: "Linux".to_string(),
            version: "Unknown".to_string(),
        }
    }
}
