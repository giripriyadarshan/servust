use crate::lib::download::download_file;
use std::process::Command;

pub async fn actix() -> Result<bool, String> {
    let actix_web = Command::new("cargo")
        .arg("add")
        .arg("actix-web")
        .output()
        .unwrap();

    if !actix_web.status.success() {
        return Err("Error adding actix package".to_string());
    }

    download_file("https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/frameworks/actix.rs", "src/main.rs").await;

    Ok(true)
}
