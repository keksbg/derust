use std::sync::Arc;
use crate::types::guild::{Guild, GuildMember};
use crate::types::user::User;
use crate::types::voice::VoiceState;
use crate::types::channel::Channel;
use std::collections::HashMap;
use crate::types::{CachedTypes, Snowflake};

pub struct Caches {
    guild_cache: Option<Arc<dyn Cache<Guild>>>,
    user_cache: Option<Arc<dyn Cache<User>>>,
    member_cache: Option<Arc<dyn Cache<GuildMember>>>,
    voice_state_cache: Option<Arc<dyn Cache<VoiceState>>>,
    channel_cache: Option<Arc<dyn Cache<Channel>>>,
}

impl Caches {
    pub async fn initialize() -> Self {
        Self {
            guild_cache: Some(Arc::new(HashMap::new())),
            user_cache: Some(Arc::new(HashMap::new())),
            member_cache: Some(Arc::new(HashMap::new())),
            voice_state_cache: Some(Arc::new(HashMap::new())),
            channel_cache: Some(Arc::new(HashMap::new())),
        }
    }
    /// Supply your own object that implements [`Cache`] to the struct and add it,
    /// or supply `None` for the data to not be cached at all.
    ///
    /// Default: `HashMap<Snowflake, Guild>` with no limits.
    pub async fn guild_cache(
        &mut self,
        cache: Option<Arc<dyn Cache<Guild>>>
    ) -> &mut Self
    {
        self.guild_cache = cache;
        self
    }

    /// Same as [`guild_cache`](method.guild_cache) above.
    ///
    /// Default: `HashMap<Snowflake, User>` with no limits.
    pub async fn user_cache(
        &mut self,
        cache: Option<Arc<dyn Cache<User>>>
    ) -> &mut Self {
        self.user_cache = cache;
        self
    }

    /// Same as [`guild_cache`](method.guild_cache).
    ///
    /// Default: `HashMap<Snowflake, GuildMember>` with no limits.
    pub async fn member_cache(
        &mut self,
        cache: Option<Arc<dyn Cache<GuildMember>>>
    ) -> &mut Self {
        self.member_cache = cache;
        self
    }

    /// Same as [`guild_cache`](method.guild_cache).
    ///
    /// Default: `HashMap<Snowflake, VoiceState>` with no limits.
    pub async fn voice_state_cache(
        &mut self,
        cache: Option<Arc<dyn Cache<VoiceState>>>
    ) -> &mut Self {
        self.voice_state_cache = cache;
        self
    }

    /// Same as [`guild_cache`](method.guild_cache).
    ///
    /// Default: `HashMap<Snowflake, Channel>` with no limits.
    pub async fn channel_cache(
        &mut self,
        cache: Option<Arc<dyn Cache<Channel>>>
    ) -> &mut Self {
        self.channel_cache = cache;
        self
    }
}

/// The base trait for your cache. By default, it is
/// implemented on `HashMap` but it can be implemented
/// on additional objects of your choice, possibly even
/// using a database like Redis to cache.
///
/// An example implementation would be something like:
/// ```
/// use derust::types::gateway::client::Cache;
/// use std::collections::HashMap;
/// use derust::types::{Snowflake, CachedTypes};
/// use derust::types::gateway::cache::Cache;
///
/// impl<V> Cache<V> for HashMap<Snowflake, V>
///     where V: CachedTypes + Send {
///     fn get(&self, k: Snowflake) -> Option<&V> {
///         self.get(&k)
///     }
///
///     fn push(&mut self, k: Snowflake, v: V) -> Option<V> {
///         self.insert(k, v)
///     }
///
///     fn remove(&mut self, k: Snowflake) -> Option<V> {
///         self.remove(&k)
///     }
///
///     fn clear(&mut self) {
///         self.drain();
///         self.shrink_to_fit()
///     }
///
///     fn len(&self) -> usize {
///         self.len()
///     }
/// }
/// ```
/// (this is the exact implementation used by us for
/// our standard `HashMap` Cache implementation)
#[cfg(feature = "cache")]
pub trait Cache<C: CachedTypes> {
    fn get(&self, k: Snowflake) -> Option<&C>;
    fn push(&mut self, k: Snowflake, v: C) -> Option<C>;
    fn remove(&mut self, k: Snowflake) -> Option<C>;
    fn clear(&mut self);
    fn len(&self) -> usize;
}

#[cfg(feature = "cache")]
impl<V> Cache<V> for HashMap<Snowflake, V>
    where V: CachedTypes + Send {
    fn get(&self, k: Snowflake) -> Option<&V> {
        self.get(&k)
    }

    fn push(&mut self, k: Snowflake, v: V) -> Option<V> {
        self.insert(k, v)
    }

    fn remove(&mut self, k: Snowflake) -> Option<V> {
        self.remove(&k)
    }

    fn clear(&mut self) {
        self.drain();
        self.shrink_to_fit()
    }

    fn len(&self) -> usize {
        self.len()
    }
}