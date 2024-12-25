use clap::{builder::TypedValueParser, command, error::Result, Parser};
use rusqlite::{Connection, Result};
use std::{
  env::var,
  fs::File,
  io::Read,
  process::Command,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Edit tasks in default text editor $EDITOR
  #[arg(short, long, default_missing_value="always")]
  edit: String,  

  /// Lists all tasks in a given tracker
  #[arg(short, long)]
  list: String,  

  /// Clears all tasks in a given tracker
  #[arg(short, long)]
  clear: String,  

  /// Queries a specific task based on ID
  #[arg(short, long)]
  query: String,  
}

fn main() -> Result<()> {
  let args = Args::parse();

  Ok(());
}
