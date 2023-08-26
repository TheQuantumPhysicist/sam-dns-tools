use config::Config;

use crate::{
    logic::run_regular, run_options::RunOptions, services::helpers::build_client, tester::run_test,
};

mod config;
mod logic;
mod run_options;
mod services;
mod tester;
mod traits;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = RunOptions::parse();

    args.check()
        .unwrap_or_else(|e| panic!("Arguments provided are not correct: {}", e));

    println!("Starting with args: {:?}", &args);

    let config = Config::from_file_or_default(&args.config_file_path)?;

    println!("Starting with config: {:?}", config);

    let domain_controllers = config.into_domain_controllers();

    let proxy = args.proxy.clone();
    let client_maker = Box::new(|| build_client(proxy.clone()));

    match args.test_run {
        true => run_test(client_maker.as_ref(), domain_controllers),
        false => run_regular(
            client_maker.as_ref(),
            args.into_simplified(),
            domain_controllers,
        ),
    }?;

    Ok(())
}
