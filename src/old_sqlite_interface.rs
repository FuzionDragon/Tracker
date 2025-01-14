use rusqlite::{Connection, Result, params};
use sqlx::{migrate::MigrateDatabase, Sqlite};

#[derive(Debug)]
pub struct Task {
  pub priority: i32,
  pub name: String,
  pub desc: String,
}

#[tokio::main]
pub async fn init(db_url: &str) -> Result<()> {
  if !Sqlite::database_exists(db_url).await().unwrap_or(false) {
    println!("Creating database: {}", db_url);
    match Sqlite::create_database(db_url).await() {
       Ok(_) => println!("Create db success"), 
       Err(error) => println!("error: {}", error),
    }
  } else {
    println!("Database already exists");
  }

  Ok(())
}

pub fn print(conn: &Connection) -> Result<()> {

  // Retrieves all tasks
  let mut stmt = 
    conn.prepare("select * from tasks")?;
  let tasks = 
    stmt.query_map(
    [],
    |row| {
    Ok(Task {
      priority: row.get(0)?,
      name: row.get(1)?,
      desc: row.get(2)?
    }).into()
  })?;

  // Prints each record
  for task in tasks {
    let row: Task = task.unwrap();
    println!("{}. {}: {}", row.priority, row.name, row.desc);
  }

  Ok(())
}

pub fn load(conn: &Connection) -> Result<Vec<Task>> {

  println!("Loading");

  // Retrieves all tasks
  let mut stmt = 
    conn.prepare("select * from tasks")?;
  let tasks = 
    stmt.query_map(
    [],
    |row| {
    Ok(Task {
      priority: row.get(0)?,
      name: row.get(1)?,
      desc: row.get(2)?
    })
  })?;

  let mut task_vec: Vec<Task> = vec![];
  for task in tasks {
    println!("Loading {:?}", &task);
    let row = task.unwrap();
    task_vec.push(row);
  }

  println!("Task vector {:?}", &task_vec);
  Ok(task_vec)
}

pub fn add(conn: &Connection, new_name: &String, new_desc: &String) -> Result<()> {

  let add_task = "insert into tasks (name, body) values (?1, ?2);"; 

  conn.execute(
    add_task,
    params![new_name, new_desc],
  )?;

  Ok(())
}

pub fn overwrite(conn: &Connection, mut tasks: Vec<Task>) -> Result<()> {
//  clear(&conn)?;

  tasks.sort_by(|a, b| a.priority.cmp(&b.priority));

  let add_task = "insert into tasks (priority, name, desc) values (?1, ?2, ?3);"; 
  println!("Task vector overwrite before prepare {:?}", &tasks);
  for task in tasks {
    println!("Inserting Task {:?}", &task);
    conn.execute(
      &add_task,
      (&task.priority, &task.name, &task.desc)
    )?;
  }

  Ok(())

}

pub fn complete(conn: &Connection, id: &i32) -> Result<()> {

  let complete_task = "delete from tasks where todo_id = ?1";

  conn.execute(
    complete_task, 
    params![id],
  )?;
  
  Ok(())
}

pub fn sort(conn: &Connection) -> Result<()> {

  let sort_tasks = r#"
    create table if not exists sorted_tasks (
    priority integer not null,
    name text not null unique,
    body text not null
    );
    insert into sorted_tasks (priority, name, body) select priority, name, body from tasks order by priority;
    drop table tasks;
    alter table sorted_tasks rename to tasks;
  "#;

  conn.execute(
    sort_tasks, 
    []
  )?;
  
  Ok(())
}

pub fn clear(conn: &Connection) -> Result<()> {

  let clear_tasks = "delete from tasks";

  conn.execute(
    clear_tasks, 
    []
  )?;

  Ok(())
}
