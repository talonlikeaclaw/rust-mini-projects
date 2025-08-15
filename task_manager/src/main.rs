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
    let path = "tasks.json";
    let mut repo: TaskRepo = match TaskRepo::load_from_path(path) {
        Ok(res) => res,
        Err(_) => TaskRepo::new(),
    };

    // Testing
    // repo.add_task(
    //     "Second!".to_string(),
    //     "Am I able to save to JSON?".to_string(),
    //     vec!["test".to_string(), "persists".to_string()],
    // );

    // match repo.complete_task(1) {
    //     Ok(_) => println!("Task marked as done!"),
    //     Err(e) => println!("Error: {}", e),
    // }

    if let Some(task) = repo.list_tasks().get(0) {
        println!("{:?}", task);
    }

    if let Some(task) = repo.list_tasks().get(1) {
        println!("{:?}", task);
    }

    // Persist to file
    if let Err(e) = repo.save_to_path(path) {
        eprint!("Failed to save tasks: {}", e);
    } else {
        println!("Saved tasks to {}", path);
    }
}
