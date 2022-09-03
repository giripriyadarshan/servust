use crate::lib::download::download_file;
use std::process::Command;

pub async fn sea_orm(database: String, framework: &str) -> Result<bool, String> {
    let tls = match framework {
        "actix" => "runtime-actix-native-tls",
        "tide" => "runtime-async-std-native-tls",
        _ => "runtime-tokio-native-tls",
    };

    let sea_orm = Command::new("cargo")
        .arg("add")
        .arg("sea-orm")
        .arg("--features")
        .arg(format!("sqlx-{},{}", database, tls))
        .status()
        .unwrap();

    if !sea_orm.success() {
        return Err("Error adding sea-orm package".to_string());
    }

    println!("get started with Sea-ORM at https://www.sea-ql.org/SeaORM/docs/index/");

    download_file(&format!(
        "https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/orms/sea_orm/all_db.rs"), "src/db.rs").await;
    Ok(true)
}
