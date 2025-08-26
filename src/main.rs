use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::data::{App, TodoItem, TodoList};

mod data;
mod cli;
mod storage;
mod app;

fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();
    let mut app = App::load();
    match cli.command {
        Commands::Add { description } => {
            println!("Add: {}", description);
        }
        Commands::List => {
            println!("List all tasks");
            app.list_all_lists();
        }
        Commands::Done { id } => {
            println!("Mark task {} as done", id);
        }
    }
    
    
}
