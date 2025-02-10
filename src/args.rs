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
  /// Initialises and sets current Tracker database at a given path 
  /// Required to do once before using Tracker
  Init {
    tracker: String,
  },
  /// Edit tasks in default text editor $EDITOR
  Edit,
  /// Lists all tasks in a given tracker
  List,
  /// Clears all tasks in a given tracker
  /// Allows specifying which tracker to be cleared
  Clear {
    tracker: Option<String>,
  },
  /// Queries a specific task based on tracker ID
  Query {
    id: String,
  },
  /// Prints current tracker name and id
  Info,
  /// Adds current directory to the tracker
  /// If there is no hooked directory, then the marked directory becomes the next jump location
  Mark {
    name: Option<String>,
  },
  /// Jumps to a marked directory
  /// By default tracker will jump to the hooked directory
  /// If there is no hooked directory, then jump to last marked directory
  Jump {
    name: Option<String>,
  },
  /// By default, sets default jump directory regardless of any newly marked directories
  Hook,
  ListTrackers,
}
