[![crates.io](https://img.shields.io/crates/v/tower-http-response-cache?color=%23227700)](https://crates.io/crates/tower-http-response-cache)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/tower-http-response-cache)

HTTP Cache for Tower
====================

HTTP response caching middleware with integrated compression for [axum](https://github.com/tokio-rs/axum) and other [Tower](https://github.com/tower-rs/tower)-compatible server stacks.

Please check out the [included examples](https://github.com/tliron/tower-http-response-cache/tree/main/examples), which are easy to run and test locally.

Rationale
---------

Though you can rely on an external caching solution instead (functioning as a reverse proxy), there are good reasons to integrate the cache directly into your application. For one, this integration allows for an in-process-memory cache, which is optimal for the first caching tier.

This middleware is particularly optimized in that, by handling both caching and compression *together*, it is able to intelligently reuse differently compressed versions of responses.

For example, imagine that a request comes in for a GZip version of a resource. This middleware will encode to GZip and cache that version (and optionally cache the original pre-encoded version, depending on how you configure it). Subsequent requests for GZip will use the cached version as is without having to reencode it. So far, this is a common "low-hanging fruit" optimization, which you can achieve by putting a cache *after* compression middleware.

Now, imagine that a new request comes in for the same resource but as Zstandard encoding instead of GZip. Because our compression layer is cache-aware we can either 1) encode to Zstandard from the cached pre-encoded version (if we stored it), or 2) decode from GZip and then encode to Zstandard (and add that to the cache). Crucially, as long as *any* version of the resource exists in the cache then this middleware will be able to fully handle the request without calling the upstream.

Far too many web servers ignore this optimization opportunity and waste compute resources by reencoding data that has not changed. Unfortunately, it's impossible to achieve this optimization if your caching middleware and compression middleware are not designed to work together.

This middleware also participates in client-side caching, a.k.a. conditional HTTP. A cache hit will respect the client's `If-None-Match` and `If-Modified-Since` headers (for ETags and modification dates respectively) and return a 304 (Not Modified) when appropriate. Even though we have the cached response, we will save on both bandwidth and client compute resources by not sending it.

Features
--------

The web's most common compression formats are supported and can be enabled via crate features: Brotli, Deflate, GZip, and Zstandard. The best encoding is selected by comparing the server and client's preferences (HTTP content negotiation).

Plug in your own cache by implementing a trait. [Moka](https://github.com/moka-rs/moka) support is included. Access to all cache functions is `async`. (Note that concurrent performance will depend on the actual cache implementation, the HTTP server, and of course your async runtime).

Note that even though [Tokio](https://github.com/tokio-rs/tokio) I/O types are used internally, this middleware does *not* require a specific async runtime.

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/tower-http-response-cache/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/tower-http-response-cache/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

All code and documentation was written by humans. We do not accept "AI"-generated contributions.
