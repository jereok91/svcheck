use regex::Regex;
use std::net::{Ipv4Addr, Ipv6Addr};
use url::Url;

pub fn es_url_valida(input: &str) -> bool {
    let regex_url = Regex::new(r"^(https?://)?([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}(/\S*)?$").unwrap();
    if Url::parse(input).is_ok() {
        return true;
    }
    if !input.contains("://") {
        let with_http = format!("http://{}", input);
        if Url::parse(&with_http).is_ok() && regex_url.is_match(&with_http) {
            return true;
        }
    }
    regex_url.is_match(input)
}

pub fn es_ip_valida(input: &str) -> bool {
    if input.parse::<Ipv4Addr>().is_ok() {
        return true;
    }
    if input.parse::<Ipv6Addr>().is_ok() {
        return true;
    }
    false
}

pub fn es_direccion_ip_con_puerto(input: &str) -> bool {
    println!("es valido {}: {}", input, es_ip_valida(input));
    if es_ip_valida(input) {
        return true;
    }
    if let Some((ip, port)) = input.split_once(':') {
        if ip.parse::<Ipv4Addr>().is_ok() && port.parse::<u16>().is_ok() {
            return true;
        }
    }

    if input.starts_with('[') && input.contains("]:") {
        let end_bracket = input.find(']').unwrap_or(0);
        let ip_part = &input[1..end_bracket];
        if ip_part.parse::<Ipv6Addr>().is_ok() {
            let port_part = &input[end_bracket + 2..];
            return port_part.parse::<u16>().is_ok();
        }
    }

    false
}
