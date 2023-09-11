use clap::{Parser, Subcommand};

use self::certbot_run_options::CertbotRunOptions;

pub mod certbot_run_options;
pub mod dnscontrol_run_options;

#[derive(Parser)]
pub struct RunOptions {
    #[clap(subcommand)]
    pub command: RunCommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum RunCommand {
    /// Run the certbot mode to do the DNS-01 test.
    Certbot(CertbotRunOptions),
}
