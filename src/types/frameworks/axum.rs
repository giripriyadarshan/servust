use crate::lib::download::download_file;
use std::process::Command;

pub async fn axum() -> Result<bool, String> {
    let axum = Command::new("cargo")
        .arg("add")
        .arg("axum@0.6.0-rc.2")
        .output()
        .unwrap();

    if !axum.status.success() {
        return Err("Error adding axum release candidate package".to_string());
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

    // change beta to main branch after testing and ready to merge with main
    download_file("https://raw.githubusercontent.com/giripriyadarshan/servust/beta/templates/frameworks/axum.rs", "src/main.rs").await;

    Ok(true)
}
