/// Deserializers that accept both numbers and strings for numeric types.
/// MCP clients (e.g. Claude) sometimes send integers as JSON strings.
use serde::{Deserialize, Deserializer};

pub fn u32_from_str_or_num<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u32>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Val {
        Num(u32),
        Str(String),
    }
    Option::<Val>::deserialize(d)?
        .map(|v| match v {
            Val::Num(n) => Ok(n),
            Val::Str(s) => s.parse::<u32>().map_err(serde::de::Error::custom),
        })
        .transpose()
}

pub fn i32_from_str_or_num<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i32>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Val {
        Num(i32),
        Str(String),
    }
    Option::<Val>::deserialize(d)?
        .map(|v| match v {
            Val::Num(n) => Ok(n),
            Val::Str(s) => s.parse::<i32>().map_err(serde::de::Error::custom),
        })
        .transpose()
}

pub fn f64_from_str_or_num<'de, D: Deserializer<'de>>(d: D) -> Result<Option<f64>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Val {
        Num(f64),
        Str(String),
    }
    Option::<Val>::deserialize(d)?
        .map(|v| match v {
            Val::Num(n) => Ok(n),
            Val::Str(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
        })
        .transpose()
}
