use crate::lib::download::download_file;
use std::process::Command;

pub async fn install_orm(orm: String, database: String) -> Result<bool, String> {
    // TODO: implement the orm module

    return match orm.as_str() {
        "diesel" => diesel(database).await,
        _ => Err(format!("{} is not an implemented orm by Servust", orm)),
    };
}

async fn diesel(database: String) -> Result<bool, String> {
    let diesel = Command::new("cargo")
        .arg("add")
        .arg("diesel")
        .arg("--features")
        .arg(format!("{},r2d2", database))
        .status()
        .unwrap();

    if !diesel.success() {
        return Err("Error adding diesel package".to_string());
    }

    let r2d2 = Command::new("cargo")
        .arg("add")
        .arg("r2d2")
        .status()
        .unwrap();

    if !r2d2.success() {
        return Err("Error adding r2d2 package".to_string());
    }

    println!("get started with Diesel ORM at https://diesel.rs/guides/getting-started");

    download_file(&format!(
        "https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/orms/diesel/{}.rs",
        &database), "src/db.rs").await;

    Ok(true)
}
