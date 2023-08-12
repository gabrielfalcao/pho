use clap::{Parser};
use pho::render;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    text: String
}


pub fn main() {
    let cli = Cli::parse();
    println!("{}", render(cli.text, " "))
}
