use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::board::Board;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub password: String,
    pub boards: Vec<Board>,
}

impl User {
    pub fn new(login: String, password: String) -> Self {
        Self {
            login,
            password,
            boards: vec![],
        }
    }

    pub fn registration(login: String, password: String) -> Result<()> {
        let user = Self::new(login, password);
        Ok(())
    }
}
