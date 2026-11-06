//! Snapshot management model.

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub id: String,
    pub volume: String,
    pub timestamp: u64,
    pub size: u64,
}

#[derive(Debug, Default)]
pub struct SnapshotManager {
    pub snapshots: Vec<Snapshot>,
}

impl SnapshotManager {
    pub fn new() -> Self { Self::default() }
    pub fn create_snapshot(&mut self, s: Snapshot) { self.snapshots.push(s); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_manager() {
        let mut sm = SnapshotManager::new();
        sm.create_snapshot(Snapshot { id: "snap1".into(), volume: "root".into(), timestamp: 0, size: 0 });
        assert_eq!(sm.snapshots.len(), 1);
    }
}
