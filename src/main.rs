use clap::{command, Parser, Subcommand};
use edit::{self, edit_file};
use std::fs;
use std::fs::{OpenOptions, File};
use std::io::{self, prelude::*, BufReader, Read};

mod args;
mod db_interface;

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
      let contents = fs::read_to_string(&path)
        .expect("Cannot find file");
      println!("{}", contents);
      parse_tasks(path);
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

fn parse_tasks(path: &String) -> io::Result<()> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let string = line?;
    let vec: Vec<char> = string.clone()
      .chars()
      .collect();    

    if vec[0] != '#' {
      let sub_str = string.split("|");
      for c in sub_str {
        println!("{}", c.trim());
      }
    }
  }

  Ok(())
}
