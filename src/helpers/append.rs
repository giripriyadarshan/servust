use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn append_at_end(file_name: &str, text: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();

    file.write_all(text.as_bytes()).unwrap();
}
