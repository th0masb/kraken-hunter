mod req;
mod resp;

use crate::req::{OhlcInterval, Subscription, WsReq};
use anyhow::{Error, Result};
use resp::ticker::TickerState;
use websocket::client::sync::Client;
use websocket::websocket_base::stream::sync::NetworkStream;
use websocket::ws::dataframe::DataFrame;
use websocket::{ClientBuilder, Message, OwnedMessage};

const ENDPOINT: &'static str = "wss://ws.kraken.com";

pub struct Kraken {
    inner: Client<Box<dyn NetworkStream + Send>>,
}

impl Kraken {
    pub fn new() -> Result<Kraken> {
        Ok(Kraken {
            inner: ClientBuilder::new(ENDPOINT)?.connect(None)?,
        })
    }

    pub fn send_req(&mut self, req: WsReq) -> Result<()> {
        self.inner
            .send_message(&Message::text(serde_json::to_string(&req)?))
            .map_err(Error::from)
    }
}

fn main() -> Result<()> {
    let mut client = Kraken::new()?;
    client.send_req(WsReq::Ping {
        request_id: Some(10),
    })?;
    println!(
        "{}",
        String::from_utf8(client.inner.recv_message()?.take_payload())?
    );
    println!(
        "{}",
        String::from_utf8(client.inner.recv_message()?.take_payload())?
    );

    client.send_req(WsReq::Subscribe {
        request_id: Some(12),
        pair: vec!["ETH/USD".to_string()],
        subscription: Subscription::Ohlc {
            interval: OhlcInterval::Mins15,
        },
    })?;
    client.send_req(WsReq::Subscribe {
        request_id: Some(12),
        pair: vec!["ETH/USD".to_string()],
        subscription: Subscription::Ticker,
    })?;

    for message in client
        .inner
        .incoming_messages()
        .filter_map(|x| x.ok())
        .filter_map(|x| match x {
            OwnedMessage::Text(s) => serde_json::from_str::<resp::Resp>(s.as_str()).ok(),
            _ => None,
        })
    {
        match message {
            resp::Resp::Ticker(TickerState { channel_id: _, .. }) => {}
            _ => {}
        }
        println!("{:?}", message)
    }

    client.send_req(WsReq::Unsubscribe {
        request_id: Some(12),
        pair: vec!["ETH/USD".to_string()],
        subscription: Subscription::Ticker,
    })?;

    Ok(())
}
