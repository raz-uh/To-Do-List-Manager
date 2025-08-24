# To-Do List Manager

This project is a simple and visually appealing To-Do List Manager application built in Rust using a modern GUI (powered by [eframe/egui](https://github.com/emilk/egui)). It allows users to create, update, complete, and view tasks with a persistent task list saved to disk.

## Project Structure

```
to-do-list-manager
├── src
│   ├── main.rs        # Entry point of the application
│   ├── tasks.rs       # Defines the Task struct and serialization
│   └── gui.rs         # GUI logic and persistent storage
├── Cargo.toml         # Rust package configuration
└── README.md          # Project documentation
```

## Features

- Modern, attractive GUI with a purple theme
- Add, edit, complete, and view tasks
- Tabs for Pending and Completed tasks
- Tasks are saved to `tasks.json` and persist between runs

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (install with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

### Running the Application

1. Clone the repository:
   ```
   git clone <repository-url>
   cd to-do-list-manager
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the application:
   ```
   cargo run
   ```

### Usage

- Type a task and press **Enter** or click **Add Task** to add it.
- Switch between **Pending** and **Completed** tabs.
- Click **Complete/Uncomplete** to toggle task status.
- Click **Edit** to modify a task.
- All tasks are automatically saved to `tasks.json` in the project directory.

## Contributing

Feel free to submit issues or pull requests if you have suggestions or improvements for the project.

---
**Enjoy managing your tasks!**