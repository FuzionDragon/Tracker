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
  /// Edit tasks in default text editor $EDITOR
  Edit,
  /// Lists all projects 
  List,
  /// Clears all projects 
  Clear,
  /// Adds current project directory to the tracker
  /// If there is no hooked directory, then the marked directory becomes the next jump location
  Mark {
    name: Option<String>,
  },
  /// By default, sets default jump directory regardless of any newly marked directories
  Hook {
    name: Option<String>,
  },
  /// Prints hooked name and directory, if there is no hooked project then it will print the last
  /// marked project.
  /// If there are neither then it will print nothing.
  Hooked,
  /// Unhooks the currently hooked project, otherwise does nothing
  Unhook,
  /// Prints both hooked and marked projects
  Special,
  /// Prints all projects with a non-empty directory field
  Dirs,
}
