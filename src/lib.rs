use std::{fs, io, path::Path};

pub fn get_input(file_name: &str) -> io::Result<String> {
    let file_stem = Path::new(file_name).file_stem().unwrap_or_default();
    let file_name = Path::new("inputs").join(file_stem).with_extension("txt");
    fs::read_to_string(file_name)
}
