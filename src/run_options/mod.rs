use clap::{Parser, Subcommand};

pub mod certbot_run_options;
pub mod dyndns_run_options;
pub mod test_domain_controllers_run_options;

const DEFAULT_CONFIG_FILE_PATH: &str = "config.yaml";

#[derive(Parser)]
pub struct RunOptions {
    #[clap(subcommand)]
    pub command: RunCommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum RunCommand {
    /// Run the certbot mode to do the DNS-01 test.
    Certbot(certbot_run_options::CertbotRunOptions),

    /// Run the dyndns mode to update the DNS record of a specific subdomain
    /// from the routable IP address of the machine where this program is running.
    Dyndns(dyndns_run_options::DynDnsRunOptions),

    /// Test domain controllers. These tests ensure that the configuration is correct
    /// and that all the implementations do what they're expected to do. If the tests
    /// work, then all features will work.
    ///
    /// The tester will test setting, reading and erasing a DNS record, and ensuring the value is correctly set
    /// This is useful in case more services are added in the future
    /// Random strings will be generated for set and delete.
    /// This will run for all domains and services in the config file.
    Test(test_domain_controllers_run_options::TestDomainControllersRunOptions),
}
