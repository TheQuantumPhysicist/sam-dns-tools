use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum DnsRecordType {
    A,
    AAAA,
    CAA,
    CNAME,
    MX,
    NS,
    PTR,
    SOA,
    SRV,
    TXT,
}

impl Display for DnsRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsRecordType::A => write!(f, "A"),
            DnsRecordType::AAAA => write!(f, "AAAA"),
            DnsRecordType::CAA => write!(f, "CAA"),
            DnsRecordType::CNAME => write!(f, "CNAME"),
            DnsRecordType::MX => write!(f, "MX"),
            DnsRecordType::NS => write!(f, "NS"),
            DnsRecordType::PTR => write!(f, "PTR"),
            DnsRecordType::SOA => write!(f, "SOA"),
            DnsRecordType::SRV => write!(f, "SRV"),
            DnsRecordType::TXT => write!(f, "TXT"),
        }
    }
}

impl FromStr for DnsRecordType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" => Ok(DnsRecordType::A),
            "aaaa" => Ok(DnsRecordType::AAAA),
            "cname" => Ok(DnsRecordType::CNAME),
            "mx" => Ok(DnsRecordType::MX),
            "ns" => Ok(DnsRecordType::NS),
            "ptr" => Ok(DnsRecordType::PTR),
            "soa" => Ok(DnsRecordType::SOA),
            "srv" => Ok(DnsRecordType::SRV),
            "txt" => Ok(DnsRecordType::TXT),
            _ => Err(format!("Unknown DNS record type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsRecord {
    pub name: String,
    pub record_type: DnsRecordType,
    pub value: String,
}

pub trait DomainController {
    /// Add a DNS record to the domain provided
    fn add_dns_record(
        &self,
        client_maker: &dyn Fn() -> reqwest::blocking::Client,
        name: &str,
        record_type: DnsRecordType,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Remove a DNS record from the domain provided with the provided name (subdomain) and value.
    /// If value is None, all records with the provided name will be removed.
    /// Returns the number of records removed
    /// Note: partial removal is possible, if an error occurs while removing a record
    fn remove_dns_record(
        &self,
        client_maker: &dyn Fn() -> reqwest::blocking::Client,
        name: &str,
        record_type: DnsRecordType,
        value: Option<&str>,
    ) -> Result<usize, Box<dyn std::error::Error>>;

    /// List all DNS records for the domain provided
    fn list_dns_records(
        &self,
        client_maker: &dyn Fn() -> reqwest::blocking::Client,
    ) -> Result<Vec<DnsRecord>, Box<dyn std::error::Error>>;
}
