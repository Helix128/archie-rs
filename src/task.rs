use std::fs;
use std::process::Command;
use serde::{Serialize, Deserialize};
use clap::Subcommand;
use colored::Colorize;

#[derive(Subcommand)]
pub enum TaskCommands{
    #[clap(name = "create", about = "Create a new task.")]
    Create{
        name: String,
        commands: Vec<String>,
    },
    #[clap(name = "delete", about = "Delete an existing task.")]
    Delete{
        name: String,
    },
    #[clap(name = "list", about = "List all tasks.")]
    List,
    #[clap(name = "run", about = "Run a task.")]
    Run{
        name: String,
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    name: String,
    commands: Vec<String>,
}

pub fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("tasks.json")?;
    let tasks = serde_json::from_str(&data)?;
    Ok(tasks)
}

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write("tasks.json", data)?;
    Ok(())
}

pub fn create_task(name: String, command_str: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = match load_tasks() {
        Ok(tasks) => tasks,
        Err(e) => {
            if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                if io_err.kind() == std::io::ErrorKind::NotFound {
                    Vec::new()
                } else {
                    return Err(e);
                }
            } else {
                return Err(e);
            }
        }
    };

    if tasks.iter().any(|task| task.name == name) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("{} {}", "Task already exists:".yellow(), name.bold())
        )));
    }
    let commands = command_str.into_iter().map(|s| s.trim().to_string()).collect();
    let task = Task { name, commands };
    println!("Task {} ({}) created {}.", task.name.bold(), task.commands.join(" && ").cyan().bold(), "successfully".green());
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

pub fn list_tasks(){
    let mut tasks = match load_tasks() {
        Ok(tasks) => tasks,
        Err(e) => {
            if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                if io_err.kind() == std::io::ErrorKind::NotFound {
                    eprintln!("{} Use {} to create a task.", "No tasks found.".red(), "task create".bold());
                    return;
                }
            }
            eprintln!("Error loading tasks: {}", e);
            return;
        }
    };
    println!("{}:", "Tasks".bold().underline());
    tasks.sort_by(|a, b| a.name.cmp(&b.name));
    for task in tasks {
        println!("{}:", task.name.bold());
        for command in task.commands {
            println!("  - {}", command.cyan().bold());
        }
    }
}

pub fn run_task(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = load_tasks()?;
    let task = tasks.iter().find(|t| t.name == name)
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Task not found: {}", name)
        ))?;
    
    println!("Running task '{}'", task.name.bold());
    for command in &task.commands {
        println!("> {}", command.cyan().bold());
        execute_command(command)?;
    }
    Ok(())
}

fn execute_command(cmd_str: &str) -> std::io::Result<()> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd_str])
            .output()?
    } else {
        Command::new("sh")
            .args(["-c", cmd_str])
            .output()?
    };

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}