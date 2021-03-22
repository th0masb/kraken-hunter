use crate::resp::IntOrDecimal;
use anyhow::{anyhow, Result};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Information about the ticker for a particular
/// pair at a given point in time.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct TickerState {
    #[serde(rename = "channelId")]
    pub channel_id: u32,
    pub pair: String,
    pub ask: BidAskData,
    pub bid: BidAskData,
    pub close: ValueMarker<String>,
    pub volume: ValueMarker<String>,
    #[serde(rename = "volumeWeightedAvgPrice")]
    pub volume_weighted_avg_price: ValueMarker<String>,
    #[serde(rename = "tradeCount")]
    pub trade_count: ValueMarker<u32>,
    #[serde(rename = "lowPrice")]
    pub low_price: ValueMarker<String>,
    #[serde(rename = "highPrice")]
    pub high_price: ValueMarker<String>,
    #[serde(rename = "openPrice")]
    pub open_price: ValueMarker<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct BidAskData {
    pub price: String,
    #[serde(rename = "wholeLotVolume")]
    pub whole_lot_volume: u64,
    #[serde(rename = "lotVolume")]
    pub lot_volume: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct ValueMarker<T>
where
    T: Debug + Clone + Eq + PartialEq + Hash + Serialize,
{
    pub today: T,
    #[serde(rename = "last24h")]
    pub last_24h: T,
}

impl BidAskData {
    fn try_from(src: &[IntOrDecimal; 3]) -> Result<BidAskData> {
        Ok(BidAskData {
            price: match &src[0] {
                IntOrDecimal::Dec(s) => Ok(s.clone()),
                _ => Err(anyhow!("First element in bid/ask must be decimal price")),
            }?,
            whole_lot_volume: match &src[1] {
                IntOrDecimal::Int(n) => Ok(*n),
                _ => Err(anyhow!(
                    "Second element in bid/ask must be int whole lot vol"
                )),
            }?,
            lot_volume: match &src[2] {
                IntOrDecimal::Dec(s) => Ok(s.clone()),
                _ => Err(anyhow!("Second element in bid/ask must be decimal lot vol")),
            }?,
        })
    }
}

impl<T> ValueMarker<T>
where
    T: Debug + Clone + Eq + PartialEq + Hash + Serialize,
{
    fn from(src: &[T; 2]) -> ValueMarker<T> {
        ValueMarker {
            today: src[0].clone(),
            last_24h: src[1].clone(),
        }
    }
}

impl<'de> Deserialize<'de> for TickerState {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        // Delegate the heavy lifting to the internal representation
        let internal = InternalTickerResponse::deserialize(deserializer)?;

        let data = match &internal.0[1] {
            TickerResponsePart::Data(d) => Ok(d),
            _ => Err(de::Error::custom("Second component must be ticker data")),
        }?;

        Ok(TickerState {
            channel_id: match internal.0[0] {
                TickerResponsePart::UInt(n) => Ok(n),
                _ => Err(de::Error::custom("First component must be channel id")),
            }?,
            pair: match &internal.0[3] {
                TickerResponsePart::Str(s) => Ok(s.clone()),
                _ => Err(de::Error::custom("Last component must be the pair")),
            }?,
            ask: BidAskData::try_from(&data.ask)
                .map_err(|e| de::Error::custom(format!("{}", e)))?,
            bid: BidAskData::try_from(&data.bid)
                .map_err(|e| de::Error::custom(format!("{}", e)))?,
            close: ValueMarker::from(&data.close),
            volume: ValueMarker::from(&data.volume),
            volume_weighted_avg_price: ValueMarker::from(&data.volume_weighted_avg_price),
            trade_count: ValueMarker::from(&data.trade_count),
            low_price: ValueMarker::from(&data.low_price),
            high_price: ValueMarker::from(&data.high_price),
            open_price: ValueMarker::from(&data.open_price),
        })
    }
}

