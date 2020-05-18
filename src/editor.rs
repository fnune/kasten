use std::{
    env::{temp_dir, var},
    fs::{remove_file, File},
    io::Read,
    process::Command,
};

pub fn create() -> String {
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
        .read_to_string(&mut temp_file)
        .expect("Could not read_to_string from file");

    remove_file(&file_path).expect("Could not remove temp file");

    temp_file
}
