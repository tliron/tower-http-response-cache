[![crates.io](https://img.shields.io/crates/v/tower-http-response-cache?color=%23227700)](https://crates.io/crates/tower-http-response-cache)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/tower-http-response-cache)

HTTP Response Cache for Tower
=============================

HTTP response caching middleware with integrated compression for [Tower](https://github.com/tower-rs/tower) and [axum](https://github.com/tokio-rs/axum).

Please check out the [included examples](https://github.com/tliron/tower-http-response-cache/tree/main/examples)!

Rationale
---------

Though you can rely on an external caching solution instead (e.g. a reverse proxy), there are good reasons to integrate the cache directly into your application. For one, direct access allows for an in-process in-memory cache, which is optimal for the first caching tier.

This solution is especially optimized in that, by handling both caching and compression *together*, it is able to cache and intelligently reuse compressed versions of responses.

For example, imagine that a request comes in for a GZip version of a resource. We encode to GZip and cache that version (and optionally cache the original pre-encoded version). Subsequent requests for GZip will use the cached version as is without having to reencode it. So far, this is a common "low-hanging fruit" optimization. Now, imagine that a new request comes in for the same resource but as Zstandard encoding instead of GZip. Because our compression layer is cache-aware we can either 1) encode to Zstandard from the cached pre-encoded version (if we stored it), or 2) decode from GZip and then encode to Zstandard (and add that to the cache). Crucially, as long as *any* encoding of the resource exists in the cache then this middleware will fully handle the request without calling the upstream.

Far too many web servers ignore this optimization opportunity and waste compute resources by reencoding data that has not changed. Unfortunately, it's impossible to achieve this optimization if your caching middleware and compression middleware are not integrated.

This middleware also participates in client-side caching (conditional HTTP). A cache hit will respect the client's `If-None-Match` and `If-Modified-Since` headers and return a 304 (Not Modified) when appropriate. Even though we have the cached response, this will save on both bandwidth and client compute resources.

Features
--------

The web's most common compression formats are supported and can be enabled via crate features: Brotli, Deflate, GZip, and Zstandard. The best encoding is selected by comparing the server and client's preferences (HTTP content negotiation).

Plug in your own cache by implementing a trait. Access to all cache functions is `async`. (Note that concurrent performance will depend on the actual cache implementation, the HTTP server, and of course your async runtime).

Note that even though [Tokio](https://github.com/tokio-rs/tokio) I/O types are used internally, this middleware does *not* require a specific async runtime.

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/tower-http-response-cache/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/tower-http-response-cache/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
