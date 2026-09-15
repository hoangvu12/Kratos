//! Engine-local SQLite document snapshots and command ledger.

mod store;
pub mod wake;
pub mod net_path;
pub use store::{DocsStore, StoreError};
