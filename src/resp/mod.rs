pub mod ohlc;
pub mod ticker;

use crate::resp::ohlc::Ohlc;
use crate::resp::ticker::TickerState;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Resp {
    Ticker(TickerState),
    Ohlc(Ohlc),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
enum IntOrDecimal {
    Int(u64),
    Dec(String),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::resp::ticker::{BidAskData, ValueMarker};
    use anyhow::Result;

    #[test]
    fn ticker_deserialization() -> Result<()> {
        assert_eq!(
            Resp::Ticker(TickerState {
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
            }),
            serde_json::from_str::<Resp>(
                r#"[
              0,
              {
                "a": ["5525.40000", 1, "1.000"],
                "b": ["5525.10000", 1, "1.000"],
                "c": ["5525.10000", "0.00398963"],
                "h": ["5783.00000", "5783.00000"],
                "l": ["5505.00000","5505.00000"],
                "o": ["5760.70000", "5763.40000"],
                "p": ["5631.44067", "5653.78939"],
                "t": [11493, 16267],
                "v": ["2634.11501494", "3591.17907851"]
              },
              "ticker",
              "XBT/USD"
            ]"#
            )?
        );
        Ok(())
    }

    #[test]
    fn ohlc_deserialization() -> Result<()> {
        assert_eq!(
            Resp::Ohlc(Ohlc {
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
            }),
            serde_json::from_str::<Resp>(
                r#"
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
              ]"#
            )?
        );
        Ok(())
    }
}
