//! Block device model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockOp { Read, Write, Flush }

#[derive(Debug, Clone)]
pub struct BlockDevice {
    pub name: String,
    pub size: u64,
    pub block_size: u32,
}

#[derive(Debug, Clone)]
pub struct BlockResult {
    pub op: BlockOp,
    pub offset: u64,
    pub len: u32,
    pub success: bool,
}

impl BlockDevice {
    pub fn new(name: String, size: u64, block_size: u32) -> Self {
        Self { name, size, block_size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_device() {
        let bd = BlockDevice::new("sda".into(), 1_000_000_000, 512);
        assert_eq!(bd.block_size, 512);
    }
}
