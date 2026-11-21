//! SigmaOS-storage — storage model.
//!
//! Models block devices, volume management, partition tables,
//! snapshot management, and basic I/O scheduling. Deterministic
//! and std-only (no actual hardware access).

/// Storage volume management module.
pub mod sigmastorage;
pub mod block;
pub mod volume;
pub mod partition;
pub mod snapshot;
pub mod io;
pub mod sigmadiskmanager;

pub use block::{BlockDevice, BlockOp, BlockResult};
pub use volume::{Volume, VolumeManager};
pub use partition::{Partition, PartitionTable, GptEntry};
pub use snapshot::{Snapshot, SnapshotManager};
pub use io::{IoRequest, IoScheduler};
