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
  let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects")
    .fetch_all(db)
    .await?;

  Ok(projects)
}

// Used by hook_project and mark_project to add 
pub async fn add(db: &SqlitePool, new_priority: i32, new_name: String, new_desc: String, new_dir: String, new_special: Special) -> Result<()> {
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
    println!("Project '{}' has been added to tracker", new_name);
  } else {
    match new_special {
      Special::Hook => {},

      Special::Mark => {},
    }
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
          if let Some(special) = project.special {
            sqlx::query("INSERT INTO projects (priority, name, desc, dir, special) VALUES ($1, $2, $3, $4, $5)")
              .bind(project.priority)
              .bind(project.name)
              .bind(project.desc)
              .bind(dir)
              .bind(special)
              .execute(db)
              .await?;
          } else {
            sqlx::query("INSERT INTO projects (priority, name, desc, dir) VALUES ($1, $2, $3, $4)")
              .bind(project.priority)
              .bind(project.name)
              .bind(project.desc)
              .bind(dir)
              .execute(db)
              .await?;
          };
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

pub async fn print_hooked(db: SqlitePool) -> Result<()> {
  let name = query_special(&db).await?;
  println!("{:?}", name);

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

// Updates special fields, replacing the specific project special if applicable.
pub async fn update_special(db: &SqlitePool, name: String, special: Special) -> Result<()>{
  let found_special = query_special(db).await?;
  let mut marked: Option<String> = None;
  let mut hooked: Option<String> = None;

  for project in found_special {
    if project.special.is_some() {
      let special = project.special.unwrap();
      if special == "MARKED" {
        marked = Some(project.name);
      } else if special == "HOOKED" {
        hooked = Some(project.name);
      }
    }
  }

  /*
   * Cases:
   * 1. Hooked: Prescence of Hooked field, replacing the existing with NULL and new one with the
   *    item
   * 2. Hooked: No prescence of Hooked field, just add it to the selected item
   * 3. Marked: Prescence of Marked field, move the Marked onto this current item
   * 4. Marked: No prescence of Marked field, just add it to the selected item
   */

  match special {
    Special::Hook => {
      if let Some(hook) = hooked {
        sqlx::query(r#"
          UPDATE projects
          SET special=NULL
          WHERE name=$1;
          "#)
          .bind(hook)
          .execute(db)
          .await?;
      }
      sqlx::query(r#"
        UPDATE projects
        SET special='HOOKED'
        WHERE name=$1;
        "#)
        .bind(name)
        .execute(db)
        .await?;
    },

    Special::Mark => {
      if hooked.is_none() {
        if let Some(mark) = marked {
          sqlx::query(r#"
            UPDATE projects
            SET special=NULL
            WHERE name=$1;
            "#)
            .bind(mark)
            .execute(db)
            .await?;
        }
        sqlx::query(r#"
          UPDATE projects
          SET special='MARKED'
          WHERE name=$1;
          "#)
          .bind(name)
          .execute(db)
          .await?;
      }
    }
  }

  Ok(())
}

pub async fn unhook(db: &SqlitePool) -> Result<()> {
  sqlx::query(r#"
    UPDATE projects
    SET special=NULL
    WHERE special='HOOKED';
    "#)
    .execute(db)
    .await?;

  println!("Unhooked any hooked projects");

  Ok(())
}
