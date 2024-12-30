use clap::{ command, Parser, Subcommand };

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
  #[command(subcommand)]
  pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Initialises a Tracker database at a given path 
  /// Required to do once before using Tracker
  Init {
    path: String,
  },
  /// Edit tasks in default text editor $EDITOR
  Edit,
  /// Lists all tasks in a given tracker
  List,
  /// Clears all tasks in a given tracker
  Clear {
    file_path: Option<String>,
  },
  /// Queries a specific task based on ID
  Query {
    id: String,
  },
}
