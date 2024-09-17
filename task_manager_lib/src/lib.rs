use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Result};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub due_date: Option<String>,
    pub priority: Option<u8>,
    pub completed: bool,
}

pub fn add_task(tasks: &mut Vec<Task>, name: String, due_date: Option<String>, priority: Option<u8>) {
    let new_task = Task { id: tasks.len() as u32, name, due_date, priority, completed: false };
    tasks.push(new_task);
}

pub fn save_tasks(tasks: &Vec<Task>) -> Result<()> {
    let mut file = File::create("tasks.json")?;
    let json = serde_json::to_string(&tasks)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}


