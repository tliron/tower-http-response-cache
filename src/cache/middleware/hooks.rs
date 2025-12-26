use {http::request::*, http::*, kutil::transcoding::*, std::sync::*};

/// Hook to check if a request or a response is cacheable.
pub type CacheableHook = Arc<Box<dyn Fn(CacheableHookContext) -> bool + Send + Sync>>;

/// Hook to check if a request or a response is encodable.
pub type EncodableHook = Arc<Box<dyn Fn(EncodableHookContext) -> bool + Send + Sync>>;

/// Hook to update a request's cache key.
pub type CacheKeyHook<CacheKeyT, RequestBodyT> =
    Arc<Box<dyn Fn(CacheKeyHookContext<CacheKeyT, RequestBodyT>) + Send + Sync>>;

//
// CacheableHookContext
//

/// Context for [CacheableHook].
#[derive(Clone, Debug)]
pub struct CacheableHookContext<'this> {
    /// URI.
    pub uri: &'this Uri,

    /// Headers.
    pub headers: &'this HeaderMap,
}

impl<'this> CacheableHookContext<'this> {
    /// Constructor.
    pub fn new(uri: &'this Uri, headers: &'this HeaderMap) -> Self {
        Self { uri, headers }
    }
}

//
// EncodableHookContext
//

/// Context for [EncodableHook].
#[derive(Clone, Debug)]
pub struct EncodableHookContext<'this> {
    /// Encoding.
    pub encoding: &'this Encoding,

    /// URI.
    pub uri: &'this Uri,

    /// Headers.
    pub headers: &'this HeaderMap,
}

impl<'this> EncodableHookContext<'this> {
    /// Constructor.
    pub fn new(encoding: &'this Encoding, uri: &'this Uri, headers: &'this HeaderMap) -> Self {
        Self {
            encoding,
            uri,
            headers,
        }
    }
}

//
// CacheKeyHookContext
//

/// Context for [CacheKeyHook].
#[derive(Debug)]
pub struct CacheKeyHookContext<'this, CacheKeyT, RequestBodyT> {
    /// Cache key.
    pub cache_key: &'this mut CacheKeyT,

    /// Request.
    pub request: &'this Request<RequestBodyT>,
}

impl<'this, CacheKeyT, RequestBodyT> CacheKeyHookContext<'this, CacheKeyT, RequestBodyT> {
    /// Constructor.
    pub fn new(cache_key: &'this mut CacheKeyT, request: &'this Request<RequestBodyT>) -> Self {
        Self { cache_key, request }
    }
}
