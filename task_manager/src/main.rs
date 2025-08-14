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

    fn add_task(&mut self, name: String, description: String, tags: Vec<String>) {
        let task = Task {
            id: self.next_id,
            name,
            description,
            tags: tags.into_iter().collect(),
        };
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
    }

    fn list_tasks(&mut self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
}
