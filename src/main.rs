use clap::Parser;
use pho::render;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    text: Vec<String>,
    #[arg(short, long, env = "PHO_SEP", default_value_t = ' ')]
    sep: char,
}

pub fn main() {
    let cli = Cli::parse();
    let sep = format!("{}", cli.sep);
    println!("{}", render(cli.text.join(&sep), sep))
}
