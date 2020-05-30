use crate::database::Entry;
use itertools::Itertools;

pub struct EntryInput {
    pub title: String,
    pub content: String,
    pub references: Vec::<ReferenceInput>,
}

pub struct ReferenceInput {
  pub target_note_id: i32,
}

#[derive(Debug, Clone)]
pub struct EntrySerializationError;

pub fn parse_entry_input(input: String) -> Result<EntryInput, std::string::ParseError> {
    let mut lines = input.lines();
    let title = lines.nth(0).expect("There's no title!").to_string();
    let content = lines.join("\n");

    Ok(EntryInput { title, content, references: Vec::new() })
}

pub fn serialize_entry(entry: Entry) -> Result<String, EntrySerializationError> {
    Ok(format!("{}\n{}", entry.title, entry.content))
}
