mod app;
mod args;
mod contentstack;
mod error;
mod generator;
mod region;

use app::App;
use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    match App::new(args) {
        Ok(app) => {
            if let Err(e) = app.run() {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize application: {e}");
            std::process::exit(1);
        }
    }
}
