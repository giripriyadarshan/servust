use crate::lib::download::download_file;
use std::process::Command;

pub async fn salvo() -> Result<bool, String> {
    let salvo = Command::new("cargo")
        .arg("add")
        .arg("salvo")
        .arg("--features")
        .arg("affix")
        .status()
        .unwrap();

    if !salvo.success() {
        return Err("Error adding salvo package".to_string());
    }

    let tokio = Command::new("cargo")
        .arg("add")
        .arg("tokio")
        .arg("--features")
        .arg("macros")
        .status()
        .unwrap();

    if !tokio.success() {
        return Err("Error adding tokio package".to_string());
    }

    // WARNING: Replace beta with main when stable is released
    download_file("https://raw.githubusercontent.com/giripriyadarshan/servust/beta/templates/frameworks/salvo.rs", "src/main.rs").await;

    Ok(true)
}
