use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

pub fn de_u128_from_str<'de, D>(deserializer: D) -> Result<u128, D::Error>
where D: Deserializer<'de>
{
let s = String::deserialize(deserializer)?;
u128::from_str(&s).map_err(de::Error::custom)
}

pub fn de_f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where D: Deserializer<'de>
{
let s = String::deserialize(deserializer)?;
f64::from_str(&s).map_err(de::Error::custom)
}
