use clap::{Parser, Subcommand};

pub mod certbot_run_options;
pub mod dyndns_run_options;

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
    /// Run the dyndns mode to update the DNS record of a specific subdomain.
    Dyndns(dyndns_run_options::DynDnsRunOptions),
}
