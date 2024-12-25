use clap::{command, Parser, Subcommand};
use rusqlite::{Connection};
use edit::{self, edit_file};
use std::{
  env::var,
  fs::File,
  io::Read,
  process::Command,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value="tasks.txt")]
  path: String,

  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  /// Edit tasks in default text editor $EDITOR
  Edit {
    file_path: String,
  },
  /// Lists all tasks in a given tracker
  List {
    file_path: String,
  },
  /// Clears all tasks in a given tracker
  Clear {
    file_path: String,
  },
  /// Queries a specific task based on ID
  Query {
    id: String,
  },
}

fn main() {
  let args = Args::parse();
  let path = &args.path;

  match &args.command {
    Some(Commands::Edit { file_path }) => {
      drop(file_path);
      edit_tasks(file_path);
    },

    Some(Commands::List { file_path }) => {
      println!("List");
    },

    Some(Commands::Clear { file_path }) => {
      println!("Clear");
    },

    Some(Commands::Query { id }) => {
      println!("Query");
    },

    None => {
      edit_tasks(path);
    },
  }
}

fn edit_tasks(path: &String) {
  println!("Editing: {}", &path);
  let edited = edit::edit_file(path).expect("Unable to edit file");
}
