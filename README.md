# SigmaOS Storage

This crate exports a `StorageManager` and organizes APIs under `storage`.
Those source modules do not by themselves establish that filesystems are
mounted, block devices are managed, or data is durable and recoverable on a
host. Verify each backend and its failure behavior before relying on storage
operations.

## License

See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).

