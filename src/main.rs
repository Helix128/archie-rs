use clap::{Parser, Subcommand};
use std::env;

mod task;
use colored::Colorize;
use task::TaskCommands;

mod system;
use system::SystemCommands;

mod ui;

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
    About,
}

fn about() {
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    println!(
        "{} {} by {}",
        "archie-rs".cyan().bold(),
        version,
        author.purple()
    );
    let homepage = env!("CARGO_PKG_HOMEPAGE");
    if !homepage.is_empty() {
        println!("{}: {}", "Source".bold(), homepage.blue().underline());
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Task(task_command) => match task_command {
            TaskCommands::Set { name, commands } => {
                if let Err(e) = task::set_task(name, commands) {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                }
            }
            TaskCommands::Delete { name } => {
                if let Err(e) = task::delete_task(name) {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                }
            }
            TaskCommands::List => {
                if let Err(e) = task::list_tasks() {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                }
            }
            TaskCommands::Run { name } => {
                if let Err(e) = task::run_task(name) {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                }
            }
            TaskCommands::Locate => match task::locate_tasks() {
                Ok(path) => println!("{} {}", "Tasks file:".bold(), path.cyan()),
                Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
            },
        },
        Command::Pls { task } => {
            if let Err(e) = task::run_task(task) {
                eprintln!("{} {}", "Error:".red().bold(), e);
            }
        }
        Command::System(system_command) => match system_command {
            SystemCommands::Partitions => {
                system::list_partitions();
            }
        },
        Command::About => {
            about();
        }
    }
}
