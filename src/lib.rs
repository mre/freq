//!
//! `freq` is a library for counting occurrences of words in files.  
//! It is asynchronous and supports multiple input files.
//! Here is a basic usage example:
//!
//! ```
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!   let frequencies = freq::count("/path/to/input").await?;
//!   println!("{:?}", frequencies);
//!   Ok(())
//! }
//! ```
//!
//! For more specific use-cases you can build a freq client yourself,
//! using the `ClientBuilder` which can be used to
//! configure and run your own frequency counter and grants full flexibility:
//!
//! ```
//! use freq::{ClientBuilder, Status};
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!   let client = ClientBuilder::default().build()?;
//!   let frequencies = client.count("/path/to/input").await?;
//!   println!("{:?}", frequencies);
//!   Ok(())
//! }
//! ```

#[deny(missing_docs)]
#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;

#[cfg(doctest)]
doctest!("../README.md");

mod client;
mod excludes;
mod stats;
mod utils;

pub use client::count;
pub use client::ClientBuilder;
pub use stats::{Stats, WordStat};
pub use utils::traverse::Input;
