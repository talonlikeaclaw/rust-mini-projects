use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

fn main() {
    println!("Hello, world!");
}

/// Represent a singular task.
#[derive(Debug, Clone)]
struct Task {
    id: u32,
    name: String,
    description: String,
    tags: HashSet<String>,
    done: bool,
}

/// Represents the list of tasks.
struct TaskRepo {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskRepo {
    /// Instantiates a new task repo.
    fn new() -> Self {
        TaskRepo {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    /// Creates a new task and adds it to the list of tasks.
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

    /// Reads all tasks in the list of tasks.
    fn list_tasks(&mut self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    /// Reads all tasks with a particular tag.
    fn filter_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.tags.contains(tag))
            .collect()
    }

    /// Deletes a task from the task list.
    fn remove_task(&mut self, task_id: u32) -> Result<(), Error> {
        if self.tasks.remove(&task_id).is_some() {
            Ok(())
        } else {
            Err(Error::new(std::io::ErrorKind::NotFound, "Task not found!"))
        }
    }
}
