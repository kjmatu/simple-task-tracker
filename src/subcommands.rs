pub fn start(task_id: u8) {
    println!("Starting {}", task_id);
}

pub fn stop() {
    println!("Stopping");
}

pub fn status() {
    println!("Status running");
}

pub fn list() {
    println!("Show task list");
}