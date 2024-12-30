use rusqlite::{Connection, Result, params};

struct Task {
  line: i32,
  priority: i32,
  name: String,
  desc: String,
}

pub fn init (conn: &Connection) -> Result<()> {

  let task_table_create = 
    "create table if not exists tasks (
      line: i32 not null unique,
      priority: i32 not null unique,
      name text not null unique,
      body text not null
    )";

  conn.execute(
    task_table_create,
    [],
  )?;

  Ok(())
}

pub fn list (conn: &Connection) -> Result<()> {

  // Retrieves all tasks
  let mut stmt = 
    conn.prepare("select * from tasks")?;
  let tasks = 
    stmt.query_map(
    [],
    |row| {
    Ok(Task {
      line: row.get(0)?,
      priority: row.get(1)?,
      name: row.get(2)?,
      desc: row.get(3)?
    }).into()
  })?;

  // Prints each record
  for task in tasks {
    let row: Task = task.unwrap();
    println!("{2}. {0}: {1}", row.name, row.desc, row.priority);
  }

  Ok(())
}

pub fn add (conn: &Connection, new_name: &String, new_desc: &String) -> Result<()> {

  let add_task = "insert into tasks (name, body) values (?1, ?2);"; 

  conn.execute(
    add_task,
    params![new_name, new_desc],
  )?;

  Ok(())
}

pub fn complete (conn: &Connection, id: &i32) -> Result<()> {

  let complete_task = "delete from tasks where todo_id = ?1";

  conn.execute(
    complete_task, 
    params![id],
  )?;
  
  Ok(())
}

pub fn clear (conn: &Connection) -> Result<()> {

  let clear_tasks = "delete from tasks";

  conn.execute(
    clear_tasks, 
    []
  )?;

  Ok(())
}
