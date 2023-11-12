use chrono::Local;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

pub fn read_from_file(current_date: Option<chrono::DateTime<Local>>) -> Vec<String> {
    let mut file = create_file(current_date).expect("Unable to create file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read string");
    let contents = contents.split("\nEND_OF_COPIED_ELEMENT_LINE_OF_CLIPPOP\n");
    contents.map(|clip| clip.to_string()).collect()
}

pub fn write_to_file(text: String, current_date: Option<chrono::DateTime<Local>>) {
    let mut file = create_file(current_date).expect("Unable to create file");
    file.write_all(text.as_bytes())
        .expect("Unable to write to file");
    file.write_all(b"\nEND_OF_COPIED_ELEMENT_LINE_OF_CLIPPOP\n")
        .expect("Unable to write to file");
}

pub fn create_backup_directory() -> String {
    let backup_directory_name = format!("./clip_pop_history/");
    let path = Path::new(&backup_directory_name);

    if !path.exists() {
        let _ = fs::create_dir(path);
    }
    backup_directory_name
}

pub fn create_file(current_date: Option<chrono::DateTime<Local>>) -> Result<File, std::io::Error> {
    let file_name = file_name(current_date);
    // create file if not exist
    let file: Result<File, std::io::Error> = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .write(true)
        .open(&file_name);
    file
}

pub fn file_name(current_date: Option<chrono::DateTime<Local>>) -> String {
    let current_date = match current_date {
        Some(date) => date,
        None => Local::now(),
    };
    let backup_directory_name = create_backup_directory();
    let file_name = format!(
        "{}{}.txt",
        backup_directory_name,
        current_date.format("%Y-%m-%d")
    );

    file_name
}
