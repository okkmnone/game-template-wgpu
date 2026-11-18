#![cfg_attr(false, no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![forbid(rustdoc::missing_crate_level_docs)]
//#![warn(rustdoc::missing_doc_code_examples)]
#![forbid(unused_imports)]
#![deny(clippy::join_absolute_paths)]
#![deny(clippy::missing_safety_doc)]
//#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::empty_line_after_doc_comments)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::empty_docs)]
#![warn(
    dead_code,
    unknown_lints,
    mismatched_lifetime_syntaxes,
    unused_qualifications
)]
//#![deny(missing_docs)]
#![deny(unused_doc_comments)]
#![deny(unsafe_code)]
#![warn(
    clippy::too_many_lines,
    clippy::missing_panics_doc,
    clippy::enum_glob_use,
    clippy::match_same_arms,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::derive_partial_eq_without_eq,
    clippy::empty_structs_with_brackets
)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::uninlined_format_args)]

pub mod config;
pub mod error;

pub use glam;

use crate::{config::Config, error::Result};

pub fn run(_config: Config) -> Result<()> {
    println!("{}", std::any::type_name_of_val(&run));

    Ok(())
}
