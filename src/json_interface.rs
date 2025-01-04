use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
  pub name: String,
  pub body: String,
}

pub fn read_json(path: &String) -> Result<String> {
  let file = File::open(&path).expect("File couldn't be found");
  let empty = "NULL".into();

  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();

  buf_reader
    .read_to_string(&mut contents)
    .expect("Couldn't convert result into vector");

  if contents != "" {
    Ok(contents)
  } else {
    Ok(empty)
  }
}

fn write_json(path: &String, new_data: Vec<Todo>) -> std::io::Result<()> {
  let ser_data = serde_json::to_string(&new_data).unwrap();
  let mut file = File::create(&path)?;

  write!(file, "{}", ser_data)?;

  Ok(())
}

pub fn add_json(path: String, new_data: Todo) -> Result<()> {
  let ser_data = read_json(&path)?;

  if ser_data == "NULL" {
    let mut data: Vec<Todo> = Vec::new();
    data.push(new_data);
    write_json(&path, data.to_owned()).expect("Cannot initialize json");
  } else {
    let mut checker = false;
    let mut data: Vec<Todo> = serde_json::from_str(&ser_data)?;

    for i in data.iter() {
      if i.name == new_data.name {
        checker = true;
      }
    }

    match checker {
      true => (),

      false => {
        data.push(new_data.to_owned());
        write_json(&path, data).expect("Couldn't write to json");
      }
    }
  }

  Ok(())
}

pub fn remove_json(path: String, item_name: String) -> Result<()> {
  let checker_data = read_json(&path)?;

  if checker_data != "NULL" {
    let data: Vec<Todo> = serde_json::from_str(&checker_data).unwrap();
    let mut new_data = data.to_vec();
    let mut checker = false;
    let mut index = 0;

    for i in 0..data.len() {
      println!("{}: {}", &data[i].name, &data[i].body); 
      if data[i].name == item_name {
        checker = true;
        index = i;
      }
    }

    if data.len() == 1 && data[0].name == item_name {
      checker = true;
      index = 0;
    }

    match checker {
      true => {
        new_data.remove(index);
        write_json(&path, new_data).expect("Couldn't write data to json");
      }

      false => (),
    }
  }

  Ok(())
}
