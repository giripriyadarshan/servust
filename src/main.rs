use clap::{self, Parser};
use console::{style, Emoji};
use indicatif::HumanDuration;
use std::time::Instant;
use std::{path::Path, process::Command};

mod lib;
mod types;

use lib::{progress_bar::progress_bar, which::which};
use types::install::{install_framework, install_orm};

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Arguments {
    #[clap(required = true)]
    /// The name of the server
    name: String,

    #[clap(short, long)]
    /// library/framework to be used (actix, warp, axum, tonic)
    framework: String,

    #[clap(short, long)]
    /// ORM to be used (diesel, sea-orm)
    orm: String,

    #[clap(short, long)]
    /// database to be used (postgres, mysql, sqlite)
    /// default: postgres
    database: Option<String>,
}

#[tokio::main]
async fn main() {
    let started = Instant::now();
    let args = Arguments::parse();

    if which("cargo").is_none() {
        println!(
            "{}",
            style("Please install cargo using rustup via https://rustup.rs/").red()
        );
        return;
    }

    let new_bin_pb = progress_bar();
    new_bin_pb.set_message("Creating new binary");

    let create_new_bin = Command::new("cargo")
        .arg("new")
        .arg(&args.name)
        .output()
        .unwrap();

    if create_new_bin.status.success() {
        new_bin_pb.finish_and_clear();
        println!(
            "  {} Created new binary {}",
            Emoji("🦀", ""),
            style(&args.name).bold()
        );
    } else {
        new_bin_pb.finish_with_message("Failed to create new binary");
        println!(
            "{}",
            style("Please check if the binary already exists").red()
        );
        return;
    }

    // change directory to the newly created project
    std::env::set_current_dir(Path::new(&args.name)).unwrap();

    let database_arg = match args.database {
        Some(db) => db,
        None => "postgres".to_string(),
    };

    let framework = args.framework.as_str();

    let install_framework_pb = progress_bar();
    install_framework_pb.set_message("Installing framework");
    match install_framework(framework).await {
        Ok(_) => {
            install_framework_pb.finish_and_clear();
            println!(
                "  {} Installed {}",
                Emoji("💻", ""),
                style(framework).bold()
            );
        }
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let install_orm_pb = progress_bar();
    install_orm_pb.set_message("Installing ORM");
    match install_orm(args.orm.clone(), database_arg, framework).await {
        Ok(_) => {
            install_orm_pb.finish_and_clear();
            println!("  {} Installed {}", Emoji("📝", ""), style(args.orm).bold());
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    let elapsed = HumanDuration(started.elapsed()).to_string();
    println! {"\n"}; // everyone needs their space
    println!("  {} Done in {}", Emoji("🎉", ""), style(elapsed).bold());
}
