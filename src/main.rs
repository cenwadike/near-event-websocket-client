use env_logger;
use serde_json::json;
use tokio_tungstenite::tungstenite::{connect, Message};
use url::Url;


pub fn main() {
    env_logger::init();
    // Pagoda open near event websocket address
    let addr = "wss://events.near.stream/ws";

    // test event filter
    let event_filter = json!({
        "filter": [{
            "account_id": "nft.nearapps.near",
            "status": "SUCCESS",
            "event": {
            "standard": "nep171",
            "event": "nft_mint",
            }
        }],
    });

    // initial message seeds the event filter
    let init_msg = json!({
        "secret": "ohyeahnftsss",
        "filter": event_filter,
        "fetch_past_events": 20,
    });

    // attempt websocket connection
    let (mut socket, response) = connect(Url::parse(addr).unwrap())
        .expect("Failed to connect to Pagoda's mainnet event websocket");

    // assert connection was successful
    if response.status().is_success() {
        // send initialization message
        socket
            .write_message(Message::Text(init_msg.to_string()))
            .unwrap();

        // continous read event streams
        loop {
            socket
                .read_message()
                .expect("Error: Failed to read event from websocket");
        }
    } else {
        // terminate socket connection
        socket.close(None).unwrap()
    }
}
