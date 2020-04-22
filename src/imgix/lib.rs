// Module declarations.
pub mod constants;
pub mod url;
pub mod util;
pub mod validate;

/// Re-exports.
pub use util::command_prelude;
pub use util::errors::{Error, Result};
pub use url::{Scheme, Url};
pub use constants::lib_version;
