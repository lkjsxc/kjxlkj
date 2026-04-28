use std::net::IpAddr;

#[derive(Clone, Debug)]
pub(super) struct IceAddresses {
    public: Vec<String>,
    lan: Vec<String>,
}

impl IceAddresses {
    pub(super) fn new(public: Vec<String>, lan: Vec<String>) -> Self {
        Self { public, lan }
    }

    pub(super) fn candidate_for(&self, client_ip: Option<IpAddr>) -> Option<String> {
        let family = client_ip.map(AddressFamily::from);
        if client_ip.is_some_and(is_lan_ip) {
            candidate_from(&self.lan, family).or_else(|| candidate_from(&self.public, family))
        } else {
            candidate_from(&self.public, family).or_else(|| candidate_from(&self.lan, family))
        }
    }
}

fn candidate_from(values: &[String], family: Option<AddressFamily>) -> Option<String> {
    let valid = valid_candidates(values);
    family
        .and_then(|family| {
            valid
                .iter()
                .find(|candidate| candidate.family == family)
                .cloned()
        })
        .or_else(|| valid.first().cloned())
        .map(|candidate| candidate.value)
}

fn valid_candidates(values: &[String]) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    for value in values {
        match normalize_candidate(value) {
            Some(candidate) if has_family(&candidates, candidate.family) => {
                tracing::warn!("ignoring extra same-family live ICE address");
            }
            Some(candidate) => candidates.push(candidate),
            None => tracing::warn!("ignoring invalid live ICE address"),
        }
    }
    candidates
}

fn has_family(candidates: &[Candidate], family: AddressFamily) -> bool {
    candidates
        .iter()
        .any(|candidate| candidate.family == family)
}

fn normalize_candidate(value: &str) -> Option<Candidate> {
    let parts = value.split('/').collect::<Vec<_>>();
    match parts.as_slice() {
        [external] => {
            let ip = external.parse::<IpAddr>().ok()?;
            Some(Candidate::new(ip.to_string(), ip))
        }
        [external, local] => {
            let external = external.parse::<IpAddr>().ok()?;
            let local = local.parse::<IpAddr>().ok()?;
            (external.is_ipv4() == local.is_ipv4())
                .then(|| Candidate::new(format!("{external}/{local}"), external))
        }
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AddressFamily {
    V4,
    V6,
}

impl From<IpAddr> for AddressFamily {
    fn from(value: IpAddr) -> Self {
        match value {
            IpAddr::V4(_) => Self::V4,
            IpAddr::V6(_) => Self::V6,
        }
    }
}

#[derive(Clone, Debug)]
struct Candidate {
    value: String,
    family: AddressFamily,
}

impl Candidate {
    fn new(value: String, ip: IpAddr) -> Self {
        Self {
            value,
            family: AddressFamily::from(ip),
        }
    }
}

fn is_lan_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => ip.is_private() || ip.is_loopback() || ip.is_link_local(),
        IpAddr::V6(ip) => ip.is_loopback() || ip.is_unique_local() || ip.is_unicast_link_local(),
    }
}

#[cfg(test)]
mod tests {
    use super::IceAddresses;

    #[test]
    fn public_client_gets_public_candidate() {
        let addrs = IceAddresses::new(
            vec!["203.0.113.10".to_string()],
            vec!["10.0.0.10".to_string()],
        );
        assert_eq!(
            addrs.candidate_for(Some("8.8.8.8".parse().unwrap())),
            Some("203.0.113.10".to_string())
        );
    }

    #[test]
    fn lan_client_gets_lan_candidate() {
        let addrs = IceAddresses::new(
            vec!["203.0.113.10".to_string()],
            vec!["10.0.0.10".to_string()],
        );
        assert_eq!(
            addrs.candidate_for(Some("10.0.0.5".parse().unwrap())),
            Some("10.0.0.10".to_string())
        );
    }

    #[test]
    fn extra_same_family_candidates_are_ignored() {
        let addrs = IceAddresses::new(
            vec!["203.0.113.10".to_string(), "198.51.100.20".to_string()],
            Vec::new(),
        );
        assert_eq!(
            addrs.candidate_for(Some("8.8.8.8".parse().unwrap())),
            Some("203.0.113.10".to_string())
        );
    }

    #[test]
    fn client_family_selects_matching_candidate() {
        let addrs = IceAddresses::new(
            vec!["203.0.113.10".to_string(), "2001:db8::10".to_string()],
            Vec::new(),
        );
        assert_eq!(
            addrs.candidate_for(Some("2001:db8::1".parse().unwrap())),
            Some("2001:db8::10".to_string())
        );
    }

    #[test]
    fn explicit_mapping_is_preserved() {
        let addrs = IceAddresses::new(vec!["203.0.113.10/172.18.0.2".to_string()], Vec::new());
        assert_eq!(
            addrs.candidate_for(None),
            Some("203.0.113.10/172.18.0.2".to_string())
        );
    }
}
