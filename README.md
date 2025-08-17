# Rust Mini Projects

## 1) Tiny Task Manager

---

A minimal CLI task manager written in Rust.

This is my first Rust project built solo (docs + a local `gpt-oss-20b` LLM with coding mentor system prompt). If you’re experienced with Rust and have tips, please open an issue! Feedback welcome!

```bash
task_manager --help
```

### Features

| Feature    | What It Does                                                                                                                      |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Add        | Create a new task with name, description, and optional tags.                                                                      |
| List       | Show all tasks sorted by ID (Upcoming -> Complete).                                                                               |
| Update     | Change any field of an existing task by its ID.                                                                                   |
| Complete   | Mark a task as finished.                                                                                                          |
| Delete     | Remove a task from the list.                                                                                                      |
| Persist    | Store the whole repository do JSON (`tasks.json`) with [Serde](https://github.com/serde-rs/serde) and load it back automatically. |
| CLI        | Built with [Clap](https://github.com/clap-rs/clap) for subcommands, flags, and arguments.                                         |
| Unit Tests | Covered with Rust's built-in test framework (`cargo test`).                                                                       |

### Install

```bash
# ensure you have rust and cargo installed

# clone the repository
git clone https://github.com/talonlikeaclaw/rust-mini-projects.git
cd task_manager

# dev (build + run)
cargo run -- --help

# release binary
cargo build --release
./target/release/task_manager --help

# or install to PATH
cargo install --path .
task_manager --help
```

### Usage

| Subcommand | Example                                                                       | Description                         |
| ---------- | ----------------------------------------------------------------------------- | ----------------------------------- |
| add        | `task_manager add -n "Write unit tests" -d "For task repo" -t coding,project` | Create a task.                      |
| list       | `task_manager list`                                                           | Lists all tasks in ID order.        |
| update     | `task_manager update 1 --name "Read book" --status complete`                  | Update fields of task #1.           |
| complete   | `task_manager complete 2`                                                     | Mark task #2 as Complete.           |
| remove     | `task_manager remove 3`                                                       | Delete task #3 from the repository. |

```bash
# add tasks
task_manager add -n "Write unit tests" -d "For task repo" -t coding,project
task_manager add -n "Buy milk" -d "2% please" -t errands -t groceries --status in-progress

# list (table by default)
task_manager list
task_manager list --status complete
task_manager list --tag groceries

# json output
task_manager list --json
task_manager show 1 --json

# update / complete / remove
task_manager update 1 --name "Read book" --status complete -t reading
task_manager complete 2
task_manager remove 3

```

### Persist

By default the program writes to `./tasks.json`.

You can override it with `-f <path>` or `--file <path>`:

```bash
task_manager add "Plan trip" -f ~/my_trip_tasks.json
```

The file is JSON-encoded via Serde. You can inspect or edit it manually if needed.
