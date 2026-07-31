//! Agent Memory File System — a local-first memory store for AI agents.
//!
//! The storage engine and the embedding backends are not implemented yet; this
//! crate currently only carries the CLI surface and the build metadata helpers.

pub mod cli;

/// Returns the build version information including git metadata
pub fn version() -> &'static str {
    env!("BUILD_VERSION")
}

/// Returns the Rust version used to build this binary
pub fn rust_version() -> &'static str {
    env!("BUILD_RUST_VERSION")
}

/// Returns the Cargo version used to build this binary
pub fn cargo_version() -> &'static str {
    env!("BUILD_CARGO_VERSION")
}

// Unit tests live alongside the code they verify, grouped into one nested
// module per function. See the Rust Book, ch. 11.3 — "Test Organization":
// https://doc.rust-lang.org/book/ch11-03-test-organization.html
#[cfg(test)]
mod tests {
    mod version_info {
        use super::super::*;

        #[test]
        fn version_is_populated() {
            assert!(!version().is_empty());
        }

        #[test]
        fn rust_version_is_populated() {
            assert!(!rust_version().is_empty());
        }

        #[test]
        fn cargo_version_is_populated() {
            assert!(!cargo_version().is_empty());
        }
    }
}
