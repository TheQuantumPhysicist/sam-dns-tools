use clap::Parser;

#[derive(Parser, Clone, Debug, Default)]
pub struct DynDnsRunOptions {
    /// the domain name (as in account), whose DNS records will be updated
    /// DO NOT put the subdomain here for your dyndns without understanding the implications.
    /// This is used to decide which authentication information to use from the config file.
    /// For example, if you want to use dyn.example.com, and your registered domain is example.com,
    /// and the config file has an entry for example.com, then you should put example.com here.
    #[clap(long)]
    pub account_domain_name: Option<String>,

    /// The subdomain to be updated. For example, if you want to update dyn.example.com, put dyn here.
    /// If not provided, the account domain name will be used.
    #[clap(long)]
    pub subdomain: Option<String>,

    /// The path to the config file
    /// If not provided, the default value is used, config.yaml
    #[clap(long, default_value_t = super::DEFAULT_CONFIG_FILE_PATH.to_string())]
    pub config_file_path: String,

    /// Proxy address, such as http, https or socks5, through which the connections to the API will be made
    /// Example: socks5://example.com:1080
    /// This helps to circumvent IP whitelisting requirements for some DNS providers
    #[clap(long)]
    pub proxy: Option<String>,
}

impl DynDnsRunOptions {
    pub fn check(&self) -> Result<(), String> {
        if self.account_domain_name.is_none() {
            return Err("Account domain name not provided".to_string());
        }

        if self.subdomain.is_none() {
            return Err(
                "The subdomain, for which the DNS record will be updated, not provided".to_string(),
            );
        }

        Ok(())
    }

    pub fn into_simplified(self) -> SimplifiedDynDnsRunOptions {
        SimplifiedDynDnsRunOptions {
            account_domain_name: self.account_domain_name.unwrap(),
            subdomain: self.subdomain.unwrap(),
        }
    }
}

pub struct SimplifiedDynDnsRunOptions {
    pub account_domain_name: String,
    pub subdomain: String,
}
