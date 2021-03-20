use serde::{Serialize, Serializer};
use serde_derive::Serialize;

/// Kraken Websocket request
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "event")]
pub enum WsReq {
    #[serde(rename = "ping")]
    Ping {
        #[serde(rename = "reqid")]
        #[serde(skip_serializing_if = "Option::is_none")]
        request_id: Option<u32>,
    },
    #[serde(rename = "subscribe")]
    Subscribe {
        #[serde(rename = "reqid")]
        #[serde(skip_serializing_if = "Option::is_none")]
        request_id: Option<u32>,
        pair: Vec<String>,
        subscription: Subscription,
    },
    #[serde(rename = "unsubscribe")]
    Unsubscribe {
        #[serde(rename = "reqid")]
        #[serde(skip_serializing_if = "Option::is_none")]
        request_id: Option<u32>,
        pair: Vec<String>,
        subscription: Subscription,
    },
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "name")]
pub enum Subscription {
    #[serde(rename = "ticker")]
    Ticker,
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "spread")]
    Spread,
    #[serde(rename = "book")]
    Book { depth: BookDepth },
    #[serde(rename = "ohlc")]
    Ohlc { interval: OhlcInterval },
    #[serde(rename = "openOrders")]
    OpenOrders {
        #[serde(rename = "ratecounter")]
        #[serde(skip_serializing_if = "Option::is_none")]
        rate_counter: Option<bool>,
        token: String,
    },
    #[serde(rename = "ownTrades")]
    OwnTrades {
        #[serde(skip_serializing_if = "Option::is_none")]
        snapshot: Option<bool>,
        token: String,
    },
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BookDepth {
    N10,
    N25,
    N100,
    N500,
    N1000,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OhlcInterval {
    Mins1,
    Mins5,
    Mins15,
    Mins30,
    Hours1,
    Hours4,
    Days1,
    Days7,
    Days15,
}

impl Serialize for BookDepth {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            BookDepth::N10 => 10,
            BookDepth::N25 => 25,
            BookDepth::N100 => 100,
            BookDepth::N500 => 500,
            BookDepth::N1000 => 1000,
        })
    }
}

