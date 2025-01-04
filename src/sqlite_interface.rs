use rusqlite::{Connection, Result, params};

#[derive(Debug)]
pub struct Task {
  pub priority: i32,
  pub name: String,
  pub desc: String,
}

pub fn init(conn: &Connection) -> Result<()> {

  let task_table_create = r#"
    create table if not exists tasks (
    priority integer not null,
    name string not null unique,
    desc string not null
  )"#;

  conn.execute(
    task_table_create,
    [],
  )?;

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

  // Retrieves all tasks
  let mut stmt = 
    conn.prepare("select * from tasks")?;
  let tasks = 
    stmt.query_map(
    [],
    |row| {
    Ok(Task {
      priority: row.get(1)?,
      name: row.get(2)?,
      desc: row.get(3)?
    }).into()
  })?;

  let mut task_vec: Vec<Task> = vec![];
  for task in tasks {
    let row: Task = task.unwrap();
    task_vec.push(row);
  }

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

pub fn overwrite(conn: &Connection, tasks: Vec<Task>) -> Result<()> {

  clear(&conn)?;

  let add_task = "insert into tasks (priority, name, desc) values (?1, ?2, ?3);"; 

  for task in tasks {
    println!("{:?}", &task);
    conn.execute(
      &add_task,
      params![task.priority, task.name, task.desc],
    )?;
  }

  sort(&conn)?;

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
