use rusqlite::{params, Connection, NO_PARAMS};

#[derive(Debug)]
struct Entry {
    id: i32,
    title: String,
    content: String,
    date: String,
}

pub fn initialize(conn: &Connection) {
    conn.execute(
        "create table if not exists entries (
             id integer primary key,
             title text not null,
             content text not null,
             date DATETIME DEFAULT CURRENT_TIMESTAMP
         )",
        NO_PARAMS,
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

pub fn save(conn: &Connection, input: String) {
    let mut lines = input.lines();
    let title = lines.nth(0).expect("There's no title!");
    let content_raw = lines.fold(String::new(), |accumulator, line| accumulator + line + "\n");
    let content = content_raw.trim();

    println!("title: {}", title);
    println!("content: {}", content);

    conn.execute(
        "insert into entries (title, content) values (?1, ?2)",
        params![title, content],
    )
    .expect("Error inserting your thing");

    println!("Created a new entry");
}
