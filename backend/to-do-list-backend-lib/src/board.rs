use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

use crate::task::Task;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Board {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tasks: vec![],
        }
    }

    pub fn add_task(&mut self, task: Task) -> Result<()> {
        self.tasks.push(task);
        Ok(())
    }

    pub fn remove_task(&mut self, task_index: usize) -> Result<()> {
        if self.tasks.len() >= task_index {
            self.tasks.remove(task_index);
            return Ok(());
        }
        bail!("Error when try to remove task")
    }
}
