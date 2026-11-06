//! Volume management model.

#[derive(Debug, Clone)]
pub struct Volume {
    pub name: String,
    pub size: u64,
    pub fs_type: String,
}

#[derive(Debug, Default)]
pub struct VolumeManager {
    pub volumes: Vec<Volume>,
}

impl VolumeManager {
    pub fn new() -> Self { Self::default() }
    pub fn add_volume(&mut self, v: Volume) { self.volumes.push(v); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_manager() {
        let mut vm = VolumeManager::new();
        vm.add_volume(Volume { name: "root".into(), size: 100_000_000, fs_type: "ext4".into() });
        assert_eq!(vm.volumes.len(), 1);
    }
}
