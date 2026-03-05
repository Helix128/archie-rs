use clap::{Parser, Subcommand};

mod task;
use task::TaskCommands;

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
    Pls { task: String }
}


fn main() {
    let args = Args::parse();

    match args.command {
        Command::Task(task_command) => {
            match task_command {
                TaskCommands::Create { name, commands } => {
                    if let Err(e) = task::create_task(name, commands) {
                        eprintln!("Error creating task: {}", e);
                    }
                }
                TaskCommands::Delete { name } => {
                    if let Err(e) = task::delete_task(name) {
                        eprintln!("Error deleting task: {}", e);
                    }
                }
                TaskCommands::List => {
                    task::list_tasks();
                }
                TaskCommands::Run { name } => {
                    if let Err(e) = task::run_task(name) {
                        eprintln!("Error running task: {}", e);
                    }
                }
            }
        }
        Command::Pls { task } => {
                if let Err(e) = task::run_task(task) {
                    eprintln!("Error running task: {}", e);
                }
            }
    }
}
