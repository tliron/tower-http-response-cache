// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod layer;
mod service;

/// Cache.
pub mod cache;

pub use {layer::*, service::*};
