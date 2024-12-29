use clap::{command, Parser, Subcommand};
use rusqlite::{Connection};
use edit::{self, edit_file};
use std::fs;
use std::fs::OpenOptions;

mod args;

use args::Commands;
use args::Args;

fn main() {
  let args = Args::parse();
  let path = &args.path;

  match &args.command {
    Some(Commands::Edit) => {
      edit_tasks(path);
    },

    Some(Commands::List) => {
      let contents = fs::read_to_string(path)
        .expect("Cannot find file");
      println!("{}", contents);
    },

    Some(Commands::Clear { file_path }) => {
      match file_path {
        Some(file_path) => {
          let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path);
        },
        none => {
          let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path);
        },
      }
      println!("Cleared tasks");
    },

    Some(Commands::Query { id }) => {
      println!("Query");
    },

    none => {
      edit_tasks(path);
    },
  }
}

fn edit_tasks(path: &String) {
  println!("Editing: {}", &path);
  let edited = edit::edit_file(path)
    .expect("Unable to edit file");
}
