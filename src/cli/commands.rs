use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo-list")]
#[command(about = "A simple todo list manager")]
pub struct Cli {

  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {

  Add {

    title: String,
    #[arg(short, long)]
    description: Option<String>,

    #[arg(short, long)]
    priority: Option<String>,
  },

  List {

    #[arg(short, long, default_value = "all")]
    filter: String,

    #[arg(short, long, default_value = "created")]
    sort: String,
  },

  Complete {

    id: String,
  },

  Toggle {

    id: String,
  },

  Update {

    id: String,

    #[arg(short, long)]
    title: Option<String>,

    #[arg(short, long)]
    description: Option<String>,

    #[arg(short, long)]
    priority: Option<String>,
  },

  Delete {

    id: String,
  },

  Clear {

    #[arg(long)]
    all: bool,
  },
}