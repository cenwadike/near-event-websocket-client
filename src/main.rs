use env_logger;
use serde_json::json;
use tungstenite::{connect, Message};
use url::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    pub events: Vec<InnerEvent>,
    pub secret: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InnerEvent {
    pub account_id: String,
    pub block_hash: String,
    pub block_height: i64,
    pub block_timestamp_ms: f64,
    pub block_timestamp_ns: String,
    pub event: EventLog,
    pub log_index: i64,
    pub predecessor_id: String,
    pub receipt_id: String,
    pub shard_id: i64,
    pub signer_id: String,
    pub signer_public_key: String,
    pub status: String,
    pub tx_hash: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EventLog {
    pub data: Vec<EventLogData>,
    pub event: String,
    pub standard: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventLogData {
    pub amount: String,
    pub memo: String,
    #[serde(rename = "new_owner_id")]
    pub new_owner_id: String,
    #[serde(rename = "old_owner_id")]
    pub old_owner_id: String,
}

pub fn main() {
    env_logger::init();
    println!("Running..........................");
    // Pagoda open near event websocket address
    let addr = "wss://events.near.stream/ws";

    let contract_address = "dac17f958d2ee523a2206206994597c13d831ec7.factory.bridge.near";
    let user_id = "claim.sweat";
    // initial message seeds the event filter
    let init_msg = json!({
        "secret": "secret",
        "account_id": contract_address,
        "filter": [
            {
                "event": {
                    "standard": "nep141",
                    "event": "ft_transfer",
                    "data":[{
                        "old_owner_id":user_id,
                    }]
                },
            }
        ],
        "status":"SUCCESS",
        "fetch_past_events": 1,
    });

    // attempt websocket connection
    let (mut socket, response) = connect(Url::parse(addr).unwrap())
        .expect("Failed to connect to Pagoda's mainnet event websocket");
    println!("Connected to websocket sever...........................");
    // assert connection was successful
    if !response.status().is_server_error() {
        // send initialization message
        socket
            .write_message(Message::Text(init_msg.to_string()))
            .unwrap();

        // continuously read event streams
        loop {
            let msg = socket.read_message();

            if msg.is_err() {
                println!("Event returned an error")
            } else {
                let raw_event = serde_json::from_str::<Event>(msg.unwrap().to_string().as_str()).expect("JSON was not well-formatted");
                let inner_event = raw_event.events.into_iter();
                for event in inner_event {
                    let event_data = event.event.data;
                    for data in event_data {
                        println!("NEAR event data: {:?}", data);
                    }
                }
            }
        }
    } else {
        // terminate socket connection
        socket.close(None).unwrap()
    }
}
