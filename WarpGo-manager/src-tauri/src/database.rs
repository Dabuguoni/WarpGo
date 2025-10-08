use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub email: String,
    pub account_data: String,
    pub health_status: String,
    pub limit_info: Option<String>,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountData {
    pub email: String,
    #[serde(rename = "stsTokenManager")]
    pub sts_token_manager: TokenManager,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenManager {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "expirationTime")]
    pub expiration_time: i64,
}

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path).context("Failed to open database")?;
        let db = Database {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Create accounts table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS accounts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT UNIQUE NOT NULL,
                account_data TEXT NOT NULL,
                health_status TEXT DEFAULT 'healthy',
                limit_info TEXT DEFAULT 'Not Updated',
                last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Create proxy settings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS proxy_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        // Insert default certificate approval setting
        conn.execute(
            "INSERT OR IGNORE INTO proxy_settings (key, value) VALUES ('certificate_approved', 'false')",
            [],
        )?;

        Ok(())
    }

    pub fn add_account(&self, account_json: &str) -> Result<String> {
        let account_data: AccountData = serde_json::from_str(account_json)
            .context("Invalid JSON format")?;
        
        let email = &account_data.email;
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "INSERT OR REPLACE INTO accounts (email, account_data, last_updated) 
             VALUES (?1, ?2, CURRENT_TIMESTAMP)",
            params![email, account_json],
        )?;

        Ok(format!("Account {} added successfully", email))
    }

    pub fn get_accounts(&self) -> Result<Vec<Account>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, email, account_data, health_status, limit_info, last_updated 
             FROM accounts ORDER BY email"
        )?;

        let accounts = stmt
            .query_map([], |row| {
                Ok(Account {
                    id: row.get(0)?,
                    email: row.get(1)?,
                    account_data: row.get(2)?,
                    health_status: row.get(3)?,
                    limit_info: row.get(4)?,
                    last_updated: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(accounts)
    }

    pub fn get_account_by_email(&self, email: &str) -> Result<Option<Account>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, email, account_data, health_status, limit_info, last_updated 
             FROM accounts WHERE email = ?1"
        )?;

        let account = stmt
            .query_row([email], |row| {
                Ok(Account {
                    id: row.get(0)?,
                    email: row.get(1)?,
                    account_data: row.get(2)?,
                    health_status: row.get(3)?,
                    limit_info: row.get(4)?,
                    last_updated: row.get(5)?,
                })
            })
            .optional()?;

        Ok(account)
    }

    pub fn delete_account(&self, email: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Delete account
        conn.execute("DELETE FROM accounts WHERE email = ?1", params![email])?;
        
        // Clear active account if it's the deleted one
        let active: Option<String> = conn
            .query_row(
                "SELECT value FROM proxy_settings WHERE key = 'active_account'",
                [],
                |row| row.get(0),
            )
            .optional()?;

        if let Some(active_email) = active {
            if active_email == email {
                conn.execute(
                    "DELETE FROM proxy_settings WHERE key = 'active_account'",
                    [],
                )?;
            }
        }

        Ok(())
    }

    pub fn update_account_health(&self, email: &str, health_status: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE accounts SET health_status = ?1, last_updated = CURRENT_TIMESTAMP 
             WHERE email = ?2",
            params![health_status, email],
        )?;
        Ok(())
    }

    pub fn update_account_token(&self, email: &str, token_data: &TokenManager) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        let account_json: String = conn.query_row(
            "SELECT account_data FROM accounts WHERE email = ?1",
            params![email],
            |row| row.get(0),
        )?;

        let mut account_data: AccountData = serde_json::from_str(&account_json)?;
        account_data.sts_token_manager = token_data.clone();

        let updated_json = serde_json::to_string(&account_data)?;
        
        conn.execute(
            "UPDATE accounts SET account_data = ?1, last_updated = CURRENT_TIMESTAMP 
             WHERE email = ?2",
            params![updated_json, email],
        )?;

        Ok(())
    }

    pub fn update_account_limit_info(&self, email: &str, limit_info: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE accounts SET limit_info = ?1, last_updated = CURRENT_TIMESTAMP 
             WHERE email = ?2",
            params![limit_info, email],
        )?;
        Ok(())
    }

    pub fn set_active_account(&self, email: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO proxy_settings (key, value) VALUES ('active_account', ?1)",
            params![email],
        )?;
        Ok(())
    }

    pub fn get_active_account(&self) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let result = conn
            .query_row(
                "SELECT value FROM proxy_settings WHERE key = 'active_account'",
                [],
                |row| row.get(0),
            )
            .optional()?;
        Ok(result)
    }

    pub fn clear_active_account(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM proxy_settings WHERE key = 'active_account'",
            [],
        )?;
        Ok(())
    }

    pub fn is_certificate_approved(&self) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let result: Option<String> = conn
            .query_row(
                "SELECT value FROM proxy_settings WHERE key = 'certificate_approved'",
                [],
                |row| row.get(0),
            )
            .optional()?;

        Ok(result.map(|v| v == "true").unwrap_or(false))
    }

    pub fn set_certificate_approved(&self, approved: bool) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let value = if approved { "true" } else { "false" };
        conn.execute(
            "INSERT OR REPLACE INTO proxy_settings (key, value) VALUES ('certificate_approved', ?1)",
            params![value],
        )?;
        Ok(())
    }
}
