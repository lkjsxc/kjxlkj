//! Nostr NIP-05/NIP-19 helpers

use serde_json::{Map, Value};
use std::collections::BTreeSet;

const CHARSET: &str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";

pub fn normalize_names_json(value: &str) -> Result<Value, String> {
    let parsed: Value = serde_json::from_str(blank_default(value, "{}"))
        .map_err(|_| "Nostr names must be a JSON object".to_string())?;
    let object = parsed
        .as_object()
        .ok_or_else(|| "Nostr names must be a JSON object".to_string())?;
    let mut normalized = Map::new();
    for (name, key) in object {
        let name = normalize_name(name)?;
        if normalized.contains_key(&name) {
            return Err("Nostr names must be unique after lowercase normalization".to_string());
        }
        let key = key
            .as_str()
            .ok_or_else(|| "Nostr public keys must be strings".to_string())?;
        normalized.insert(name, Value::String(normalize_public_key(key)?));
    }
    Ok(Value::Object(normalized))
}

pub fn normalize_relays_json(value: &str) -> Result<Value, String> {
    let parsed: Value = serde_json::from_str(blank_default(value, "[]"))
        .map_err(|_| "Nostr relays must be a JSON array".to_string())?;
    let array = parsed
        .as_array()
        .ok_or_else(|| "Nostr relays must be a JSON array".to_string())?;
    let mut seen = BTreeSet::new();
    let mut relays = Vec::new();
    for item in array {
        let relay = item
            .as_str()
            .ok_or_else(|| "Nostr relays must be strings".to_string())?
            .trim();
        let url = url::Url::parse(relay).map_err(|_| "Nostr relays must be valid URLs")?;
        if url.scheme() != "wss" || url.host_str().is_none() {
            return Err("Nostr relays must be wss:// URLs".to_string());
        }
        if seen.insert(relay.to_string()) {
            relays.push(Value::String(relay.to_string()));
        }
    }
    Ok(Value::Array(relays))
}

pub fn normalize_name(value: &str) -> Result<String, String> {
    let name = value.trim().to_ascii_lowercase();
    if name.is_empty()
        || name.len() > 64
        || !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    {
        return Err("Nostr names may use ASCII letters, digits, dash, underscore, and dot".into());
    }
    Ok(name)
}

fn normalize_public_key(value: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.len() == 64 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(trimmed.to_ascii_lowercase());
    }
    decode_npub(trimmed)
}

fn decode_npub(value: &str) -> Result<String, String> {
    let (hrp, data) = bech32_decode(value)?;
    if hrp != "npub" {
        return Err("Nostr public keys must be hex or npub".to_string());
    }
    let bytes = convert_bits(&data, 5, 8)?;
    if bytes.len() != 32 {
        return Err("npub keys must decode to 32 bytes".to_string());
    }
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn bech32_decode(value: &str) -> Result<(String, Vec<u8>), String> {
    let has_lower = value.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = value.chars().any(|c| c.is_ascii_uppercase());
    if has_lower && has_upper {
        return Err("npub keys must not mix letter case".to_string());
    }
    let value = value.to_ascii_lowercase();
    let split = value
        .rfind('1')
        .ok_or_else(|| "npub keys must be valid bech32".to_string())?;
    let (hrp, rest) = value.split_at(split);
    let data_part = &rest[1..];
    if hrp.is_empty() || data_part.len() < 6 {
        return Err("npub keys must be valid bech32".to_string());
    }
    let values = data_part
        .chars()
        .map(|c| {
            CHARSET
                .find(c)
                .map(|index| index as u8)
                .ok_or_else(|| "npub keys must be valid bech32".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let expanded = hrp_expand(hrp)
        .into_iter()
        .chain(values.iter().copied())
        .collect::<Vec<_>>();
    if polymod(&expanded) != 1 {
        return Err("npub checksum is invalid".to_string());
    }
    Ok((hrp.to_string(), values[..values.len() - 6].to_vec()))
}

fn hrp_expand(hrp: &str) -> Vec<u8> {
    let mut expanded = hrp.bytes().map(|b| b >> 5).collect::<Vec<_>>();
    expanded.push(0);
    expanded.extend(hrp.bytes().map(|b| b & 31));
    expanded
}

fn polymod(values: &[u8]) -> u32 {
    let mut chk = 1u32;
    for value in values {
        let top = chk >> 25;
        chk = (chk & 0x1ffffff) << 5 ^ u32::from(*value);
        for (index, generator) in [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3]
            .iter()
            .enumerate()
        {
            if (top >> index) & 1 == 1 {
                chk ^= generator;
            }
        }
    }
    chk
}

fn convert_bits(data: &[u8], from: u32, to: u32) -> Result<Vec<u8>, String> {
    let mut acc = 0u32;
    let mut bits = 0u32;
    let maxv = (1 << to) - 1;
    let mut output = Vec::new();
    for value in data {
        if u32::from(*value) >> from != 0 {
            return Err("npub keys contain invalid bech32 data".to_string());
        }
        acc = (acc << from) | u32::from(*value);
        bits += from;
        while bits >= to {
            bits -= to;
            output.push(((acc >> bits) & maxv) as u8);
        }
    }
    if bits >= from || ((acc << (to - bits)) & maxv) != 0 {
        return Err("npub keys contain invalid padding".to_string());
    }
    Ok(output)
}

fn blank_default<'a>(value: &'a str, default: &'a str) -> &'a str {
    if value.trim().is_empty() {
        default
    } else {
        value
    }
}
