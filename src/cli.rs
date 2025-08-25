use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo app", version = "0.1")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        description: String,
    },
    List,
    Done {
        id: usize,
    },
}
