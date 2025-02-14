use anyhow::{ Ok, Result };
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, FromRow, Row, Error::Database};

#[derive(Debug, FromRow, Clone)]
pub struct Project {
  pub priority: i32,
  pub name: String,
  pub desc: String,
  pub dir: Option<String>,
  pub special: Option<String>,
}

pub enum Fields {
  Priority,
  Name,
  Desc,
  Dir,
  Special,
}

pub enum Special {
  Mark,
  Hook,
}

pub async fn init(db: &SqlitePool, name: String) -> Result<()> {
  sqlx::query(r#"
    create table if not exists projects (
    priority integer not null,
    name text not null UNIQUE,
    desc text not null,
    dir text UNIQUE,
    special text UNIQUE
    );
  "#).execute(db)
    .await?;

  Ok(())
}

pub async fn load(db: &SqlitePool) -> Result<Vec<Project>> {
  println!("Loading Tasks");

  let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects")
    .fetch_all(db)
    .await?;

  Ok(projects)
}

pub async fn add(db: &SqlitePool, new_priority: i32, new_name: String, new_desc: String, new_dir: String) -> Result<()> {
  let name_result = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE name==$1")
    .bind(&new_name)
    .fetch_all(db)
    .await?;

  if name_result.is_empty() {
    sqlx::query("INSERT INTO projects (priority, name, desc, dir) VALUES ($1, $2, $3, $4)")
      .bind(new_priority)
      .bind(&new_name)
      .bind(new_desc)
      .bind(new_dir)
      .execute(db)
      .await?;
    println!("{} has been added to tracker", new_name);
  } else {
    println!("{} already exists", new_name);
  }

  Ok(())
}

/// Updates a specific project directory, used by Mark
pub async fn update_directory(db: &SqlitePool, name: String, new_dir: String) -> Result<()> {
  let directories  = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE dir==$1")
    .bind(&new_dir)
    .fetch_all(db)
    .await?;

  if directories.is_empty() {
    sqlx::query(r#"
      UPDATE projects
      SET dir=$1
      WHERE name=$2;
      "#).bind(new_dir)
      .bind(name)
      .execute(db)
      .await?;
  } else {
    println!("Directory already exists");
  }

  Ok(())
}

pub async fn overwrite(db: &SqlitePool, mut projects: Vec<Project>) -> Result<()> {
  clear(db).await?;
  projects.sort_by(|a, b| a.priority.cmp(&b.priority));

  for project in projects {
    let names = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE name==$1")
      .bind(&project.name)
      .fetch_all(db)
      .await?;

    if names.is_empty() {
      match project.dir {
        Some(dir) => {
          sqlx::query("INSERT INTO projects (priority, name, desc, dir) VALUES ($1, $2, $3, $4)")
            .bind(project.priority)
            .bind(project.name)
            .bind(project.desc)
            .bind(dir)
            .execute(db)
            .await?;
        },
        None => {
          sqlx::query("INSERT INTO projects (priority, name, desc) VALUES ($1, $2, $3)")
            .bind(project.priority)
            .bind(project.name)
            .bind(project.desc)
            .execute(db)
            .await?;
        }
      }
    };
  }

  Ok(())
}

pub async fn clear(db: &SqlitePool) -> Result<()> {
  sqlx::query("DELETE FROM projects").execute(db)
    .await?;
    
  Ok(())
}

pub async fn query_name(db: &SqlitePool, name: String) -> Result<Project> {
  let found_special = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE name==$1")
    .bind(&name)
    .fetch_one(db)
    .await?;

  Ok(found_special)
}

pub async fn query_special(db: &SqlitePool) -> Result<Vec<Project>> {
  let found_special = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE special IS NOT NULL")
    .fetch_all(db)
    .await?;
    
  Ok(found_special)
}

pub async fn update_special(db: &SqlitePool, name: String, special: Special) -> Result<()>{
  let found_special = query_special(db).await?;

  let new_special = match special {
    Special::Hook => "Hook".to_string(),

    Special::Mark => "Mark".to_string(),
  };

  for special in found_special {
    sqlx::query(r#"
      UPDATE projects
      SET special=NULL
      WHERE name=$1;
      "#)
      .bind(special.name)
      .execute(db)
      .await?;
  }

  sqlx::query(r#"
    UPDATE projects
    SET special=$1
    WHERE name=$2;
    "#).bind(new_special)
    .bind(name)
    .execute(db)
    .await?;

  Ok(())
}
