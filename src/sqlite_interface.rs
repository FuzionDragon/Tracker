use anyhow::{ Ok, Result };
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, FromRow, Row, Error::Database};

#[derive(Debug, FromRow, Clone)]
pub struct Task {
  pub priority: i32,
  pub name: String,
  pub desc: String,
}

pub async fn init(db: &SqlitePool, name: String) -> Result<()> {
  let result = sqlx::query(r#"
    create table if not exists tasks (
    priority integer not null,
    name text not null,
    desc text not null
    );
  "#).execute(db)
    .await
    .unwrap();

  Ok(())
}

pub async fn load(db: &SqlitePool) -> Result<Vec<Task>> {
  println!("Loading Tasks");

  let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
    .fetch_all(db)
    .await
    .unwrap();

  Ok(tasks)
}

pub async fn add(db: &SqlitePool, new_priority: i32, new_name: String, new_desc: String) -> Result<()> {
  let tasks = sqlx::query("INSERT INTO tasks (priority, name, desc) VALUES ($1, $2, $3)")
    .bind(new_priority)
    .bind(new_name)
    .bind(new_desc)
    .execute(db)
    .await
    .unwrap();

  Ok(())
}

pub async fn overwrite(db: &SqlitePool, mut tasks: Vec<Task>) -> Result<()> {
  clear(&db).await?;
  tasks.sort_by(|a, b| a.priority.cmp(&b.priority));

  for task in tasks {
    sqlx::query("INSERT INTO tasks (priority, name, desc) VALUES ($1, $2, $3)")
      .bind(task.priority)
      .bind(task.name)
      .bind(task.desc)
      .execute(db)
      .await
      .unwrap();
  };

  Ok(())
}

pub async fn clear(db: &SqlitePool) -> Result<()> {
  let result = sqlx::query("DELETE FROM tasks").execute(db)
    .await
    .unwrap();
    
  Ok(())
}
