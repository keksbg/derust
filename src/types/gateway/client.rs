use super::builder::IdentifyObject;
use super::cache::{CacheRequest, Caches, CacheType, CacheAction};
use tokio::sync::{mpsc, mpsc::Receiver, oneshot, Barrier};
use tokio::runtime::Builder;
use tracing::{trace};
use tokio_tungstenite::{tungstenite::Message, connect_async};
use crate::types::gateway::payloads::{GatewayPayload, ReadyObject};
use crate::types::gateway::opcodes::OPCode;
use tokio::time::{delay_for, Duration};
use miniz_oxide::inflate::decompress_to_vec;
use futures_util::{SinkExt, stream::StreamExt};
use std::sync::Arc;
use super::payloads::{GatewayMessages, DiscordEvent, GatewayPayloadObjects, GuildType};

pub struct Client {
    url: String,
    heartbeat_interval: Option<u64>,
    session_id: Option<String>,
    seq: Option<u64>,
    identify_object: &'static mut IdentifyObject,
    ready_object: Option<ReadyObject>,
    reqwest_client: reqwest::Client,
    #[cfg(feature = "cache")]
    cache: Caches,
}

#[derive(Debug, Clone, Deserialize)]
struct GatewayUrl {
    pub url: String,
}

impl Client {
    /// Construct a new client object with the
    /// provided [`IdentifyObject`]. By default
    /// the URL will be set to the `GATEWAY_URL`
    /// defined in `lib.rs`.
    ///
    /// If the cache feature is enabled, a simple
    /// `HashMap<Snowflake, TYPE>` with no limits
    /// is constructed. See the [`Cache`] trait if
    /// you'd like to modify it to your needs or disable
    /// it.
    ///
    /// [`Cache`]: types::gateway::cache::Cache
    pub async fn new(mut io: IdentifyObject, caches: Caches) -> Self {        let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(reqwest::header::AUTHORIZATION, 
            reqwest::header::HeaderValue::from_str(io.token.as_str()).unwrap()).unwrap();
        let reqwest_client = reqwest::Client::builder()
            .user_agent("DiscordBot (https://github.com/keksbg/derust, 0.1)")
            .default_headers(headers).build().unwrap();
         Self {
            url: reqwest::get(&(crate::API_URL.to_owned() + "/gateway")).await.unwrap().json::<GatewayUrl>().await.unwrap().url,
            heartbeat_interval: None,
            session_id: None,
            seq: None,
            identify_object: &io,
            ready_object: None,
            #[cfg(feature = "cache")]
            cache: caches,
            reqwest_client
        }
    }

    /// Set the URL to which to send the data. By default
    /// the URL is obtained by requesting it at the API
    /// endpoint `/gateway`. This should be good enough for
    /// most things, but if required you may change it using
    /// this method.
    pub async fn url(&mut self, url: String) -> &mut Self {
        self.url = url;
        self
    }

    /// Launch the websocket connection, returning a tokio `mpsc` receiver.
    /// over which [`DiscordEvent`]s will be dispatched.
    ///
    /// [`DiscordEvent`]: gateway::payloads::DiscordEvent
    /// Handshakes, reconnects, authentication and heartbeats will
    /// be handled by this function and *not* passed to the user.
    //#[instrument(skip(self))]
    pub async fn login(&'static mut self)
                       -> Receiver<DiscordEvent> {
        let rt = Builder::new().threaded_scheduler().build().expect("Failed to start runtime.");
        let (mut tx, mut rx) = mpsc::channel(250);
        let (mut stream, _) = connect_async(&self.url).await.expect("Failed to connect.");
        let barrier = Arc::new(Barrier::new(2));
        let (mut cache_tx, mut cache_rx) = mpsc::channel(125);
        
        // spawn the thread which manages the received
        // messages.
        rt.spawn(async move {
            while let Some(val) = stream.next().await {
                if val.is_ok() {
                    let msg = val.unwrap();
                    if msg.is_text() {
                        let text = msg.to_text().expect("failed to decode message received");
                        let data: GatewayMessages = serde_json::from_str(text).expect("Failed to convert to Rust object");
                        match data {
                            GatewayMessages::Others(v) => {
                                match v.op {
                                    OPCode::Hello => {
                                        match v.d {
                                            GatewayPayloadObjects::Hello(n) => {
                                                self.heartbeat_interval = 
                                                    Some(n.heartbeat_interval);

                                                    stream.send(
                                                        Message::Text(
                                                            serde_json::to_string(self.identify_object).unwrap()
                                                        )
                                                    ).await.expect("failed to send identify object");


                                            },
                                            _ => {}
                                        }
                                    },
                                    _ => handle_ws_payload(v).await,
                                }
                            },
                            GatewayMessages::Ready(r) => {
                                for g in r.guilds {
                                    let (tmp_tx, tmp_rx) = oneshot::channel();
                                    cache_tx.send(
                                        CacheRequest::new(
                                            CacheAction::Push(
                                                (
                                                    g.id, 
                                                    DiscordEvent::GuildCreate(
                                                        GuildType::Partial(g)
                                                    )
                                                )
                                            ),
                                            CacheType::Guild,
                                            tmp_tx
                                        )
                                    ).await.map_err(|_| "failed to write to cache (dead?)").unwrap();
                                    tmp_rx.await.ok().unwrap()
                                }
                            },
                        }
                    } else if msg.is_binary() {
                        let data = msg.into_data();
                        if &data[data.len()-4..] == b"\x00\x00\xff\xff" {
                            let decompressed = 
                            decompress_to_vec(&data).expect("Failed to decompress!");
                            let string = String::from_utf8(decompressed).expect("Failed to convert into string!");
                        }
                    }
                }
            }
        });

        // heartbeat thread
        rt.spawn(async move {
            barrier.wait().await;
                        
        });

        // cache requests thread
        rt.spawn(async move {

        });

        rx
    }
}

async fn handle_ws_payload(msg: GatewayPayload) {
    match msg.op {
        OPCode::Dispatch => {
            match msg.d {
                _ => {
                    
                }
            }
        }
        _ => {}
    }
}
