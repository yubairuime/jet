use clap::Parser;
use commands::Cli;
mod commands;
mod generate;
mod server;
mod helper;
mod articles;
mod rss;
mod blog;

fn main() {
    let current_directory = String::from("./");

    if !helper::check_is_root(current_directory) {
        println!("Error: the current directory is not a Jet project.");
        return;
    } else {
        let cli = Cli::parse();
        cli.run();
    }
}
