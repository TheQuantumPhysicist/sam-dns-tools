/// DNS TXT records may or may not have quotes. Quotes shouldn't matter.
pub fn compare_dns_txt_value(current: &str, provided: Option<&str>) -> bool {
    match provided {
        None => current.is_empty(),
        Some(provided) => current == provided || current == format!("\"{}\"", provided),
    }
}

pub fn build_client(proxy_address: Option<String>) -> reqwest::blocking::Client {
    let builder = reqwest::blocking::ClientBuilder::new();
    match proxy_address {
        Some(proxy) => builder
            .proxy(reqwest::Proxy::all(proxy).unwrap_or_else(|e| panic!("Invalid proxy URL: {e}")))
            .build()
            .expect("Client builder with proxy failed"),
        None => builder.build().expect("Client builder failed"),
    }
}
