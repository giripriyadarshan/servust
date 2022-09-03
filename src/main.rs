use clap::{self, Parser};
use std::{path::Path, process::Command};

mod types;

use types::framework::install_framework;

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Giri Priyadarshan <github/giripriyadarshan>",
    version,
    about = "Rust backend server template generator"
)]
struct Arguments {
    #[clap(forbid_empty_values = true)]
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

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);

    let create_new_bin = Command::new("cargo")
        .arg("new")
        .arg(&args.name)
        .status()
        .unwrap();

    if create_new_bin.success() {
        println!("warming up {}", &args.name);
    } else {
        println!("Error creating {}", &args.name);
        return;
    }

    // change directory to the newly created project
    std::env::set_current_dir(Path::new(&args.name)).unwrap();

    // TODO: implement the database module
    let _database_arg = match args.database {
        Some(db) => db,
        None => "postgres".to_string(),
    };

    match install_framework(args.framework) {
        Ok(_) => println!("framework added"),
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    /*
        The structure of the app
        - `cargo new <name>` // done
        - change directory to <name> // done
        - check if the database is to be set to default or specified correctly // done
        - sort library, database and orm specifications into proper structs
        - pass in database option to the framework.rs and orm.rs files
        - check the <library> and <orm> and install all dependencies using `cargo add`
    */
}
