use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

use crate::board::{self, Board};

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

    pub fn add_board(&mut self, board: Board) -> Result<()> {
        if let Some(_) = self.boards.iter().find(|b| b.name == board.name) {
            bail!("Board with same name already exists")
        }
        self.boards.push(board);
        Ok(())
    }
}
