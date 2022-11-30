use crate::lib::download::download_file;
use std::process::Command;

pub async fn axum() -> Result<bool, String> {
    let axum = Command::new("cargo")
        .arg("add")
        .arg("axum")
        .output()
        .unwrap();

    if !axum.status.success() {
        return Err("Error adding axum release candidate package".to_string());
    }

    let tokio = Command::new("cargo")
        .arg("add")
        .arg("tokio")
        .arg("--features")
        .arg("full")
        .output()
        .unwrap();

    if !tokio.status.success() {
        return Err("Error adding tokio package".to_string());
    }

    download_file("https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/frameworks/axum.rs", "src/main.rs").await;

    Ok(true)
}
