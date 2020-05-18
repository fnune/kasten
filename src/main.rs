use rusqlite::{Connection, Result};
use structopt::StructOpt;

mod database;
mod editor;
mod format;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "kasten",
    about = "A journaling tool with support for linking entries, which can be used to create a Zettelkasten."
)]
struct Opt {
    /// Prints the database.
    #[structopt(short, long)]
    view: bool,

    /// Edits one entry by ID.
    #[structopt(short, long)]
    edit: Option<i32>,

    /// Prints a single entry using its ID.
    #[structopt(short, long)]
    id: Option<i32>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let conn = Connection::open("development.db")?;

    database::initialize(&conn);

    if opt.view {
        database::view(&conn);
    } else if let Some(id) = opt.id {
        let entry = database::view_one(&conn, id).expect("No entry with this ID.");
        println!("{}", entry);
    } else if let Some(id) = opt.edit {
        let entry = database::view_one(&conn, id).expect("No entry with that ID.");
        let input = editor::create(Some(entry));
        database::update(&conn, input, id);
    } else {
        let input = editor::create(None);
        database::save(&conn, input);
    }

    Ok(())
}
