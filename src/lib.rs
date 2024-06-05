#![doc(
    test(attr(deny(warnings))),
    test(attr(allow(bare_trait_objects, unknown_lints)))
)]
#![warn(missing_docs)]
// Don't fail on links to things not enabled in features
#![allow(
    unknown_lints,
    renamed_and_removed_lints,
    intra_doc_link_resolution_failure,
    broken_intra_doc_links
)]
// These little nifty labels saying that something needs a feature to be enabled
#![cfg_attr(docsrs, feature(doc_cfg))]
//! Library for rust service wrapper that run on Linux
//! # Examples
//! ```rust
//! use rs_svc::svc::service::Service;
//!
//! struct MyService;
//!
//! impl Service for MyService {
//!     fn init(&self) -> anyhow::Result<()> {
//!         println!("init");
//!         Ok(())
//!     }
//!
//!     // must be non-blocking
//!     fn start(&self) -> anyhow::Result<()> {
//!         std::thread::spawn(move || {
//!             println!("start")
//!         });
//!         Ok(())
//!     }
//!
//!     fn stop(&self) -> anyhow::Result<()> {
//!         print!("stop");
//!         Ok(())
//!     }
//! }
//!
//!
//! fn main() {
//!     let my_svc = MyService;
//!     rs_svc::svc::service::run(&my_svc).unwrap()
//! }
//! ```

pub mod svc;
