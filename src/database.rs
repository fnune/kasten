use crate::format;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Entry {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub date: String,
}

pub fn initialize(conn: &Connection) {
    conn.execute_batch(
        "create table if not exists 'entries' (
             id integer primary key,
             title text not null,
             content text not null,
             date DATETIME DEFAULT CURRENT_TIMESTAMP
         );
         create table if not exists 'references' (
             id integer primary key,
             source_note_id integer not null,
             target_note_id integer not null,
             date DATETIME DEFAULT CURRENT_TIMESTAMP,
             FOREIGN KEY(source_note_id) REFERENCES entries(id),
             FOREIGN KEY(target_note_id) REFERENCES entries(id)
         );"
    )
    .expect("Error creating the database");
}

pub fn view(conn: &Connection) {
    let mut statement = conn
        .prepare("select id, title, content, date from entries")
        .expect("Error preparing query.");
    let entries_iter = statement
        .query_map(params![], |row| {
            Ok(Entry {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                date: row.get(3)?,
            })
        })
        .expect("Failed to execute the query.");

    for entry in entries_iter {
        println!("{:?}", entry.unwrap());
    }
}

pub fn view_one(conn: &Connection, id: i32) -> Result<String> {
    conn.query_row(
        "SELECT id, title, content, date FROM entries WHERE id = ?1",
        params![id],
        |row| {
            Ok(format::serialize_entry(Entry {
                id: row.get(0).expect("Could not get ID from row."),
                title: row.get(1).expect("Could not get title from row."),
                content: row.get(2).expect("Could not get content from row."),
                date: row.get(3).expect("Could not get date from row."),
            })
            .expect("Failed to serialize entry."))
        },
    )
}

pub fn save(conn: &Connection, input: String) {
    let parsed = format::parse_entry_input(input).expect("Could not parse input as EntryInput.");

    conn.execute(
        "insert into entries (title, content) values (?1, ?2)",
        params![parsed.title, parsed.content],
    )
    .expect("Error inserting your thing");

    println!("Created a new entry.");
}

pub fn update(conn: &Connection, input: String, id: i32) {
    let parsed = format::parse_entry_input(input).expect("Could not parse input as EntryInput.");

    conn.execute(
        "UPDATE entries SET title = ?1, content = ?2 WHERE id = ?3",
        params![parsed.title, parsed.content, id],
    )
    .expect("Error updating your thing");

    println!("Updated your entry.");
}
