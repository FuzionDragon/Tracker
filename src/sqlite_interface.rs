use anyhow::{ Ok, Result };
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, FromRow, Row, Error::Database};

#[derive(Debug, FromRow, Clone)]
pub struct Project {
  pub priority: i32,
  pub name: String,
  pub desc: String,
  pub dir: Option<String>,
}

pub enum Fields {
  Priority,
  Name,
  Desc,
  Dir,
}

pub async fn init(db: &SqlitePool, name: String) -> Result<()> {
  sqlx::query(r#"
    create table if not exists projects (
    priority integer not null,
    name text not null UNIQUE,
    desc text not null,
    dir text UNIQUE
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
  let result = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE name==$1")
    .bind(&new_name)
    .fetch_all(db)
    .await?;

  if result.iter().count() == 0 {
    sqlx::query("INSERT INTO projects (priority, name, desc, dir) VALUES ($1, $2, $3, $4)")
      .bind(new_priority)
      .bind(&new_name)
      .bind(new_desc)
      .bind(new_dir)
      .execute(db)
      .await?;
    println!("{} has been added to tracker", new_name);
  } else {
    println!("{} is already marked", new_name);
  }

  Ok(())
}

pub async fn update(db: &SqlitePool, name: String, new_dir: String) -> Result<()> {
  sqlx::query(r#"
    UPDATE projects
    SET dir=$1
    WHERE name=$2;
    "#).bind(new_dir)
    .bind(name)
    .execute(db)
    .await?;

  Ok(())
}

pub async fn overwrite(db: &SqlitePool, mut projects: Vec<Project>) -> Result<()> {
  clear(&db).await?;
  projects.sort_by(|a, b| a.priority.cmp(&b.priority));

  for project in projects {
    let result = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE name==$1")
      .bind(&project.name)
      .fetch_all(db)
      .await?;

    if result.iter().count() == 0 {
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
