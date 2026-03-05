use std::env;
use clap::{Parser, Subcommand};

mod task;
use colored::Colorize;
use task::TaskCommands;

mod system;
use system::SystemCommands;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(subcommand, name = "task", about = "Manage tasks.")]
    Task(TaskCommands),

    #[clap(name = "pls", about = "Shortcut for 'task run'.")]
    Pls { task: String },

    #[clap(subcommand, name = "system", about = "View system info.")]
    System(SystemCommands),

    #[clap(name = "about", about = "About archie.")]
    About
}

fn about() {
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    println!("{} {}, made with love by {}","archie-rs".cyan().bold(), version, author.purple());
    let homepage = env!("CARGO_PKG_HOMEPAGE");
    if !homepage.is_empty() {
        println!("{}: {}","Source".bold(), homepage.blue().underline());
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Task(task_command) => {
            match task_command {
                TaskCommands::Set { name, commands } => {
                    task::set_task(name, commands);
                }
                TaskCommands::Delete { name } => {
                    task::delete_task(name);
                }
                TaskCommands::List => {
                    task::list_tasks();
                }
                TaskCommands::Run { name } => {
                    task::run_task(name);
                }
                TaskCommands::Locate => {
                    match task::locate_tasks() {
                        Ok(path) => println!("Tasks file location: {}", path),
                        Err(e) => eprintln!("Error locating tasks file: {}", e),
                    }
                }
            }
        }
        Command::Pls { task } => {
                task::run_task(task);
            }
        Command::System(system_command) => {
            match system_command {
                SystemCommands::Disks => {
                    system::list_disks();
                }
            }
        }
        Command::About => {
            about();
        }
    }
}
