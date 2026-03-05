use clap::Subcommand;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

const TASKS_FILE: &str = "tasks.json";

#[derive(Subcommand)]
pub enum TaskCommands {
    #[clap(name = "set", about = "Update an existing task or create a new one.")]
    Set { name: String, commands: Vec<String> },
    #[clap(name = "delete", about = "Delete an existing task.")]
    Delete { name: String },
    #[clap(name = "list", about = "List all tasks.")]
    List,
    #[clap(name = "run", about = "Run a task.")]
    Run { name: String },
    #[clap(name = "locate", about = "Show the location of the tasks file.")]
    Locate,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    name: String,
    commands: Vec<String>,
}

pub fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("tasks.json");

    match data {
        Ok(content) => {
            let tasks = serde_json::from_str(&content)?;
            Ok(tasks)
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                let empty_tasks = Vec::new();
                save_tasks(&empty_tasks)?;
                Ok(empty_tasks)
            } else {
                Err(Box::new(e))
            }
        }
    }
}

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write("tasks.json", data)?;
    Ok(())
}

pub fn set_task(name: String, command_str: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = load_tasks()?;

    if tasks.iter().any(|task| task.name == name) {
        println!("{} {}", "Task already exists:".yellow(), name.bold());
        println!("Do you want to overwrite it? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Task overwrite cancelled.");
            return Ok(());
        }
        tasks.retain(|task| task.name != name);
    }
    let commands = command_str
        .into_iter()
        .map(|s| s.trim().to_string())
        .collect();
    let task = Task { name, commands };
    println!(
        "Task {} ({}) created {}.",
        task.name.bold(),
        task.commands.join(" && ").cyan().bold(),
        "successfully".green()
    );
    tasks.push(task);
    save_tasks(&tasks)?;

    Ok(())
}

pub fn delete_task(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = load_tasks()?;
    tasks.retain(|task| task.name != name);
    save_tasks(&tasks)?;
    println!("Task {} deleted successfully.", name);
    Ok(())
}

pub fn list_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = load_tasks()?;

    if tasks.is_empty() {
        println!(
            "{} Use {} to create a task.",
            "No tasks found.".yellow(),
            "task set".bold()
        );
        return Ok(());
    }

    println!("{}:", "Tasks".bold().underline());
    tasks.sort_by(|a, b| a.name.cmp(&b.name));
    for task in tasks {
        println!("{}:", task.name.bold());
        for command in task.commands {
            println!("  - {}", command.cyan().bold());
        }
    }
    Ok(())
}

pub fn run_task(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = load_tasks()?;
    match tasks.into_iter().find(|task| task.name == name) {
        Some(task) => execute_task(&task)?,
        None => return Err(format!("Task '{}' not found.", name).into()),
    }
    Ok(())
}

pub fn execute_task(task: &Task) -> std::io::Result<()> {
    for command in &task.commands {
        println!("{} {}", "Executing:".blue(), command.cyan().bold());
        execute_command(command)?;
    }
    Ok(())
}

fn execute_command(cmd_str: &str) -> std::io::Result<()> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", cmd_str]).output()?
    } else {
        Command::new("sh").args(["-c", cmd_str]).output()?
    };

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

pub fn locate_tasks() -> Result<String, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let task_file_path = current_dir.join(TASKS_FILE);
    Ok(task_file_path.to_string_lossy().to_string())
}
