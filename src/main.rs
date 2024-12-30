use clap::{command, Parser, Subcommand};
use edit::{self, edit_file};
use std::fs;
use std::fs::{OpenOptions, File};
use std::io::{self, prelude::*, BufReader, Read};
use serde_derive::Deserialize;
use dirs::home_dir;
use toml;

mod args;
mod db_interface;

use args::Commands;
use args::Args;

#[derive(Deserialize)]
struct Data {
  config: Config,
}

#[derive(Deserialize)]
struct Config {
  path: String,
}

fn main() {
  let config_path = "dev/rust/tracker/config/tracker.toml".to_owned();
  let full_path = home_dir()
    .expect("Unable to find home directory")
    .join(config_path);
  let config = fs::read_to_string(full_path).expect("Cannot read file");
  let data: Data = toml::from_str(&config).expect("Cannot convert toml to table");
  let path = data.config.path;
  let args = Args::parse();

  println!("{}", &path);

  match &args.command {
    Some(Commands::Edit) => {
      edit_tasks(&path);
    },

    Some(Commands::List) => {
      parse_tasks(&path);
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
      edit_tasks(&path);
    },
  }
}

fn edit_tasks(path: &String) {
  println!("Editing: {}", &path);
  let full_path = home_dir()
    .expect("Unable to find home directory")
    .join(path);
  let edited = edit::edit_file(full_path)
    .expect("Unable to edit file");
}

fn parse_tasks(path: &String) -> io::Result<()> {
  let full_path = home_dir()
    .expect("Unable to find home directory")
    .join(path);
  let file = File::open(full_path)?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let string = line?;
    println!("{}", &string);
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
