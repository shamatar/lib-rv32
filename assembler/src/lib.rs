/// Functions for assembling instructions and buffers.
mod assembler;

/// Functions for encoding integers as instruction fields.
pub mod encode;

/// Errors that may arise when assembling.
pub mod error;

/// Functions for parsing an instruction string.
pub mod parse;

/// Parsing function for special cased opcodes
pub mod special_cases;

/// Tools for external experiments
pub mod utils;

/// Unit-tests.
#[cfg(test)]
mod test;

/// Re-export common library.
pub use lib_rv32_common as common;

// Re-export assembler functions.
pub use assembler::*;
