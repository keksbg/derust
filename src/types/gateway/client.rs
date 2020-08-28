use super::builder::IdentifyObject;
use super::cache::Caches;

pub struct Client {
    url: String,
    heartbeat_interval: Option<u32>,
    session_id: Option<String>,
    seq: Option<u32>,
    identify_object: &'static mut IdentifyObject,
    #[cfg(feature = "cache")]
    cache: Caches,
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
    pub async fn new(io: &'static mut IdentifyObject, caches: Caches) -> Self {
        Self {
            url: String::from(crate::GATEWAY_URL),
            heartbeat_interval: None,
            session_id: None,
            seq: None,
            identify_object: io,
            #[cfg(feature = "cache")]
            cache: caches,
        }
    }

    /// Set the URL to which to send the data. By default
    /// the URL is defined as a `static` in src/lib.rs
    /// under the name `GATEWAY_URL`, but if you'd like
    /// to change it to something else you can use
    /// this function.
    pub async fn url(&mut self, url: String) -> &mut Self {
        self.url = url;
        self
    }
}

