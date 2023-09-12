use clap::Parser;

use crate::run_options::RunOptions;

mod run_options;

mod certbot;
mod config;
mod dyndns;
mod services;
mod tester;
mod traits;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = RunOptions::parse();

    match args.command {
        run_options::RunCommand::Certbot(options) => certbot::run(options),
        run_options::RunCommand::Dyndns(options) => dyndns::run(options),
        run_options::RunCommand::Test(options) => tester::run(options),
    }
}
