use crate::{
    config::Config, dyndns::logic::run_regular, run_options::dyndns_run_options::DynDnsRunOptions,
    services::helpers::build_client,
};

mod logic;

pub fn run(options: DynDnsRunOptions) -> Result<(), Box<dyn std::error::Error>> {
    options
        .check()
        .unwrap_or_else(|e| panic!("Arguments provided are not correct: {}", e));

    println!("Starting in dyndns mode with args: {:?}", &options);

    let config = Config::from_file_or_default(&options.config_file_path)?;

    println!("Starting with config: {:?}", config);

    let domain_controllers = config.into_domain_controllers();

    let proxy = options.proxy.clone();
    let client_maker = Box::new(|| build_client(proxy.clone()));

    run_regular(
        client_maker.as_ref(),
        options.into_simplified(),
        domain_controllers,
    )?;

    Ok(())
}
