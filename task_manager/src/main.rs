use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    name: String,
    description: String,
    tags: HashSet<String>,
    done: bool,
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
            done: false,
        };
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
    }

    fn remove_task(&mut self, task_id: u32) -> Result<(), Error> {
        if self.tasks.remove(&task_id).is_some() {
            Ok(())
        } else {
            Err(Error::new(std::io::ErrorKind::NotFound, "Task not found!"))
        }
    }

    fn list_tasks(&mut self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn filter_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.tags.contains(tag))
            .collect()
    }
}
