use clap::{Parser, Subcommand, ValueEnum};
use prelude::*;
use std::path::PathBuf;

mod models;
mod repo;

mod prelude {
    pub use crate::models::*;
    pub use crate::repo::*;
    pub use std::collections::{HashMap, HashSet};
    pub use std::io::Error;
}

/// CLI task status value enum.
#[derive(Copy, Clone, Debug, ValueEnum)]
enum StatusArg {
    Upcoming,
    InProgress,
    Complete,
    StandBy,
}

impl From<StatusArg> for Status {
    /// Convert from CLI task status to domain task status enum.
    fn from(s: StatusArg) -> Self {
        match s {
            StatusArg::Upcoming => Status::Upcoming,
            StatusArg::InProgress => Status::InProgress,
            StatusArg::Complete => Status::Complete,
            StatusArg::StandBy => Status::StandBy,
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "tasks", version, about = "A tiny task tracker")]
struct Cli {
    /// Path to the data file (JSON)
    #[arg(short = 'f', long = "file", default_value = "tasks.json")]
    data: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task
    Add {
        /// Name/title of the task.
        #[arg(short = 'n', long)]
        name: String,

        /// Description of the task.
        #[arg(short = 'd', long, default_value = "")]
        description: String,

        /// Tags (repeat: -t a -t b) or comma-separarted: -t a,b
        #[arg(short = 't', long = "tag", value_delimiter = ',')]
        tags: Vec<String>,

        /// Optional initial status
        #[arg(long)]
        status: Option<StatusArg>,
    },
    List {},
    Show {},
    Complete {},
    Update {},
    Remove {},
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
