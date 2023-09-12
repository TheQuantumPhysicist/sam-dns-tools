use crate::{
    config::Config, dns_providers::helpers::build_client,
    run_options::test_domain_controllers_run_options::TestDomainControllersRunOptions,
};

mod tester;

pub fn run(options: TestDomainControllersRunOptions) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Starting in domain-controllers test mode with args: {:?}",
        &options
    );

    let config = Config::from_file_or_default(&options.config_file_path)?;

    println!("Starting with config: {:?}", config);

    let domain_controllers = config.into_domain_controllers();

    let proxy = options.proxy.clone();
    let client_maker = Box::new(|| build_client(proxy.clone()));

    tester::run_test(client_maker.as_ref(), domain_controllers)?;

    Ok(())
}
