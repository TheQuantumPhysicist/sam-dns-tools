use run_options::SimplifiedRunOptions;
pub use std::collections::BTreeMap;
use traits::domain_control::DomainController;

use crate::{run_options, traits};

const ACME_CHALLENGE_SUBDOMAIN: &str = "_acme-challenge";

pub fn run_regular(
    client_maker: &dyn Fn() -> reqwest::blocking::Client,
    args: SimplifiedRunOptions,
    domain_controllers: BTreeMap<String, Box<dyn DomainController>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let domain_controller = domain_controllers
        .get(&args.domain_name)
        .unwrap_or_else(|| {
            panic!(
                "Domain controller for domain {} not found in config",
                &args.domain_name
            )
        });

    let value = &args.validation_string;

    match args.operation {
        run_options::Operation::SetRecord => {
            set_record(client_maker, domain_controller.as_ref(), value)?
        }
        run_options::Operation::Cleanup => {
            cleanup(client_maker, domain_controller.as_ref(), value)?
        }
    }

    Ok(())
}

fn set_record(
    client_maker: &dyn Fn() -> reqwest::blocking::Client,
    domain_controller: &dyn DomainController,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    domain_controller.add_dns_record(
        client_maker,
        ACME_CHALLENGE_SUBDOMAIN,
        traits::domain_control::DnsRecordType::TXT,
        value,
    )?;

    println!(
        "Record set end reached: {}={}",
        ACME_CHALLENGE_SUBDOMAIN, value
    );

    Ok(())
}

fn cleanup(
    client_maker: &dyn Fn() -> reqwest::blocking::Client,
    domain_controller: &dyn DomainController,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    domain_controller.remove_dns_record(
        client_maker,
        ACME_CHALLENGE_SUBDOMAIN,
        traits::domain_control::DnsRecordType::TXT,
        Some(value),
    )?;

    println!(
        "Cleanup end reached for: {}={}",
        ACME_CHALLENGE_SUBDOMAIN, value
    );

    Ok(())
}
