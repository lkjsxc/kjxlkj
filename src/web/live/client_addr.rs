use axum::http::HeaderMap;
use std::net::{IpAddr, SocketAddr};

pub(crate) fn client_ip(
    headers: &HeaderMap,
    direct: SocketAddr,
    trusted_proxies: &[IpAddr],
) -> IpAddr {
    let direct_ip = direct.ip();
    if !trusted_proxies.contains(&direct_ip) {
        return direct_ip;
    }
    forwarded_ip(headers).unwrap_or(direct_ip)
}

fn forwarded_ip(headers: &HeaderMap) -> Option<IpAddr> {
    x_forwarded_for(headers).or_else(|| header_ip(headers, "x-real-ip"))
}

fn x_forwarded_for(headers: &HeaderMap) -> Option<IpAddr> {
    headers
        .get("x-forwarded-for")?
        .to_str()
        .ok()?
        .split(',')
        .find_map(|part| part.trim().parse().ok())
}

fn header_ip(headers: &HeaderMap, name: &str) -> Option<IpAddr> {
    headers.get(name)?.to_str().ok()?.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::client_ip;
    use axum::http::{HeaderMap, HeaderValue};
    use std::net::{IpAddr, SocketAddr};

    #[test]
    fn ignores_forwarded_headers_from_untrusted_peer() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("203.0.113.10"));
        let direct = "10.0.0.2:443".parse::<SocketAddr>().unwrap();
        assert_eq!(client_ip(&headers, direct, &[]), direct.ip());
    }

    #[test]
    fn trusts_first_forwarded_ip_from_trusted_peer() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            HeaderValue::from_static("203.0.113.10, 10.0.0.2"),
        );
        let direct = "10.0.0.2:443".parse::<SocketAddr>().unwrap();
        let trusted = vec!["10.0.0.2".parse::<IpAddr>().unwrap()];
        assert_eq!(
            client_ip(&headers, direct, &trusted),
            "203.0.113.10".parse::<IpAddr>().unwrap()
        );
    }
}
