use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

/// Represents the list of tasks.
#[derive(Serialize, Deserialize)]
pub struct TaskRepo {
    pub tasks: HashMap<u32, Task>,
    pub next_id: u32,
}

impl TaskRepo {
    /// Instantiates a new task repo.
    pub fn new() -> Self {
        TaskRepo {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    /// Creates a new task and adds it to the list of tasks.
    pub fn add_task(&mut self, name: String, description: String, tags: Vec<String>) {
        let task = Task {
            id: self.next_id,
            name,
            description,
            tags: tags.into_iter().collect(),
            status: Status::Upcoming,
        };
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
    }

    /// Reads all tasks in the list of tasks.
    pub fn list_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        tasks.sort_by_key(|task| task.id);
        tasks
    }

    /// Reads all tasks with a particular tag.
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.tags.contains(tag))
            .collect()
    }

    /// Updates a task's name, description, and tags via id.
    pub fn update_task(
        &mut self,
        task_id: u32,
        new_name: Option<String>,
        new_description: Option<String>,
        new_tags: Option<Vec<String>>,
        new_status: Option<Status>,
    ) -> Result<(), Error> {
        let task = self
            .tasks
            .get_mut(&task_id)
            .ok_or_else(|| Error::new(std::io::ErrorKind::NotFound, "Task not found!"))?;

        if let Some(name) = new_name {
            task.name = name;
        }
        if let Some(description) = new_description {
            task.description = description;
        }
        if let Some(tags) = new_tags {
            task.tags = tags.into_iter().collect();
        }
        if let Some(status) = new_status {
            task.status = status;
        }

        Ok(())
    }

    /// Updates a task's done field to true.
    pub fn complete_task(&mut self, task_id: u32) -> Result<(), Error> {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.status = Status::Complete;
            Ok(())
        } else {
            Err(Error::new(std::io::ErrorKind::NotFound, "Task not found!"))
        }
    }

    /// Deletes a task from the task list.
    pub fn remove_task(&mut self, task_id: u32) -> Result<(), Error> {
        if self.tasks.remove(&task_id).is_some() {
            Ok(())
        } else {
            Err(Error::new(std::io::ErrorKind::NotFound, "Task not found!"))
        }
    }

    /// Save repo to JSON at `path`.
    pub fn save_to_path<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let file = File::create(path.as_ref())?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)
            .map_err(|e| Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    /// Load repo from JSON at `path`.
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(file);
        let mut repo: TaskRepo = serde_json::from_reader(reader)
            .map_err(|e| Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        // Ensure next_id is valid even if file was edited.
        if let Some(max_id) = repo.tasks.keys().max() {
            repo.next_id = repo.next_id.max(max_id + 1);
        } else {
            repo.next_id = 1;
        }
        Ok(repo)
    }
}
