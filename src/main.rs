use std::{ fs, env, path::Path };
use anyhow::Ok;
use clap::Parser;
use dirs::home_dir;
use sqlite_interface::Special;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use serde_derive::{ Deserialize, Serialize };
use anyhow::Result;
use toml::from_str;

mod args;
mod sqlite_interface;
use args::Commands;
use args::Args;
use sqlite_interface::Project;

#[derive(Deserialize, Serialize)]
struct Data {
  location: String,
}

#[tokio::main]
async fn main() -> Result<()> {
  let config_path = home_dir().expect("Unable to find home directory")
    .join("dev/rust/tracker/config/tracker.toml".to_owned());
  let config = fs::read_to_string(config_path).expect("Cannot read file");
  let data: Data = toml::from_str(&config).expect("Cannot convert toml to table");
  let db_path = home_dir().expect("Unable to find home directory")
    .join(data.location)
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
      let projects: Vec<Project> = sqlite_interface::load(&db).await?;
      for project in projects {
        match project.dir {
          Some(dir) => {
            if project.special.is_none() {
              println!("Priority: {} | Name: {} | Description: {} | Directory: {} | Special: None", project.priority, project.name, project.desc, dir);
            } else {
              println!("Priority: {} | Name: {} | Description: {} | Directory: {} | Special: {}", project.priority, project.name, project.desc, dir, project.special.unwrap());
            }
          },
          None => println!("Priority: {} | Name: {} | Description: {} | Directory: None | Special: None", project.priority, project.name, project.desc),
        }
      }
    },

    Some(Commands::Clear { tracker }) => {
      sqlite_interface::clear(&db).await?;
      println!("Cleared projects");
    },

    Some(Commands::Info) => {
      println!("Query");
    },
    
    Some(Commands::Mark { name }) => {
      mark_project(db, name).await?;
    },

    Some(Commands::Marked) => {
      let special = sqlite_interface::query_special(&db).await?;
      if special.is_empty() {
        println!("No marked projects found");
      } else {
        for project in special {
          println!("{} | {} | {}", project.name, project.dir.unwrap(), project.special.unwrap());
        }
      }
    }

    Some(Commands::Hooked) => {
      let special = sqlite_interface::query_special(&db).await?;
      let mut marked: Option<Project> = None;
      if special.is_empty() {
        println!("No hooked projects found");
      } else {
        for project in special {
          if project.special.to_owned().unwrap() == *"HOOKED" {
            println!("{} | {} | {}", project.name, project.dir.unwrap(), project.special.unwrap());
          } else if project.special.to_owned().unwrap() == *"MARKED" && marked.is_none() {
            marked = Some(project);
          }
        }
      }
      if marked.is_some() {
        let project: Project = marked.unwrap();
        println!("{} | {} | {}", project.name, project.dir.unwrap(), project.special.unwrap());
      }
    },

    Some(Commands::Hook { name }) => {
      let special = Special::Hook;
      sqlite_interface::update_special(&db, name.to_string(), special).await?;
    },

    Some(Commands::Unhook) => {
      sqlite_interface::unhook(&db).await?;
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
  let mut special_vec: Vec<Option<String>> = Vec::new();
  let mut data: String = String::new();

  data.push_str("# priority, name, description, directory (optional)\n");
  for project in projects {
    special_vec.push(project.special);
    let line = match project.dir {
      Some(dir) => format!("{}, {}, {}, {}\n", project.priority, project.name, project.desc, dir),
      
      None => format!("{}, {}, {}\n", project.priority, project.name, project.desc),
    };

    data.push_str(&line);
  }
  let edited = edit::edit(data).expect("Unable to edit file");

  let mut edited_lines: Vec<&str> = edited.lines().collect();
  edited_lines.remove(0);
  special_vec.reverse();

  let mut edited_tasks: Vec<Project> = vec![]; 
  for line in edited_lines {
    let project: Vec<&str> = line.split(',').collect();
    if project[0].parse::<i32>().is_ok() {
      if project.len() == 3 {
        edited_tasks.push(
          Project {
            priority: project[0].trim().parse::<i32>().unwrap(),
            name: project[1].trim().to_string(),
            desc: project[2].trim().to_string(),
            dir: None,
            special: None,
          }
        )
      } else if project.len() == 4 {
        let special_opt = special_vec.pop().unwrap_or(None);
        edited_tasks.push(
          Project {
            priority: project[0].trim().parse::<i32>().unwrap(),
            name: project[1].trim().to_string(),
            desc: project[2].trim().to_string(),
            dir: Some(project[3].trim().to_string()),
            special: special_opt,
          }
        )
      }
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

  // Checking if the specific project is marked
  if name.is_none() {
    let collection = cwd
      .split('/')
      .collect::<Vec<&str>>();
    sqlite_interface::add(&db, 1, collection[collection.len() - 1].to_string(), "Marked Directory".to_owned(), cwd.clone()).await?;
    sqlite_interface::update_special(&db, collection[collection.len() - 1].to_string(), Special::Mark).await?;
  } else {
    sqlite_interface::update_directory(&db, name.to_owned().unwrap(), cwd).await?;
    sqlite_interface::update_special(&db, name.to_owned().unwrap(), Special::Mark).await?;
  }

  Ok(())
}
