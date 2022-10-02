use crate::lib::download::download_file;
use std::process::Command;

pub async fn salvo() -> Result<bool, String> {
    let salvo = Command::new("cargo")
        .arg("add")
        .arg("salvo")
        .arg("--features")
        .arg("affix")
        .output()
        .unwrap();

    if !salvo.status.success() {
        return Err("Error adding salvo package".to_string());
    }

    let tokio = Command::new("cargo")
        .arg("add")
        .arg("tokio")
        .arg("--features")
        .arg("macros")
        .output()
        .unwrap();

    if !tokio.status.success() {
        return Err("Error adding tokio package".to_string());
    }

    download_file("https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/frameworks/salvo.rs", "src/main.rs").await;

    Ok(true)
}
