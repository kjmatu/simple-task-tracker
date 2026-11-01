extern crate serde;
extern crate toml;

use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

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
    PathBuf::from(home)
        .join(".config")
        .join("simple-task-tracker")
        .join("tasks.toml")
}

pub fn list() {
    let task_file_path = resolve_task_file_path();
    println!("Reading tasks from {}", task_file_path.display());

    let contents = match std::fs::read_to_string(&task_file_path) {
        Ok(contents) => contents,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Error: Task file not found at {}", task_file_path.display());
            eprintln!("Create the file first, or set the STT_TASK_FILE environment variable.");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error reading task file: {}", e);
            std::process::exit(1);
        }
    };

    let tasks_file = match toml::from_str::<TasksFile>(&contents) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error parsing task file: {}", e);
            std::process::exit(1);
        }
    };

    for task in tasks_file.tasks {
        println!("{}: {}", task.id, task.name);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    mod resolve_task_file_path {
        use std::env;
        use std::path::PathBuf;

        #[test]
        fn check_resolved_task_file_path_existence() {
            const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");
            let task_toml = PathBuf::from(PROJECT_ROOT).join("tests").join("fixture").join("task.toml");
            unsafe {
                env::set_var("STT_TASK_FILE", task_toml.to_str());
            }
            let task_file_path = resolve_task_file_path();
            assert!(task_file_path.exists(), "タスクファイルが存在しません: {}", task_file_path.display())
        }
    }
}
