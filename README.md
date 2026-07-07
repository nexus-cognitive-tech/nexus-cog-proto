# nexus-cog-proto

Protocol contracts for the [Nexus Cog](https://github.com/nexus-cognitive-tech) cognitive stack.

## Contents

- `proto/nexus_cog.proto` — canonical protobuf schema (cloud sync, future gRPC IPC)
- `src/lib.rs` — Rust-side serde-friendly representations

The `.proto` file is the source of truth; the Rust types mirror the protobuf messages so that JSON serialization works today and gRPC can be added without a breaking change.

## License

Apache-2.0.
