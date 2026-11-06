//! Partition table model (GPT).

#[derive(Debug, Clone)]
pub struct GptEntry {
    pub start_lba: u64,
    pub end_lba: u64,
    pub partition_type_guid: [u8; 16],
    pub unique_guid: [u8; 16],
    pub attributes: u64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Partition {
    pub number: u32,
    pub start: u64,
    pub size: u64,
    pub fs_type: String,
}

#[derive(Debug, Default)]
pub struct PartitionTable {
    pub partitions: Vec<Partition>,
}

impl PartitionTable {
    pub fn new() -> Self { Self::default() }
    pub fn add_partition(&mut self, p: Partition) { self.partitions.push(p); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_table() {
        let mut pt = PartitionTable::new();
        pt.add_partition(Partition { number: 1, start: 2048, size: 100_000, fs_type: "ext4".into() });
        assert_eq!(pt.partitions.len(), 1);
    }
}
