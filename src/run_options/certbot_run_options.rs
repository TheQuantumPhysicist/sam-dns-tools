use std::str::FromStr;

use clap::Parser;

const DEFAULT_CONFIG_FILE_PATH: &str = "config.yaml";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    SetRecord,
    Cleanup,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "set-record" => Ok(Operation::SetRecord),
            "cleanup" => Ok(Operation::Cleanup),
            _ => Err(format!("Unknown operation: {}", s)),
        }
    }
}

#[derive(Parser, Clone, Debug, Default)]
pub struct CertbotRunOptions {
    /// Test run
    /// If set, the program will test setting, reading and erasing a DNS record, and ensuring the value is correctly set
    /// This is useful in case more services are added in the future
    /// This does not need a validation string. Random strings will be generated, set and deleted.
    /// This will run for all domains and services in the config file.
    #[clap(long, default_value = "false")]
    pub test_run: bool,

    /// the domain name, whose DNS records will be updated ($CERTBOT_DOMAIN goes here)
    /// Note that this domain's configuration must be present in the config file
    #[clap(long)]
    pub domain_name: Option<String>,

    /// Operation to be done. Possible values: `set-record` or `cleanup`
    /// set-record: set the TXT DNS record to the provided validation string
    /// cleanup: remove the TXT DNS record
    #[clap(long)]
    pub operation: Option<Operation>,

    /// The validation string to be set as TXT DNS record ($CERTBOT_VALIDATION goes here)
    /// Only used with operation set-record, otherwise error
    #[clap(long)]
    pub validation_string: Option<String>,

    /// The path to the config file
    /// If not provided, the default value is used, config.yaml
    #[clap(long, default_value_t = DEFAULT_CONFIG_FILE_PATH.to_string())]
    pub config_file_path: String,

    /// Proxy address, such as http, https or socks5, through which the connections to the API will be made
    /// Example: socks5://example.com:1080
    /// This helps to circumvent IP whitelisting requirements for some DNS providers
    #[clap(long)]
    pub proxy: Option<String>,
}

impl CertbotRunOptions {
    pub fn check(&self) -> Result<(), String> {
        if self.test_run {
            if self.domain_name.is_some() {
                return Err(
                    "Domain name should not be provided in test mode. All domains will be tested."
                        .to_string(),
                );
            }

            if self.operation.is_some() {
                return Err(
                    "Operation should not be provided in test mode. All operations will be tested."
                        .to_string(),
                );
            }

            if self.validation_string.is_some() {
                return Err(
                    "Validation string should not be provided in test mode. Random strings will be used."
                        .to_string(),
                );
            }
        } else {
            if self.domain_name.is_none() {
                return Err("Domain name not provided".to_string());
            }

            if self.operation.is_none() {
                return Err("Operation not provided".to_string());
            }

            if self.validation_string.is_none() {
                return Err("Validation string not provided".to_string());
            }
        }
        Ok(())
    }

    pub fn into_simplified(self) -> SimplifiedRunOptions {
        SimplifiedRunOptions {
            domain_name: self.domain_name.unwrap(),
            operation: self.operation.unwrap(),
            validation_string: self.validation_string.unwrap(),
        }
    }
}

pub struct SimplifiedRunOptions {
    pub domain_name: String,
    pub operation: Operation,
    pub validation_string: String,
}
