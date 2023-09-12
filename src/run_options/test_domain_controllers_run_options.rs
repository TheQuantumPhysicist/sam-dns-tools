use clap::Parser;

#[derive(Parser, Clone, Debug, Default)]
pub struct TestDomainControllersRunOptions {
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
