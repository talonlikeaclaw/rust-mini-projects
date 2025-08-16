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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_initial_state() {
        // arrange & act
        let repo = TaskRepo::new();
        // assert
        assert_eq!(repo.next_id, 1);
        assert!(repo.tasks.is_empty());
    }

    #[test]
    fn test_add_task_creates_entry() {
        // arrange
        let mut repo = TaskRepo::new();
        // act
        repo.add_task(
            "Buy milk".into(),
            "From IGA".into(),
            vec!["groceries".into()],
        );
        // assert
        assert_eq!(repo.tasks.len(), 1);
        let task = repo.tasks.get(&1).expect("task should exist");
        assert_eq!(task.name, "Buy milk");
        assert_eq!(task.description, "From IGA");
        assert_eq!(task.tags, HashSet::from(["groceries".to_string()]));
        assert_eq!(task.status, Status::Upcoming);
    }

    #[test]
    fn test_list_tasks_return_sorted_by_id() {
        // arrange
        let mut repo = TaskRepo::new();
        repo.add_task("A".into(), "1".into(), vec![]);
        repo.add_task("B".into(), "2".into(), vec![]);
        repo.add_task("C".into(), "3".into(), vec![]);
        // act
        let list = repo.list_tasks();
        // assert
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].id, 1);
        assert_eq!(list[1].id, 2);
        assert_eq!(list[2].id, 3);
    }

    #[test]
    fn test_update_task_all_fields() {
        // arrange
        let mut repo = TaskRepo::new();
        repo.add_task("Old".into(), "desc".into(), vec!["old".into()]);
        // act
        let res = repo.update_task(
            1,
            Some("New".into()),
            Some("changed".into()),
            Some(vec!["new".into()]),
            Some(Status::Complete),
        );
        // assert
        assert!(res.is_ok());
        let t = repo.tasks.get(&1).unwrap();
        assert_eq!(t.name, "New");
        assert_eq!(t.description, "changed");
        assert_eq!(t.tags, HashSet::from(["new".to_string()]));
        assert_eq!(t.status, Status::Complete);
    }

    #[test]
    fn test_update_task_partial() {
        // arrange
        let mut repo = TaskRepo::new();
        repo.add_task("Task".into(), "desc".into(), vec![]);
        // act
        let res = repo.update_task(1, Some("Renamed".into()), None, None, None);
        // assert
        let t = repo.tasks.get(&1).unwrap();
        assert!(res.is_ok());
        assert_eq!(t.name, "Renamed");
        assert_eq!(t.description, "desc");
    }

    #[test]
    fn test_complete_task_success() {
        // arrange
        let mut repo = TaskRepo::new();
        repo.add_task("Todo".into(), "do it".into(), vec![]);
        // act
        let res = repo.complete_task(1);
        // assert
        assert!(res.is_ok());
        let t = repo.tasks.get(&1).unwrap();
        assert_eq!(t.status, Status::Complete);
    }

    #[test]
    fn complete_task_not_found() {
        // arrange
        let mut repo = TaskRepo::new();
        // act
        let err = repo.complete_task(1);
        // assert
        assert_eq!(err.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn test_remove_task_success() {
        // arrange
        let mut repo = TaskRepo::new();
        repo.add_task("Keep".into(), "".into(), vec![]);
        assert_eq!(repo.tasks.len(), 1);
        // act
        let res = repo.remove_task(1);
        // assert
        assert!(res.is_ok());
        assert!(repo.tasks.is_empty());
    }
}
