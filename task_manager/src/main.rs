use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Task {
    id: u32,
    name: String,
    description: String,
    tags: HashSet<String>,
}

struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }
}
