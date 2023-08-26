use std::collections::BTreeMap;

use crate::{services::helpers, traits::domain_control::DomainController};

fn random_string(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::Rng;

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn run_test(
    client_maker: &dyn Fn() -> reqwest::blocking::Client,
    domain_controllers: BTreeMap<String, Box<dyn DomainController>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for (name, domain_controller) in domain_controllers {
        println!("Testing domain controller for domain: {}", name);
        let key = random_string(10).to_lowercase();
        let value = random_string(32);

        // Add a random record
        domain_controller.add_dns_record(
            client_maker,
            &key,
            crate::traits::domain_control::DnsRecordType::TXT,
            &value,
        )?;

        // List all records, and try to find it
        let records = domain_controller.list_dns_records(client_maker)?;

        let expected_record = records.iter().find(|r| {
            r.name.to_lowercase() == key && helpers::compare_dns_txt_value(&r.value, Some(&value))
        });

        match expected_record {
            Some(_) => (),
            None => {
                eprintln!(
                    "The following record was set but not found in the list: {} {} {}",
                    key,
                    crate::traits::domain_control::DnsRecordType::TXT,
                    value
                );
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Record was not found in the list of recorded provided: {records}",
                )));
            }
        }

        // Remove the record
        let removed_count = domain_controller.remove_dns_record(
            client_maker,
            &key,
            crate::traits::domain_control::DnsRecordType::TXT,
            Some(&value),
        )?;

        if removed_count != 1 {
            eprintln!(
                "The following record was set but not removed: {} {} {}",
                key,
                crate::traits::domain_control::DnsRecordType::TXT,
                value
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Record was not removed",
            )));
        }
    }

    println!("All tests have passed successfully.");

    Ok(())
}
