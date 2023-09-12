use std::collections::BTreeMap;

use rand::Rng;

use crate::{services::helpers, traits::domain_control::DomainController};

fn random_string(length: usize) -> String {
    use rand::distributions::Alphanumeric;

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
        test_singular_add_and_delete_record(client_maker, domain_controller.as_ref(), &name)?;
        test_multiple_add_and_delete_record(client_maker, domain_controller.as_ref(), &name)?;
    }

    println!("All tests have passed successfully.");

    Ok(())
}

fn test_singular_add_and_delete_record(
    client_maker: &dyn Fn() -> reqwest::blocking::Client,
    domain_controller: &dyn DomainController,
    domain_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Testing domain controller's singular record add/remove for domain: {}",
        domain_name
    );
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

    Ok(())
}

fn test_multiple_add_and_delete_record(
    client_maker: &dyn Fn() -> reqwest::blocking::Client,
    domain_controller: &dyn DomainController,
    domain_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Testing domain controller's multiple records add/remove for domain: {}",
        domain_name
    );
    let record_count = rand::thread_rng().gen_range::<usize, _>(5..10);
    let key = random_string(10).to_lowercase();
    let values = (0..record_count)
        .map(|_| random_string(32).to_lowercase())
        .collect::<Vec<_>>();

    assert!(
        values.len() == record_count,
        "Values length is not equal to record count"
    );

    // Add a random records
    for value in &values {
        domain_controller.add_dns_record(
            client_maker,
            &key,
            crate::traits::domain_control::DnsRecordType::TXT,
            value,
        )?;
    }

    // List all records, and try to find it
    let records = domain_controller.list_dns_records(client_maker)?;

    let expected_records = records
        .iter()
        .filter(|r| r.name.to_lowercase() == key)
        .collect::<Vec<_>>();

    // The count found should match the one we stored
    if expected_records.len() != record_count {
        eprintln!(
            "The following record was set but not found in the list: {} {} {}",
            key,
            crate::traits::domain_control::DnsRecordType::TXT,
            values.join(", ")
        );
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Record was not found in the list of recorded provided: {records}",
        )));
    }

    // Make sure every singular value we added exists
    for value in values {
        let expected_record = expected_records.iter().find(|r| {
            r.name.to_lowercase() == key && helpers::compare_dns_txt_value(&r.value, Some(&value))
        });

        if expected_record.is_none() {
            eprintln!(
                "The following record was set but not found in the list: {} {} {}",
                key,
                crate::traits::domain_control::DnsRecordType::TXT,
                value
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Record was not found in the list of records provided: {records}",
            )));
        }
    }

    // Remove the records
    let removed_count = domain_controller.remove_dns_record(
        client_maker,
        &key,
        crate::traits::domain_control::DnsRecordType::TXT,
        None,
    )?;

    if removed_count != record_count {
        eprintln!(
            "Not all records were successfully removed for key {} {}. Added {} records, removed {}",
            key,
            crate::traits::domain_control::DnsRecordType::TXT,
            record_count,
            removed_count
        );
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Record was not removed",
        )));
    }

    Ok(())
}
