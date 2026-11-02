pub mod board;
pub mod disk_manager;
pub mod task;
pub mod user;

#[cfg(test)]
pub mod tests {
    use anyhow::Result;

    use crate::{
        disk_manager::{get_user, registrate_user},
        user::User,
    };

    use super::*;

    #[test]
    pub fn test_registration() -> Result<()> {
        registrate_user("test".to_string(), "qwerty".to_string())
    }

    #[test]
    pub fn test_get_user() -> Result<()> {
        let user = get_user("test".to_string()).unwrap();
        println!("{:#?}", user);
        Ok(())
    }
}
