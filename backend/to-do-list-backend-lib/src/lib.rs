pub mod board;
pub mod disk_manager;
pub mod task;
pub mod user;

#[cfg(test)]
pub mod tests {
    use anyhow::Result;

    use crate::{
        board::Board,
        disk_manager::{get_user, registrate_user, save_user_boards},
        user::User,
    };

    use super::*;

    #[test]
    pub fn test_registration() -> Result<()> {
        registrate_user("test".to_string(), "qwerty".to_string())
    }

    #[test]
    pub fn test_get_user() -> Result<()> {
        let user = get_user("test".to_string(), "qwert".to_string()).unwrap();
        println!("{:#?}", user);
        Ok(())
    }

    #[test]
    pub fn test_save_user_with_correct_password() -> Result<()> {
        let login = "test".to_string();
        let password = "qwerty".to_string();
        let mut user = get_user(login.clone(), password.clone()).unwrap();
        let board = Board::new("Test".to_string());
        user.add_board(board)?;
        save_user_boards(login, password, user.boards)?;
        Ok(())
    }

    #[test]
    pub fn test_save_user_with_incorrect_password() -> Result<()> {
        let login = "test".to_string();
        let password = "qwerty2".to_string();
        let mut user = get_user(login.clone(), password.clone()).unwrap();
        let board = Board::new("Test".to_string());
        user.add_board(board)?;
        save_user_boards(login, password, user.boards)?;
        Ok(())
    }
}
