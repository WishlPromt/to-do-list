use crate::task::TaskType::{Progressive, Switch};

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub text: String,
    pub task_type: TaskType,
}

impl Task {
    pub fn new(name: String, text: String, task_type: TaskType) -> Self {
        Self {
            name,
            text,
            task_type,
        }
    }

    pub fn edit(
        mut self,
        new_name: Option<String>,
        new_text: Option<String>,
        new_task_type: Option<TaskType>,
    ) {
        if let Some(new_name) = new_name {
            self.name = new_name;
        }
        if let Some(new_text) = new_text {
            self.text = new_text;
        }
        if let Some(new_task_type) = new_task_type {
            self.task_type = new_task_type;
        }
    }

    pub fn switch(&mut self) {
        match &mut self.task_type {
            Switch(state) => {
                *state = !*state;
            }
            _ => {}
        }
    }

    pub fn switch_progress(&mut self, new_status: u8) {
        match &mut self.task_type {
            Progressive(max, status) => {
                let mut st = new_status;
                if new_status > *max {
                    st = *max;
                }
                *status = st;
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum TaskType {
    Switch(bool),        // status
    Progressive(u8, u8), // max, status
}

impl TaskType {
    pub fn new_switch() -> Self {
        Self::Switch(false)
    }

    pub fn new_progressive(max: u8) -> Self {
        Self::Progressive(max, 0)
    }
}
