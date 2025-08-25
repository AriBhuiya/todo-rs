use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::data::{Lists, TodoItem, TodoList};

mod data;
mod cli;
mod storage;

fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { description } => {
            println!("Add: {}", description);
        }
        Commands::List => {
            println!("List all tasks");
        }
        Commands::Done { id } => {
            println!("Mark task {} as done", id);
        }
    }
    
    

    let mList = TodoList::new("mList".to_string());
    let item1 = TodoItem::new("item1".to_string(), "testItem".to_string());
    let item2 = TodoItem::new("item2".to_string(), "testItem2".to_string());
    let item3 = TodoItem::new("item3".to_string(), "testItem3".to_string());


    let json = serde_json::to_string_pretty(&mList).unwrap();
    println!("{}", json);
    
    
}
