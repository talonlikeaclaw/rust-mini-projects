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

    /// List tasks (optionally filter by status and/or tags)
    List {
        /// The status to filter by.
        #[arg(long)]
        status: Option<StatusArg>,

        /// The tag to filter by.
        #[arg(short = 't', long = "tag")]
        tag: Option<String>,

        /// Output as JSON instead of table.
        #[arg(long)]
        json: bool,
    },

    /// Show one task by id
    Show {
        /// The id of the task to display.
        id: u32,

        /// Output as JSON instead of table.
        #[arg(long)]
        json: bool,
    },

    /// Mark a task as complete
    Complete {
        /// The id of the task to complete.
        id: u32,
    },

    /// Updates fields of a task
    Update {
        /// The id of the task to update.
        id: u32,

        /// The new name of the task.
        #[arg(long)]
        name: Option<String>,

        /// The new description of the task.
        #[arg(long)]
        description: Option<String>,

        /// The new tags of the task.
        #[arg(short = 't', long = "tag", value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// The new status of the task.
        #[arg(long)]
        status: Option<StatusArg>,
    },

    /// Remove a task by id
    Remove {
        /// The id of the task to remove.
        id: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    // Load rep (or start fresh).
    let mut repo: TaskRepo = match TaskRepo::load_from_path(&cli.data) {
        Ok(r) => r,
        Err(_) => TaskRepo::new(),
    };

    let mut dirty = false;

    match cli.command {
        Commands::Add {
            name,
            description,
            tags,
            status,
        } => {
            let id = repo.next_id;
            repo.add_task(name, description, tags);
            if let Some(s) = status {
                let _ = repo.update_task(id, None, None, None, Some(s.into()));
            }
            println!("Created task #{id}");
            dirty = true;
        }
        Commands::List { status, tag, json } => {}
        Commands::Show { id, json } => {}
        Commands::Complete { id } => {}
        Commands::Update {
            id,
            name,
            description,
            tags,
            status,
        } => {}
        Commands::Remove { id } => {}
    }

    // Save only if task was mutated.
    if dirty {
        if let Err(e) = repo.save_to_path(&cli.data) {
            eprint!("Failed to save {}: {e}", cli.data.display());
        }
    }
}

fn print_table(tasks: &[&Task]) {
    if tasks.is_empty() {
        println!("(no tasks)");
        return;
    }

    println!("{:<4}  {:<12}  {:<10}  {}", "ID", "STATUS", "NAME", "TAGS");

    for t in tasks {
        let mut tags: Vec<&str> = t.tags.iter().map(String::as_str).collect();
        tags.sort_unstable();
        let tags_str = tags.join(",");

        println!(
            "{:<4}  {:<12}  {:<10}  {}",
            t.id,
            fmt_status(&t.status),
            t.name,
            tags_str
        );
    }
}

/// Formats a status enum as a str.
fn fmt_status(s: &Status) -> &'static str {
    match s {
        Status::Upcoming => "Upcoming",
        Status::InProgress => "InProgress",
        Status::Complete => "Complete",
        Status::StandBy => "StandBy",
    }
}