// Internal type used for deserializing the ticker
// update which is an array of different types.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
struct InternalTickerResponse([TickerResponsePart; 4]);

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
enum TickerResponsePart {
    UInt(u32),
    Data(TickerResponseData),
    Str(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
struct TickerResponseData {
    #[serde(rename = "a")]
    ask: [IntOrDecimal; 3],
    #[serde(rename = "b")]
    bid: [IntOrDecimal; 3],
    #[serde(rename = "c")]
    close: [String; 2],
    #[serde(rename = "v")]
    volume: [String; 2],
    #[serde(rename = "p")]
    volume_weighted_avg_price: [String; 2],
    #[serde(rename = "t")]
    trade_count: [u32; 2],
    #[serde(rename = "l")]
    low_price: [String; 2],
    #[serde(rename = "h")]
    high_price: [String; 2],
    #[serde(rename = "o")]
    open_price: [String; 2],
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    const VALID_TICKER_RESPONSE: &'static str = r#"[
      0,
      {
        "a": ["5525.40000", 1,  "1.000"],
        "b": ["5525.10000", 1,  "1.000"],
        "c": ["5525.10000", "0.00398963"],
        "h": ["5783.00000", "5783.00000"],
        "l": ["5505.00000", "5505.00000"],
        "o": ["5760.70000", "5763.40000"],
        "p": ["5631.44067", "5653.78939"],
        "t": [11493, 16267],
        "v": ["2634.11501494", "3591.17907851"]
      },
      "ticker",
      "XBT/USD"
    ]"#;

    #[test]
    fn external_success_deserialization() -> Result<()> {
        assert_eq!(
            TickerState {
                channel_id: 0,
                pair: format!("XBT/USD"),
                ask: BidAskData {
                    price: format!("5525.40000"),
                    whole_lot_volume: 1,
                    lot_volume: format!("1.000")
                },
                bid: BidAskData {
                    price: format!("5525.10000"),
                    whole_lot_volume: 1,
                    lot_volume: format!("1.000")
                },
                close: ValueMarker {
                    today: format!("5525.10000"),
                    last_24h: format!("0.00398963")
                },
                high_price: ValueMarker {
                    today: format!("5783.00000"),
                    last_24h: format!("5783.00000")
                },
                low_price: ValueMarker {
                    today: format!("5505.00000"),
                    last_24h: format!("5505.00000")
                },
                open_price: ValueMarker {
                    today: format!("5760.70000"),
                    last_24h: format!("5763.40000")
                },
                volume_weighted_avg_price: ValueMarker {
                    today: format!("5631.44067"),
                    last_24h: format!("5653.78939")
                },
                trade_count: ValueMarker {
                    today: 11493,
                    last_24h: 16267
                },
                volume: ValueMarker {
                    today: format!("2634.11501494"),
                    last_24h: format!("3591.17907851")
                }
            },
            serde_json::from_str::<TickerState>(VALID_TICKER_RESPONSE)?
        );
        Ok(())
    }

    #[test]
    fn internal_success_deserialization() -> Result<()> {
        assert_eq!(
            InternalTickerResponse([
                TickerResponsePart::UInt(0),
                TickerResponsePart::Data(TickerResponseData {
                    ask: [
                        IntOrDecimal::Dec(format!("5525.40000")),
                        IntOrDecimal::Int(1),
                        IntOrDecimal::Dec(format!("1.000"))
                    ],
                    bid: [
                        IntOrDecimal::Dec(format!("5525.10000")),
                        IntOrDecimal::Int(1),
                        IntOrDecimal::Dec(format!("1.000"))
                    ],
                    close: [format!("5525.10000"), format!("0.00398963"),],
                    high_price: [format!("5783.00000"), format!("5783.00000"),],
                    low_price: [format!("5505.00000"), format!("5505.00000"),],
                    open_price: [format!("5760.70000"), format!("5763.40000"),],
                    volume_weighted_avg_price: [format!("5631.44067"), format!("5653.78939"),],
                    trade_count: [11493, 16267],
                    volume: [format!("2634.11501494"), format!("3591.17907851"),]
                }),
                TickerResponsePart::Str(format!("ticker")),
                TickerResponsePart::Str(format!("XBT/USD")),
            ]),
            serde_json::from_str::<InternalTickerResponse>(VALID_TICKER_RESPONSE)?
        );
        Ok(())
    }
}
