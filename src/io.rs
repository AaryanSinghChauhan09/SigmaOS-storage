//! I/O scheduling model.

#[derive(Debug, Clone)]
pub struct IoRequest {
    pub offset: u64,
    pub len: u32,
    pub op: crate::block::BlockOp,
}

#[derive(Debug, Default)]
pub struct IoScheduler {
    pub queue: Vec<IoRequest>,
}

impl IoScheduler {
    pub fn new() -> Self { Self::default() }
    pub fn submit(&mut self, req: IoRequest) { self.queue.push(req); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_scheduler() {
        let mut s = IoScheduler::new();
        s.submit(IoRequest { offset: 0, len: 512, op: crate::block::BlockOp::Read });
        assert_eq!(s.queue.len(), 1);
    }
}
