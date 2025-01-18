use std::fs;
use clap::Parser;
use edit;
use dirs::home_dir;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use serde_derive::Deserialize;
use anyhow::Result;
use toml;

mod args;
mod sqlite_interface;
use args::Commands;
use args::Args;
use sqlite_interface::Task;

#[derive(Deserialize)]
struct Data {
  default_config: Config,
}

#[derive(Deserialize)]
struct Config {
  database: String,
}

const DB_URL: &str = "sqlite://tracker.db";

#[tokio::main]
async fn main() -> Result<()> {
  let config_path = home_dir().expect("Unable to find home directory")
    .join("dev/rust/tracker/config/tracker.toml".to_owned());
  let config = fs::read_to_string(config_path).expect("Cannot read file");
  let data: Data = toml::from_str(&config).expect("Cannot convert toml to table");

  if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
    println!("Creating database: {}", DB_URL);
    match Sqlite::create_database(DB_URL).await {
       Ok(_) => println!("Create db success"), 
       Err(error) => println!("error: {}", error),
    }
  }

  let db = SqlitePool::connect(DB_URL).await.unwrap();

  sqlite_interface::init(&db, "Name".to_string()).await?;

  let args = Args::parse();
  match &args.command {
    Some(Commands::Edit) => {
      edit_tasks(db).await?;
    },

    Some(Commands::List) => {
      println!("Listing Database");
      let tasks: Vec<Task> = sqlite_interface::load(&db).await?;
      for task in tasks {
        println!("Priority: {} | Name: {} | Description: {}", task.priority, task.name, task.desc);
      }
    },

    Some(Commands::Clear { tracker }) => {
      sqlite_interface::clear(&db).await?;
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
      edit_tasks(db).await?;
    },
  }

  Ok(())
}

async fn edit_tasks(db: SqlitePool) -> Result<()> {
  println!("Editing task");
  let tasks: Vec<Task> = sqlite_interface::load(&db).await?;
  let mut data: String = String::new();
  println!("{:?}", &tasks);

  data.push_str("# priority, name, description\n");
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
//    println!("{:?}", &task);
    edited_tasks.push(
      Task {
        priority: task[0].trim().parse::<i32>().unwrap(),
        name: task[1].trim().to_string(),
        desc: task[2].trim().to_string(),
      }
    )
  }
  sqlite_interface::overwrite(&db, edited_tasks).await?;

  Ok(())
}
