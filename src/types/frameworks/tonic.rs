use crate::lib::{append::append_at_end, download::download_file};
use std::process::Command;

pub async fn tonic() -> Result<bool, String> {
    let tonic = Command::new("cargo")
        .arg("add")
        .arg("tonic")
        .status()
        .unwrap();

    if !tonic.success() {
        return Err("Error adding tonic package".to_string());
    }

    let prost = Command::new("cargo")
        .arg("add")
        .arg("prost")
        .status()
        .unwrap();

    if !prost.success() {
        return Err("Error adding prost package".to_string());
    }

    let tonic_build = Command::new("cargo")
        .arg("add")
        .arg("tonic-build")
        .arg("--build")
        .status()
        .unwrap();

    if !tonic_build.success() {
        return Err("Error adding tonic-build package".to_string());
    }

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

    // append bin to Cargo.toml
    let tonic_bin_append_text = "\n[lib]\npath=\"./src/lib.rs\"\n\n[[bin]]\nname=\"server\"\npath=\"./src/server.rs\"\n\n[[bin]]\nname=\"client\"\npath=\"./src/client.rs\"\n";
    append_at_end("Cargo.toml", tonic_bin_append_text);

    std::fs::remove_file("src/main.rs").unwrap();

    std::fs::create_dir("proto").unwrap();

    download_file(
        "https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/tonic/server.rs",
        "src/server.rs",
    )
    .await;
    download_file(
        "https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/frameworks/tonic/client.rs",
        "src/client.rs",
    )
    .await;
    download_file(
        "https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/frameworks/tonic/lib.rs",
        "src/lib.rs",
    )
    .await;
    download_file(
        "https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/frameworks/tonic/main.proto",
        "proto/main.proto",
    ).await;
    download_file(
        "https://raw.githubusercontent.com/giripriyadarshan/servust/main/templates/frameworks/tonic/build.rs",
        "build.rs",
    )
    .await;

    Ok(true)
}
