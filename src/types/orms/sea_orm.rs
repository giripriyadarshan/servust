use crate::lib::download::download_file;
use std::process::Command;

pub async fn sea_orm(database: String, framework: &str) -> Result<bool, String> {
    let tls = match framework {
        "actix" => "runtime-actix-native-tls",
        "tide" => "runtime-async-std-native-tls",
        _ => "runtime-tokio-native-tls",
    };

    let tokio = Command::new("cargo")
        .arg("add")
        .arg("tokio")
        .arg("--features")
        .arg("full")
        .output()
        .expect("failed to execute process");

    if !tokio.status.success() {
        return Err(String::from_utf8_lossy(&tokio.stderr).to_string());
    }

    let db = match database.as_str() {
        "mysql" => "mysql",
        "postgres" => "postgres",
        "sqlite" => "sqlite",
        _ => "postgres",
    };

    let sea_orm = Command::new("cargo")
        .arg("add")
        .arg("sea-orm")
        .arg("--features")
        .arg(format!("sqlx-{},{}", db, tls))
        .output()
        .unwrap();

    if !sea_orm.status.success() {
        return Err("Error adding sea-orm package".to_string());
    }

    download_file("https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/orms/sea_orm/all_db.rs", "src/db.rs").await;
    Ok(true)
}
