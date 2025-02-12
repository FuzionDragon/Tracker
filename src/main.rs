use std::{ fs, env };
use anyhow::Ok;
use clap::Parser;
use dirs::home_dir;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use serde_derive::{ Deserialize, Serialize };
use anyhow::Result;
use toml;

mod args;
mod sqlite_interface;
use args::Commands;
use args::Args;
use sqlite_interface::Project;

#[derive(Deserialize, Serialize)]
struct Data {
  config: Config,
}

#[derive(Deserialize, Serialize)]
struct Config {
  location: String,
}

const DB_URL: &str = "sqlite://tracker.db";

#[tokio::main]
async fn main() -> Result<()> {
  let config_path = home_dir().expect("Unable to find home directory")
    .join("dev/rust/tracker/config/tracker.toml".to_owned());
  let config = fs::read_to_string(config_path).expect("Cannot read file");
  let data: Data = toml::from_str(&config).expect("Cannot convert toml to table");
  let db_path = home_dir().expect("Unable to find home directory")
    .join(data.config.location)
    .into_os_string()
    .into_string()
    .unwrap();

  if !Sqlite::database_exists(&db_path).await.unwrap_or(false) {
    println!("Creating database: {}", &db_path);
    Sqlite::create_database(&db_path).await?;
  }

  let db = SqlitePool::connect(&db_path).await.unwrap();

  sqlite_interface::init(&db, "projects".to_string()).await?;

  let args = Args::parse();
  match &args.command {
    Some(Commands::Edit) => {
      edit_projects(db).await?;
    },

    Some(Commands::List) => {
      println!("Listing Database");
      let projects: Vec<Project> = sqlite_interface::load(&db).await?;
      for project in projects {
        match project.dir {
          Some(dir) => println!("Priority: {} | Name: {} | Description: {} | Directory: {}", project.priority, project.name, project.desc, dir),
          None => println!("Priority: {} | Name: {} | Description: {} | Directory: None", project.priority, project.name, project.desc),
        }
      }
    },

    Some(Commands::Clear { tracker }) => {
      sqlite_interface::clear(&db).await?;
      println!("Cleared projects");
    },

    Some(Commands::Query { id }) => {
      println!("Query");
    },

    Some(Commands::Info) => {
      println!("Query");
    },
    
    Some(Commands::Update { new_dir, name }) => {
      sqlite_interface::update(&db, name.to_string(), new_dir.to_string()).await?;
      println!("Updated project dir");
    },

    Some(Commands::Mark { name }) => {
      mark_project(db, name).await?;
      println!("Marked Directory");
    },

    Some(Commands::Jump { name }) => {
      println!("Query");
    },

    Some(Commands::Hook) => {
      println!("Query");
    },
    
    Some(Commands::ListTrackers) => {
      println!("Query");
    },

    none => {
      edit_projects(db).await?;
    },
  }

  Ok(())
}

async fn edit_projects(db: SqlitePool) -> Result<()> {
  println!("Editing project");
  let projects: Vec<Project> = sqlite_interface::load(&db).await?;
  let mut data: String = String::new();

  data.push_str("# priority, name, description, directory (optional)\n");
  for project in projects {
    let line = match project.dir {
      Some(dir) => format!("{}, {}, {}, {}\n", project.priority, project.name, project.desc, dir),
      
      None => format!("{}, {}, {}\n", project.priority, project.name, project.desc),
    };

    data.push_str(&line);
  }
  let edited = edit::edit(data).expect("Unable to edit file");

  let mut edited_lines: Vec<&str> = edited.lines().collect();
  edited_lines.remove(0);

  let mut edited_tasks: Vec<Project> = vec![]; 
  for line in edited_lines {
    let project: Vec<&str> = line.split(',').collect();
    if project.len() == 3 {
      if project[0].parse::<i32>().is_ok() {
        edited_tasks.push(
          Project {
            priority: project[0].trim().parse::<i32>().unwrap(),
            name: project[1].trim().to_string(),
            desc: project[2].trim().to_string(),
            dir: None,
          }
        )
      }
    } else if project.len() == 4 && project[0].parse::<i32>().is_ok() {
      edited_tasks.push(
        Project {
          priority: project[0].trim().parse::<i32>().unwrap(),
          name: project[1].trim().to_string(),
          desc: project[2].trim().to_string(),
          dir: Some(project[3].trim().to_string()),
        }
      )
    }
  }
  sqlite_interface::overwrite(&db, edited_tasks).await?;

  Ok(())
}

async fn mark_project(db: SqlitePool, name: &Option<String>) -> Result<()> {
  let cwd = env::current_dir()?
    .into_os_string()
    .into_string()
    .unwrap();

  if name.is_none() {
    let collection = cwd
      .split('/')
      .collect::<Vec<&str>>();
    sqlite_interface::add(&db, 1, collection[collection.len() - 1].to_string(), "Marked Directory".to_owned(), cwd).await?;
  } else {
    sqlite_interface::update(&db, name.to_owned().unwrap(), cwd).await?;
  }

  Ok(())
}

async fn jump_project(db: SqlitePool, name: &Option<String>) -> Result<()> {
  Ok(())
}

async fn hook_project(db: SqlitePool, name: &Option<String>) -> Result<()> {
  Ok(())
}