impl Serialize for OhlcInterval {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(match self {
            OhlcInterval::Mins1 => 1,
            OhlcInterval::Mins5 => 5,
            OhlcInterval::Mins15 => 15,
            OhlcInterval::Mins30 => 30,
            OhlcInterval::Hours1 => 60,
            OhlcInterval::Hours4 => 240,
            OhlcInterval::Days1 => 1440,
            OhlcInterval::Days7 => 10080,
            OhlcInterval::Days15 => 21600,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::req::{BookDepth, OhlcInterval, Subscription, WsReq};
    use anyhow::Result;

    #[test]
    fn serialize_owntrades_subscription() -> Result<()> {
        assert_eq!(
            r#"{"event":"subscribe","reqid":13,"pair":["XBT/USD"],"subscription":{"name":"ownTrades","snapshot":true,"token":"abc"}}"#,
            serde_json::to_string(&WsReq::Subscribe {
                request_id: Some(13),
                pair: vec!["XBT/USD".to_string()],
                subscription: Subscription::OwnTrades {
                    snapshot: Some(true),
                    token: "abc".to_string()
                }
            })?
        );
        Ok(())
    }

    #[test]
    fn serialize_openorders_subscription() -> Result<()> {
        assert_eq!(
            r#"{"event":"subscribe","reqid":13,"pair":["XBT/USD"],"subscription":{"name":"openOrders","ratecounter":true,"token":"abc"}}"#,
            serde_json::to_string(&WsReq::Subscribe {
                request_id: Some(13),
                pair: vec!["XBT/USD".to_string()],
                subscription: Subscription::OpenOrders {
                    rate_counter: Some(true),
                    token: "abc".to_string()
                }
            })?
        );
        Ok(())
    }

    #[test]
    fn serialize_ohlc_subscription() -> Result<()> {
        assert_eq!(
            r#"{"event":"subscribe","reqid":13,"pair":["XBT/USD"],"subscription":{"name":"ohlc","interval":30}}"#,
            serde_json::to_string(&WsReq::Subscribe {
                request_id: Some(13),
                pair: vec!["XBT/USD".to_string()],
                subscription: Subscription::Ohlc {
                    interval: OhlcInterval::Mins30
                }
            })?
        );
        Ok(())
    }

    #[test]
    fn serialize_spread_subscription() -> Result<()> {
        assert_eq!(
            r#"{"event":"subscribe","reqid":13,"pair":["XBT/USD","XBT/ETH"],"subscription":{"name":"spread"}}"#,
            serde_json::to_string(&WsReq::Subscribe {
                request_id: Some(13),
                pair: vec!["XBT/USD".to_string(), "XBT/ETH".to_string()],
                subscription: Subscription::Spread
            })?
        );
        Ok(())
    }

    #[test]
    fn serialize_trade_subscription() -> Result<()> {
        assert_eq!(
            r#"{"event":"subscribe","pair":["XBT/USD"],"subscription":{"name":"trade"}}"#,
            serde_json::to_string(&WsReq::Subscribe {
                request_id: None,
                pair: vec!["XBT/USD".to_string()],
                subscription: Subscription::Trade
            })?
        );
        assert_eq!(
            r#"{"event":"subscribe","reqid":13,"pair":["XBT/USD","XBT/ETH"],"subscription":{"name":"trade"}}"#,
            serde_json::to_string(&WsReq::Subscribe {
                request_id: Some(13),
                pair: vec!["XBT/USD".to_string(), "XBT/ETH".to_string()],
                subscription: Subscription::Trade
            })?
        );
        Ok(())
    }

    #[test]
    fn serialize_ticker_subscription() -> Result<()> {
        assert_eq!(
            r#"{"event":"subscribe","pair":["XBT/USD"],"subscription":{"name":"ticker"}}"#,
            serde_json::to_string(&WsReq::Subscribe {
                request_id: None,
                pair: vec!["XBT/USD".to_string()],
                subscription: Subscription::Ticker
            })?
        );
        assert_eq!(
            r#"{"event":"subscribe","reqid":13,"pair":["XBT/USD","XBT/ETH"],"subscription":{"name":"ticker"}}"#,
            serde_json::to_string(&WsReq::Subscribe {
                request_id: Some(13),
                pair: vec!["XBT/USD".to_string(), "XBT/ETH".to_string()],
                subscription: Subscription::Ticker
            })?
        );
        Ok(())
    }

    #[test]
    fn serialize_book_subscription() -> Result<()> {
        assert_eq!(
            r#"{"event":"subscribe","pair":["XBT/USD"],"subscription":{"name":"book","depth":25}}"#,
            serde_json::to_string(&WsReq::Subscribe {
                request_id: None,
                pair: vec!["XBT/USD".to_string()],
                subscription: Subscription::Book {
                    depth: BookDepth::N25
                }
            })?
        );
        assert_eq!(
            r#"{"event":"subscribe","reqid":13,"pair":["XBT/USD","XBT/ETH"],"subscription":{"name":"book","depth":10}}"#,
            serde_json::to_string(&WsReq::Subscribe {
                request_id: Some(13),
                pair: vec!["XBT/USD".to_string(), "XBT/ETH".to_string()],
                subscription: Subscription::Book {
                    depth: BookDepth::N10
                }
            })?
        );
        Ok(())
    }

    #[test]
    fn serialize_ping() -> Result<()> {
        assert_eq!(
            r#"{"event":"ping"}"#,
            serde_json::to_string(&WsReq::Ping { request_id: None })?
        );
        assert_eq!(
            r#"{"event":"ping","reqid":5}"#,
            serde_json::to_string(&WsReq::Ping {
                request_id: Some(5)
            })?
        );
        Ok(())
    }

    #[test]
    fn serialize_depth() -> Result<()> {
        assert_eq!("10", serde_json::to_string(&BookDepth::N10)?);
        assert_eq!("25", serde_json::to_string(&BookDepth::N25)?);
        assert_eq!("100", serde_json::to_string(&BookDepth::N100)?);
        assert_eq!("500", serde_json::to_string(&BookDepth::N500)?);
        assert_eq!("1000", serde_json::to_string(&BookDepth::N1000)?);
        Ok(())
    }

    #[test]
    fn serialize_interval() -> Result<()> {
        assert_eq!("1", serde_json::to_string(&OhlcInterval::Mins1)?);
        assert_eq!("5", serde_json::to_string(&OhlcInterval::Mins5)?);
        assert_eq!("15", serde_json::to_string(&OhlcInterval::Mins15)?);
        assert_eq!("30", serde_json::to_string(&OhlcInterval::Mins30)?);
        assert_eq!("60", serde_json::to_string(&OhlcInterval::Hours1)?);
        assert_eq!("240", serde_json::to_string(&OhlcInterval::Hours4)?);
        assert_eq!("1440", serde_json::to_string(&OhlcInterval::Days1)?);
        assert_eq!("10080", serde_json::to_string(&OhlcInterval::Days7)?);
        assert_eq!("21600", serde_json::to_string(&OhlcInterval::Days15)?);
        Ok(())
    }
}
