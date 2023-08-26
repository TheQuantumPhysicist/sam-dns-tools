use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::traits::domain_control::{DnsRecord, DnsRecordType, DomainController};

use super::helpers::compare_dns_txt_value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

pub const DEFAULT_TTL: u32 = 360;
pub const DEFAULT_AUX: u32 = 0;

/// Epic requires only the signature string to be provided for the requests
/// (that are submitted in GET requests).
/// Their security is based on IP address whitelisting.
///
/// Note for developers adding more services:
/// If you'd like to add services, you have to create a similar struct,
/// support Serialize and Deserialize, and implement the DomainController trait.
/// Your structs must be deserialized from the config file and be used as DomainControllers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Epik {
    domain_name: String,
    signature: String,
}

impl Epik {
    pub fn domain_name(&self) -> &String {
        &self.domain_name
    }
}

impl Epik {
    fn list_full_dns_records(
        &self,
        client_maker: &dyn Fn() -> reqwest::blocking::Client,
    ) -> Result<EpikDnsRecordsResponse, Error> {
        let url = format!(
            "https://usersapiv2.epik.com/v2/domains/{}/records?SIGNATURE={}",
            self.domain_name, self.signature
        );

        let client = client_maker();

        let resp = client.request(Method::GET, url).send()?;

        if !resp.status().is_success() {
            return Err(Error::Reqwest(resp.error_for_status().unwrap_err()));
        }

        let resp_json = resp.json::<EpikDnsRecordsResponse>()?;

        assert_eq!(
            resp_json.data.domain_name, self.domain_name,
            "Domain name returned doesn't match the requested domain name"
        );

        Ok(resp_json)
    }

    fn delete_dns_record(
        &self,
        client_maker: &dyn Fn() -> reqwest::blocking::Client,
        id: &str,
    ) -> Result<(), Error> {
        let url = format!(
            "https://usersapiv2.epik.com/v2/domains/{}/records?SIGNATURE={}&ID={id}",
            self.domain_name, self.signature
        );

        let client = client_maker();

        let resp = client.request(Method::DELETE, url).send()?;

        if !resp.status().is_success() {
            return Err(Error::Reqwest(resp.error_for_status().unwrap_err()));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct EpikDnsEntry {
    id: String,
    name: String,
    #[serde(rename = "type")]
    record_type: DnsRecordType,
    data: String,
    aux: u32,
    ttl: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct EpikDnsRecordsResponse {
    data: EpikDnsRecordsResponseData,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct EpikDnsRecordsResponseData {
    #[serde(rename = "name")]
    domain_name: String,
    code: u32,
    records: Vec<EpikDnsEntry>,
}

impl TryFrom<EpikDnsEntry> for DnsRecord {
    type Error = String;

    fn try_from(value: EpikDnsEntry) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            record_type: value.record_type,
            value: value.data,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CreateHostRecordsPayload {
    #[serde(rename = "HOST")]
    host: String,
    #[serde(rename = "TYPE")]
    record_type: DnsRecordType,
    #[serde(rename = "DATA")]
    data: String,
    #[serde(rename = "AUX")]
    aux: u32,
    #[serde(rename = "TTL")]
    ttl: u32,
}

impl DomainController for Epik {
    fn add_dns_record(
        &self,
        client_maker: &dyn Fn() -> reqwest::blocking::Client,
        name: &str,
        record_type: DnsRecordType,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://usersapiv2.epik.com/v2/domains/{}/records?SIGNATURE={}",
            self.domain_name, self.signature
        );

        let body = serde_json::to_string(&CreateHostRecordsPayload {
            host: name.to_string(),
            record_type,
            data: value.to_string(),
            aux: DEFAULT_AUX,
            ttl: DEFAULT_TTL,
        })
        .expect("Serializing CreateHostRecordsPayload to JSON should never fail");
        // requests are wrapped in this "create_host_records_payload" key
        let body = format!("{{ \"create_host_records_payload\": {body} }}");

        let client = client_maker();

        let resp = client.post(url).body(body).send()?;

        if !resp.status().is_success() {
            return Err(Box::new(Error::Reqwest(
                resp.error_for_status().unwrap_err(),
            )));
        }

        Ok(())
    }

    fn remove_dns_record(
        &self,
        client_maker: &dyn Fn() -> reqwest::blocking::Client,
        name: &str,
        record_type: DnsRecordType,
        value: Option<&str>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let current_records = self.list_full_dns_records(client_maker)?;

        let records_to_remove = current_records
            .data
            .records
            .iter()
            .filter(|r| {
                r.name == name
                    && r.record_type == record_type
                    && compare_dns_txt_value(&r.data, value)
            })
            .map(|r| r.id.clone())
            .collect::<Vec<String>>();

        let size_to_remove = records_to_remove.len();

        records_to_remove
            .into_iter()
            .try_for_each(|id| self.delete_dns_record(client_maker, &id))?;

        Ok(size_to_remove)
    }

    fn list_dns_records(
        &self,
        client_maker: &dyn Fn() -> reqwest::blocking::Client,
    ) -> Result<Vec<DnsRecord>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://usersapiv2.epik.com/v2/domains/{}/records?SIGNATURE={}",
            self.domain_name, self.signature
        );

        let client = client_maker();

        let resp = client.get(url).send()?;

        if !resp.status().is_success() {
            return Err(Box::new(Error::Reqwest(
                resp.error_for_status().unwrap_err(),
            )));
        }

        let resp_json = resp.json::<EpikDnsRecordsResponse>()?;

        assert_eq!(
            resp_json.data.domain_name, self.domain_name,
            "Domain name returned doesn't match the requested domain name"
        );

        let dns_entries = resp_json.data.records;

        let dns_records = dns_entries
            .into_iter()
            .map(|e| e.try_into())
            .collect::<Result<Vec<DnsRecord>, String>>()?;

        Ok(dns_records)
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::domain_control::DnsRecordType;

    use super::*;

    #[test]
    fn test_deserializing_dns_entry() {
        let json = r#"
        {
            "id": "abc-xyz",
            "name": "www",
            "type": "A",
            "data": "1.2.3.4",
            "aux": 0,
            "ttl": 300
        }
        "#;
        let deserialized: EpikDnsEntry = serde_json::from_str(json).unwrap();

        assert_eq!(deserialized.id, "abc-xyz");
        assert_eq!(deserialized.name, "www");
        assert_eq!(deserialized.record_type, DnsRecordType::A);
        assert_eq!(deserialized.data, "1.2.3.4");
        assert_eq!(deserialized.aux, 0);
        assert_eq!(deserialized.ttl, 300);

        let dns_record: DnsRecord = deserialized.try_into().unwrap();
        assert_eq!(dns_record.name, "www");
        assert_eq!(dns_record.record_type, DnsRecordType::A);
        assert_eq!(dns_record.value, "1.2.3.4");
    }

    #[test]
    fn test_deserializing_dns_response() {
        let json = r#"
        {
            "data": {
                "name": "example.com",
                "code": 1000,
                "records": [
                    {
                        "id": "abcdefg",
                        "name": "www",
                        "type": "A",
                        "data": "1.2.3.4",
                        "aux": 0,
                        "ttl": 300
                    },
                    {
                        "id": "fffff",
                        "name": "mail",
                        "type": "CAA",
                        "data": "the great data",
                        "aux": 0,
                        "ttl": 300
                    }
                ]
            }
        }"#;
        let deserialized: EpikDnsRecordsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.data.domain_name, "example.com");
        assert_eq!(deserialized.data.code, 1000);
        assert_eq!(deserialized.data.records.len(), 2);
    }
}
