#![cfg_attr(evil_json_nightly, feature(core_intrinsics))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

mod api;
mod bufwrite;
mod bufwriter;
mod error;
mod escape;
mod raw;
mod ser;
mod suffix;

pub use api::{to_string, to_vec, to_writer};
pub use error::Error;
