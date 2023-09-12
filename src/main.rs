use clap::Parser;
use features::{certbot, dyndns, tester};

use crate::run_options::RunOptions;

mod run_options;

mod config;
mod dns_providers;
mod features;
mod traits;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = RunOptions::parse();

    match args.command {
        run_options::RunCommand::Certbot(options) => certbot::run(options),
        run_options::RunCommand::Dyndns(options) => dyndns::run(options),
        run_options::RunCommand::Test(options) => tester::run(options),
    }
}
