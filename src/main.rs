use websocket::{ClientBuilder, Message};
use anyhow::{Result, Error, anyhow};
use websocket::ws::dataframe::DataFrame;
use bytes::Bytes;

fn main() -> Result<()> {
    let public_endpoint = "wss://ws.kraken.com";
    let mut client = ClientBuilder::new(public_endpoint)?
        .connect(None)?;

    client.send_message(&Message::text(r#"
        {
            "event": "subscribe",
            "pair": [
                "XBT/USD"
            ],
            "subscription": {
                "name": "ticker"
            }
        }"#))?;

    for i in 0..10 {
        let message = Bytes::from(client.recv_message()?.take_payload());
        println!("{} {:?}", i, message)
    }
    Ok(())
}
