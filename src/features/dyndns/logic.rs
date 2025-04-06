use std::{collections::BTreeMap, net::Ipv4Addr};

use rand::seq::SliceRandom;

use crate::{
    run_options::dyndns_run_options::SimplifiedDynDnsRunOptions,
    traits::domain_control::{DnsRecord, DnsRecordType, DomainController},
};

/// List of services/URLs to get the public IP address from
const IP_ADDRESSES_SERVICES: [&str; 9] = [
    "https://api.ipify.org",
    "https://checkip.amazonaws.com",
    "https://ipinfo.io/ip",
    "https://ifconfig.me/ip",
    "https://icanhazip.com",
    "https://ipecho.net/plain",
    "https://myexternalip.com/raw",
    "https://ident.me/",
    "https://ip.tyk.nu/",
];

pub fn run_regular(
    client_maker: &dyn Fn() -> reqwest::blocking::Client,
    args: SimplifiedDynDnsRunOptions,
    domain_controllers: BTreeMap<String, Box<dyn DomainController>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let domain_controller = domain_controllers
        .get(&args.account_domain_name)
        .unwrap_or_else(|| {
            panic!(
                "Domain controller for domain {} not found in config",
                &args.account_domain_name
            )
        });

    set_ipv4_record(client_maker, domain_controller.as_ref(), &args.subdomain)?;

    println!("DynDns end reached. If nothing was printed, the record was already set correctly.\n");

    Ok(())
}

fn set_ipv4_record(
    client_maker: &dyn Fn() -> reqwest::blocking::Client,
    domain_controller: &dyn DomainController,
    subdomain: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    const DNS_RECORD_TYPE: DnsRecordType = DnsRecordType::A;

    let my_ip_address = get_my_routable_ip_address()?;

    println!("Found local routable ip address: {}", my_ip_address);

    let records = domain_controller.list_dns_records(client_maker)?;

    let current_ipv4_records = records
        .iter()
        .filter(|r| r.name == subdomain && r.record_type == DNS_RECORD_TYPE)
        .collect::<Vec<&DnsRecord>>();

    remove_reduntant_records(
        domain_controller,
        subdomain,
        &current_ipv4_records,
        client_maker,
    )?;

    // Check if the first record is the same as the current IP address
    if let Some(record) = current_ipv4_records.first() {
        if record.value != my_ip_address.to_string() {
            println!(
                "Record `{}` found but its value is {}. Setting it to the current IP address {}",
                subdomain, record.value, my_ip_address
            );

            // Remove the current incorrect record
            domain_controller.remove_dns_record(
                client_maker,
                subdomain,
                DNS_RECORD_TYPE,
                Some(&record.value),
            )?;

            // Record found, but it's value is different. Update it.
            domain_controller.add_dns_record(
                client_maker,
                subdomain,
                DNS_RECORD_TYPE,
                &my_ip_address.to_string(),
            )?;
        }
    } else {
        println!(
            "Record `{}` not found. Setting it to the current IP address {}",
            subdomain, my_ip_address
        );

        // No record found, create one
        domain_controller.add_dns_record(
            client_maker,
            subdomain,
            DNS_RECORD_TYPE,
            &my_ip_address.to_string(),
        )?;
    }

    Ok(())
}

fn remove_reduntant_records(
    domain_controller: &dyn DomainController,
    subdomain: &str,
    current_ipv4_records: &Vec<&DnsRecord>,
    client_maker: &dyn Fn() -> reqwest::blocking::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    // Remove all records except the first one. Duplicates are bad.
    if current_ipv4_records.len() > 1 {
        println!(
            "Found {} records for `{}`. Removing all except the first one",
            current_ipv4_records.len(),
            subdomain
        );
    }
    for record in current_ipv4_records.iter().skip(1) {
        println!(
            "Removing redundant record `{}` with value {}",
            record.name, record.value
        );

        domain_controller
            .remove_dns_record(
                client_maker,
                subdomain,
                record.record_type,
                Some(&record.value),
            )
            .unwrap_or_else(|e| panic!("Failed to remove redundant record {}: {}", record.name, e));
    }

    Ok(())
}

fn get_my_routable_ip_address() -> Result<Ipv4Addr, Box<dyn std::error::Error>> {
    let services = {
        let mut services = IP_ADDRESSES_SERVICES.to_vec();

        services.shuffle(&mut rand::rng());

        services
    };

    // Get the first IP address that is returned
    let result = services
        .into_iter()
        .map(get_my_ip_address_from_url)
        .find(|r| r.is_ok())
        .ok_or(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not get public IP address from any of the services",
        )))??;

    Ok(result)
}

fn get_my_ip_address_from_url(url: &str) -> Result<Ipv4Addr, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;

    if response.status().is_success() {
        let body = response.text()?;

        let ip_address = body.trim().parse::<Ipv4Addr>().map_err(|e| {
            eprintln!(
                "Could not parse IP address from service {} with value {}. Error: {}",
                url, body, e
            );
            e
        })?;

        return Ok(ip_address);
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Could not get public IP address from any of the services",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_my_routable_ip_address() {
        let ip_address = get_my_routable_ip_address().unwrap();

        let ip_addresses = IP_ADDRESSES_SERVICES
            .iter()
            .cloned()
            .map(get_my_ip_address_from_url)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(
            ip_addresses.into_iter().all(|v| v == ip_address),
            "One of the IP addresses returned by the services is different from the others"
        );

        println!("My IP address is: {}", ip_address);
    }
}
