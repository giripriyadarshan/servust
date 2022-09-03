use std::process::Command;
use crate::lib::download::download_file;

pub async fn install_framework(framework: String) -> Result<bool, String> {
    // TODO: implement the framework module

    return match framework.as_str() {
        "actix" => actix().await,
        _ => Err(format!(
            "{} is not an implemented framework by Servust",
            framework
        )),
    };
}

async fn actix() -> Result<bool, String> {
    let actix_web = Command::new("cargo")
        .arg("add")
        .arg("actix-web")
        .status()
        .unwrap();

    if !actix_web.success() {
        return Err("Error adding actix package".to_string());
    }

    //TODO: write code to download and replace main.rs and other files

    download_file("https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/frameworks/actix.rs", "src/main.rs").await;

    Ok(true)
}
