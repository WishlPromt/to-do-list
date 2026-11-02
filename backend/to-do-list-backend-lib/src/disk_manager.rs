use std::fs;

use anyhow::{Result, bail};

use crate::{
    board::Board,
    user::{self, User},
};

const USERS_PATH: &str = "./users";

pub fn path_to_data() -> String {
    if !fs::exists(USERS_PATH).unwrap() {
        fs::create_dir(USERS_PATH).unwrap();
    }
    USERS_PATH.to_string()
}

pub fn get_user(login: String, password: String) -> Result<User> {
    let path = format!("{}/{}.json", path_to_data(), login);
    if fs::exists(&path).unwrap() {
        let json = fs::read_to_string(&path).unwrap();
        let user: User = serde_json::from_str(&json)?;
        if user.password == password {
            return Ok(user);
        } else {
            bail!("Incorrect password")
        }
    }
    bail!("User don`t exists")
}

pub fn registrate_user(login: String, password: String) -> Result<()> {
    let path = format!("{}/{}.json", path_to_data(), login);
    if !fs::exists(&path).unwrap() {
        let user = User::new(login, password);
        let json = serde_json::to_string(&user).unwrap();
        fs::write(path, json).unwrap();
        return Ok(());
    }
    bail!("User already exists")
}

pub fn save_user_boards(login: String, password: String, boards: Vec<Board>) -> Result<()> {
    let path = format!("{}/{}.json", path_to_data(), login);
    if fs::exists(&path).unwrap() {
        let json = fs::read_to_string(&path).unwrap();
        let mut user: User = serde_json::from_str(&json)?;
        if user.password == password {
            user.boards = boards;
            let json = serde_json::to_string(&user).unwrap();
            fs::write(path, json).unwrap();
            return Ok(());
        }
        bail!("Incorrect password")
    }
    bail!("User don`t exists")
}
