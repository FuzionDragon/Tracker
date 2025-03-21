use std::{ fs, fs::File, env, collections::HashMap };
use anyhow::Ok;
use clap::Parser;
use dirs::data_local_dir;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use serde_derive::{ Deserialize, Serialize };
use anyhow::Result;

mod args;
use args::Commands;
use args::Args;
use tracker::sqlite_interface;
use tracker::sqlite_interface::*;

#[tokio::main]
async fn main() -> Result<()> {
  // Accessing local share directory on the system
  // Creating a tracker directory and database if needed
  let tracker_dir = data_local_dir().expect("Unable to find local share directory")
    .join("tracker");
  if !fs::exists(&tracker_dir).unwrap() {
    println!("{:?}", &tracker_dir);
    println!("Tracker directory path doesn't exist, creating directory ~/.local/share/tracker");
    fs::create_dir_all(&tracker_dir).expect("Unable to create tracker directory");
  }
  let db_path = tracker_dir
    .join("tracker.db")
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
        if let Some(dir) = project.dir {
          if let Some(special) = project.special {
            println!("Priority: {} | Name: {} | Description: {} | Directory: {} | Special: {}", project.priority, project.name, project.desc, dir, special);
          } else {
            println!("Priority: {} | Name: {} | Description: {} | Directory: {} | Special: None", project.priority, project.name, project.desc, dir);
          }
        } else {
          println!("Priority: {} | Name: {} | Description: {} | Directory: None | Special: None", project.priority, project.name, project.desc);
        }
      }
    },

    Some(Commands::Dirs) => {
      let projects: Vec<Project> = sqlite_interface::load(&db).await?;
      for project in projects {
        if let Some(dir) = project.dir {
          if let Some(special) = project.special {
            println!("Priority: {} | Name: {} | Description: {} | Directory: {} | Special: {}", project.priority, project.name, project.desc, dir, special);
          } else {
            println!("Priority: {} | Name: {} | Description: {} | Directory: {} | Special: None", project.priority, project.name, project.desc, dir);
          }
        }
      }
    },

    Some(Commands::Clear) => {
      sqlite_interface::clear(&db).await?;
      println!("Cleared projects");
    },

    Some(Commands::Mark { name }) => {
      mark_project(db, name).await?;
    },

    Some(Commands::Special) => {
      let special = sqlite_interface::query_special(&db).await?;
      for project in special {
        println!("{} | {} | {}", project.name, project.dir.unwrap(), project.special.unwrap());
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
      hook_project(db, name).await?;
    },

    Some(Commands::Unhook) => {
      sqlite_interface::unhook(&db).await?;
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
  let mut special_map: HashMap<String, Special> = HashMap::new();
  let mut data: String = String::new();

  data.push_str("# priority, name, description, directory (optional)\n");
  for project in projects {
    if project.special.is_some() {
      if project.special.clone().unwrap() == *"MARKED" {
        special_map.insert(project.name.clone(), Special::Mark);
      }
      if project.special.unwrap() == *"HOOKED" {
        special_map.insert(project.name.clone(), Special::Hook);
      }
    }
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
        // needs to set special_opt to a none or a special enum depending on if it exists in the
        // hashmap
        let key = project[1].trim().to_string();
        let special_opt: Option<String> = if special_map.contains_key(&key) {
          match special_map[&key] {
            Special::Hook => Some("HOOKED".to_string()),

            Special::Mark => Some("MARKED".to_string()),
          }
        } else {
          None
        };

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
    sqlite_interface::add(&db, 1, collection[collection.len() - 1].to_string(), "Marked Directory".to_owned(), cwd.clone(), Special::Mark).await?;
    sqlite_interface::update_special(&db, collection[collection.len() - 1].to_string(), Special::Mark).await?;
  } else {
    sqlite_interface::update_directory(&db, name.to_owned().unwrap(), cwd).await?;
    sqlite_interface::update_special(&db, name.to_owned().unwrap(), Special::Mark).await?;
  }

  Ok(())
}

async fn hook_project(db: SqlitePool, name: &Option<String>) -> Result<()> {
  let cwd = env::current_dir()?
    .into_os_string()
    .into_string()
    .unwrap();

  // Checking if the specific project is marked or hooked
  if name.is_none() {
    let collection = cwd
      .split('/')
      .collect::<Vec<&str>>();
    sqlite_interface::add(&db, 1, collection[collection.len() - 1].to_string(), "Hooked Directory".to_owned(), cwd.clone(), Special::Hook).await?;
    sqlite_interface::update_special(&db, collection[collection.len() - 1].to_string(), Special::Hook).await?;
  } else {
    sqlite_interface::update_directory(&db, name.to_owned().unwrap(), cwd).await?;
    sqlite_interface::update_special(&db, name.to_owned().unwrap(), Special::Hook).await?;
  }

  Ok(())
}
