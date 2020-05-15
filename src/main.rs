extern crate rusqlite;

use std::{
    env::{temp_dir, var},
    fs::{File, remove_file},
    io::Read,
    process::Command,
};

use rusqlite::{Connection, Result, NO_PARAMS, params};

fn main() -> Result<()> {
    let conn = Connection::open("development.db")?;

    conn.execute(
        "create table if not exists entries (
             id integer primary key,
             title text not null,
             content text not null,
             date DATETIME DEFAULT CURRENT_TIMESTAMP
         )",
        NO_PARAMS,
    ).expect("Error creating the database");

    let editor = var("EDITOR").expect("No $EDITOR set!");

    let mut file_path = temp_dir();
    file_path.push("editable");
    File::create(&file_path).expect("Could not create file");

    Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Something went wrong");

    let mut temp_file = String::new();
    File::open(&file_path)
        .expect("Could not open file")
        .read_to_string(&mut temp_file).expect("Could not read_to_string from file");

    remove_file(&file_path).expect("Could not remove temp file");

    let mut lines = temp_file.lines();
    let title = lines.nth(0).expect("There's no title!");
    let content_raw = lines.fold(String::new(), |accumulator, line| accumulator + line + "\n");
    let content = content_raw.trim();

    println!("title: {}", title);
    println!("content: {}", content);

    conn.execute(
      "insert into entries (title, content) values (?1, ?2)",
      params![title, content],
    ).expect("Error inserting your thing");

    println!("Created a new entry");

    Ok(())
}

