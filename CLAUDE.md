# Sony Bravia CLI

Rust CLI + HTTP API for controlling Sony Bravia TVs via RS-232 serial.

## Build & Test

```sh
cargo build
cargo test
cargo clippy
```

## Architecture

- `src/main.rs` - Entry point, CLI dispatch, tokio async runtime
- `src/cli.rs` - Clap argument definitions
- `src/transport.rs` - Serial port abstraction, command execution
- `src/http.rs` - Axum HTTP API server (60+ endpoints)
- `src/protocol/` - RS-232 protocol implementation
  - `mod.rs` - `Command` trait, packet building, checksum logic
  - `values.rs` - Bounded value types via `bounded_value!` macro
  - `error.rs` - Error types, response codes
  - Category modules: `mode_control.rs`, `picture.rs`, `screen.rs`, `sound.rs`, `sircs.rs`, `language.rs`, `signage.rs`

## Key Patterns

**Command trait:** Each protocol command implements `Command` with associated `Action` and `Response` types. Generic `Transport::execute::<C>()` and `Transport::query::<C>()` methods.

**Bounded values:** Use `bounded_value!(TypeName, min, max)` macro for range-validated parameters.

**Packet format:** `[header, category(0x00), function_code, length, data..., checksum]`

**Headers:** Control=0x8C, Query=0x83, Response=0x70

## Adding New Commands

1. Add action/state enums to appropriate category module
2. Implement `Command` trait with function code from docs/
3. Add CLI variant in `cli.rs`
4. Add match arm in `main.rs`
5. Add HTTP endpoint in `http.rs` if needed

## Protocol Docs

Sony RS232C specifications: https://github.com/andrewrabert/sony-bravia-rs232c-documentation
