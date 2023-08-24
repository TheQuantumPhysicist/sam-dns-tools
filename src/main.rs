use std::collections::BTreeMap;

use config::Config;
use run_options::SimplifiedRunOptions;
use traits::domain_control::DomainController;

use crate::{run_options::RunOptions, tester::run_test};

mod config;
mod run_options;
mod services;
mod tester;
mod traits;

fn run_regular(
    _args: SimplifiedRunOptions,
    _domain_controllers: BTreeMap<String, Box<dyn DomainController>>,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = RunOptions::parse();

    args.check()
        .unwrap_or_else(|e| panic!("Arguments provided are not correct: {}", e));

    println!("Starting with args: {:?}", &args);

    let config = Config::from_file_or_default(&args.config_file_path)?;

    println!(
        "Starting with config: {:?}",
        serde_yaml::to_string(&config)?
    );

    let domain_controllers = config.into_domain_controllers();

    match args.test_run {
        true => run_test(domain_controllers),
        false => run_regular(args.into_simplified(), domain_controllers),
    }?;

    Ok(())
}
