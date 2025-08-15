use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

fn main() {
    // Testing
    let mut repo: TaskRepo = TaskRepo::new();
    repo.add_task(
        "Hello World!".to_string(),
        "Testing 123..".to_string(),
        vec!["test".to_string(), "note".to_string()],
    );
    let complete: Result<(), Error> = repo.complete_task(1);
    if complete.unwrap() == () {
        println!("Task marked as done!");
    }
    let tasks: Vec<&Task> = repo.list_tasks();
    if let Some(task) = tasks.first() {
        println!("{:?}", task);
    }
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
    fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    /// Reads all tasks with a particular tag.
    fn filter_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.tags.contains(tag))
            .collect()
    }

    /// Updates a task's name, description, and tags via id.
    fn update_task(
        &mut self,
        task_id: u32,
        new_name: Option<String>,
        new_description: Option<String>,
        new_tags: Option<Vec<String>>,
    ) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            if let Some(name) = new_name {
                task.name = name;
            }
            if let Some(description) = new_description {
                task.description = description;
            }
            if let Some(tags) = new_tags {
                task.tags = tags.into_iter().collect();
            }
        }
    }

    /// Updates a task's done field to true.
    fn complete_task(&mut self, task_id: u32) -> Result<(), Error> {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.done == true;
            Ok(())
        } else {
            Err(Error::new(std::io::ErrorKind::NotFound, "Task not found!"))
        }
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
