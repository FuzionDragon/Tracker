use std::fs;
use clap::Parser;
use edit;
use dirs::home_dir;
use rusqlite::{Connection, Result};
use serde_derive::Deserialize;
use sqlite_interface::init;
use toml;

mod args;
mod sqlite_interface;
use args::Commands;
use args::Args;
use sqlite_interface::{overwrite, Task};

#[derive(Deserialize)]
struct Data {
  default_config: Config,
}

#[derive(Deserialize)]
struct Config {
  path: String,
  database: String,
}

fn main() -> Result<(), rusqlite::Error> {
  let config_path = home_dir().expect("Unable to find home directory")
    .join("dev/rust/tracker/config/tracker.toml".to_owned());
  let config = fs::read_to_string(config_path).expect("Cannot read file");
  let data: Data = toml::from_str(&config).expect("Cannot convert toml to table");
  let db_path = home_dir().expect("Unable to find home directory")
    .join(data.default_config.path);
  let conn = Connection::open(db_path)?;

  init(&conn)?;
  let args = Args::parse();
  match &args.command {
    Some(Commands::Edit) => {
      edit_tasks(&conn)?;
    },

    Some(Commands::List) => {
      sqlite_interface::print(&conn)?;
    },

    Some(Commands::Clear { tracker }) => {
      sqlite_interface::clear(&conn)?;
      println!("Cleared tasks");
    },

    Some(Commands::Query { id }) => {
      println!("Query");
    },

    Some(Commands::Info) => {
      println!("Query");
    },
    
    Some(Commands::Change) => {
      println!("Query");
    },
    
    Some(Commands::ListTrackers) => {
      println!("Query");
    },

    none => {
      let _ = edit_tasks(&conn);
    },
  }

  Ok(())
}

fn edit_tasks(conn: &Connection) -> Result<(), rusqlite::Error> {
  println!("Editing task");
  let tasks: Vec<Task> = sqlite_interface::load(&conn)?;
  let mut data: String = String::new();

  data.push_str("# priority, name, description");
  for task in tasks {
    let line = format!("{}, {}, {}\n", task.priority, task.name, task.desc);
    data.push_str(&line);
  }
  let edited = edit::edit(data).expect("Unable to edit file");

  let mut edited_lines: Vec<&str> = edited.lines().collect();
  edited_lines.remove(0);

  let mut edited_tasks: Vec<Task> = vec![]; 
  for line in edited_lines {
    let task: Vec<&str> = line.split(',').collect();
    println!("{}: {}, {}", &task[0], &task[1], &task[2]);
    edited_tasks.push(
      Task {
        priority: task[0].trim().parse::<i32>().unwrap(),
        name: task[1].trim().to_string(),
        desc: task[2].trim().to_string(),
      }
    )
  }
  overwrite(&conn, edited_tasks)?;

  Ok(())
}
