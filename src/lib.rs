#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![no_std]

//! Backwards compatibility is the worst invention ever
//!
//! # Modules
//! By default all modules are disabled.
//! To enable a module, add its name as a feature prefixed by `M` (eg: `Mmacros`)
//!
//! - [`macros`]
//!
//! # Available features
//!  By default all features are disabled
//!
//! | Name | Description |
//! | - | - |
//! | `std` | Enables items that require or use features from `std` |

#[doc(hidden)]
#[cfg(feature = "std")]
pub extern crate std as _std;

#[cfg(feature = "Mmacros")]
pub mod macros;
