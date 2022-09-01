use clap::{self, Parser};

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
    /// ORM to be used (diesel, sea-orm)
    /// Default: diesel
    orm: Option<String>,

    #[clap(short, long)]
    /// library/framework to be used (actix, warp, axum, tonic)
    library: String,

    #[clap(short, long)]
    /// database to be used (postgres, mysql, sqlite)
    /// default: postgres
    database: Option<String>,
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
}
