/// DNS TXT records may or may not have quotes. Quotes shouldn't matter.
pub fn compare_dns_txt_value(current: &str, provided: Option<&str>) -> bool {
    match provided {
        None => current.is_empty(),
        Some(provided) => current == provided || current == format!("\"{}\"", provided),
    }
}
