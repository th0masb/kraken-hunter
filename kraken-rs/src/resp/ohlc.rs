use crate::resp::IntOrDecimal;
use anyhow::Result;
use serde::{de, Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct Ohlc {
    #[serde(rename = "channelId")]
    pub channel_id: u32,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    pub pair: String,
    pub time: String,
    pub etime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub vwap: String,
    pub volume: String,
    pub count: u32,
}

impl<'de> Deserialize<'de> for Ohlc {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        // Delegate the heavy lifting to the internal representation
        let internal = InternalOhlcResponse::deserialize(deserializer)?;

        let data = match &internal.0[1] {
            OhlcResponsePart::Data(d) => Ok(d),
            _ => Err(de::Error::custom("Second component must be ticker data")),
        }?;

        Ok(Ohlc {
            channel_id: match internal.0[0] {
                OhlcResponsePart::UInt(n) => Ok(n),
                _ => Err(de::Error::custom("First component must be channel id")),
            }?,
            time: force_dec::<D>(&data.0[0], "Time component must be decimal")?,
            etime: force_dec::<D>(&data.0[1], "Etime component must be decimal")?,
            open: force_dec::<D>(&data.0[2], "open component must be decimal")?,
            high: force_dec::<D>(&data.0[3], "high component must be decimal")?,
            low: force_dec::<D>(&data.0[4], "low component must be decimal")?,
            close: force_dec::<D>(&data.0[5], "close component must be decimal")?,
            vwap: force_dec::<D>(&data.0[6], "vwap component must be decimal")?,
            volume: force_dec::<D>(&data.0[7], "volume component must be decimal")?,
            count: match &data.0[8] {
                IntOrDecimal::Int(n) => Ok(*n as u32),
                _ => Err(de::Error::custom("count component must be integer")),
            }?,
            channel_name: match &internal.0[2] {
                OhlcResponsePart::Str(s) => Ok(s.clone()),
                _ => Err(de::Error::custom("")),
            }?,
            pair: match &internal.0[3] {
                OhlcResponsePart::Str(s) => Ok(s.clone()),
                _ => Err(de::Error::custom("")),
            }?,
        })
    }
}

fn force_dec<'de, D>(x: &IntOrDecimal, e: &str) -> Result<String, <D as Deserializer<'de>>::Error>
where
    D: Deserializer<'de>,
{
    match x {
        IntOrDecimal::Dec(s) => Ok(s.clone()),
        _ => Err(de::Error::custom(e)),
    }
}

// Internal type used for deserializing the ticker
// update which is an array of different types.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
struct InternalOhlcResponse([OhlcResponsePart; 4]);

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
enum OhlcResponsePart {
    UInt(u32),
    Data(OhlcResponseData),
    Str(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
struct OhlcResponseData([IntOrDecimal; 9]);

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    const VALID_OHLC_RESPONSE: &'static str = r#"
    [
      42,
      [
        "1542057314.748456",
        "1542057360.435743",
        "3586.70001",
        "3586.70000",
        "3586.60001",
        "3586.60000",
        "3586.68894",
        "0.03373000",
        2
      ],
      "ohlc-5",
      "XBT/USD"
    ]"#;

    #[test]
    fn external_success_deserialization() -> Result<()> {
        assert_eq!(
            Ohlc {
                channel_id: 42,
                time: "1542057314.748456".to_owned(),
                etime: "1542057360.435743".to_string(),
                open: "3586.70001".to_string(),
                high: "3586.70000".to_string(),
                low: "3586.60001".to_string(),
                close: "3586.60000".to_string(),
                vwap: "3586.68894".to_string(),
                volume: "0.03373000".to_string(),
                count: 2,
                channel_name: "ohlc-5".to_string(),
                pair: "XBT/USD".to_string()
            },
            serde_json::from_str::<Ohlc>(VALID_OHLC_RESPONSE)?
        );
        Ok(())
    }
}
