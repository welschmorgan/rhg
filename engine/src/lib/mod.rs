pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod location;
pub mod error;
pub mod archive;

pub use location::*;
pub use error::*;
pub use archive::*;