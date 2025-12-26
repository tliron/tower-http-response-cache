use {
    http::*,
    std::{sync::*, time::*},
};

/// Hook to get a response's cache duration.
pub type CacheDurationHook =
    Arc<Box<dyn Fn(CacheDurationHookContext) -> Option<Duration> + Send + Sync>>;

//
// CacheDurationHookContext
//

/// Context for [CacheDurationHook].
pub struct CacheDurationHookContext<'this> {
    /// URI.
    pub uri: &'this Uri,

    /// Headers.
    pub headers: &'this HeaderMap,
}

impl<'this> CacheDurationHookContext<'this> {
    /// Constructor.
    pub fn new(uri: &'this Uri, headers: &'this HeaderMap) -> Self {
        Self { uri, headers }
    }
}
