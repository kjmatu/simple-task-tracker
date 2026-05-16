extern crate toml;
extern crate serde;

use serde::{Serialize, Deserialize};
use std::path::{PathBuf};
use std::env;

pub fn start(task_id: u8) {
    println!("Starting {}", task_id);
}

pub fn stop() {
    println!("Stopping");
}

pub fn status() {
    println!("Status running");
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TasksFile {
    tasks: Vec<Task>,
}

fn resolve_task_file_path() -> PathBuf {
    if let Ok(path) = env::var("STT_TASK_FILE") {
        return PathBuf::from(path);
    }

    let home = env::var("HOME").expect("Could not find HOME environment variable");
    PathBuf::from(home).join(".config").join("simple-task-tracker").join("tasks.toml")
}


pub fn list() {
    let task_file_path = resolve_task_file_path();
    println!("Reading tasks from {}", task_file_path.display());

    let contents = std::fs::read_to_string(task_file_path).unwrap();
    let tasks_file: TasksFile = toml::from_str(&contents).expect("Could not parse tasks file");

    for task in tasks_file.tasks {
        println!("{}: {}", task.id, task.name);
    }
}