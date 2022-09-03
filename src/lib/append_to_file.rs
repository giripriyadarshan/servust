use std::fs;
use std::io::Write;

pub fn append(text: String, path: String) -> Result<(), String> {
    let original_contents = fs::read_to_string(&path).unwrap();

    let mut file = fs::File::create(&path).unwrap();

    file.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
    file.write_all(original_contents.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}
