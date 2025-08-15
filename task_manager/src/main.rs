use prelude::*;

mod models;
mod repo;

mod prelude {
    pub use crate::models::*;
    pub use crate::repo::*;
    pub use std::collections::{HashMap, HashSet};
    pub use std::io::Error;
}

fn main() {
    // Testing
    let mut repo: TaskRepo = TaskRepo::new();
    repo.add_task(
        "Hello World!".to_string(),
        "Testing 123..".to_string(),
        vec!["test".to_string(), "note".to_string()],
    );
    match repo.complete_task(1) {
        Ok(_) => println!("Task marked as done!"),
        Err(e) => println!("Error: {}", e),
    }
    let tasks: Vec<&Task> = repo.list_tasks();
    if let Some(task) = tasks.first() {
        println!("{:?}", task);
    }
}
