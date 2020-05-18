use rusqlite::{Connection, Result};
use structopt::StructOpt;

mod database;
mod editor;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "kasten",
    about = "A journaling tool with support for linking entries, which can be used to create a Zettelkasten."
)]
struct Opt {
    /// Prints the database.
    #[structopt(short, long)]
    view: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let conn = Connection::open("development.db")?;

    database::initialize(&conn);

    if opt.view {
        database::view(&conn);
    } else {
        let input = editor::create(None);
        database::save(&conn, input);
    }

    Ok(())
}
