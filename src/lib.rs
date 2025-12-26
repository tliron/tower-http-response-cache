// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/*!
HTTP response caching middleware with integrated encoding (compression).

For more information and usage examples see the
[home page](https://github.com/tliron/tower-http-response-cache).
*/

mod layer;
mod service;

/// Cache.
pub mod cache;

pub use {layer::*, service::*};
