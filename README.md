# phone-backup — Phase 01: Foundation

Rust workspace skeleton for the Android backup platform, structured as
Clean Architecture / Hexagonal Architecture so the backup engine is
never locked to ADB.

## Layout

```
crates/
├── domain/              core entities: Device, Capability, CapabilityMatrix, DomainError
│                         zero dependency on anything below
├── ports/                DevicePort trait — the seam application depends on
│                         and adapters implement (dependency inversion)
├── application/          BackupService: use cases orchestrated through ports only
├── adapter-mock/         MockDeviceAdapter — fake DevicePort impl, stands in
│                         for AdbDeviceAdapter until Phase 02
├── adapter-filesystem/   placeholder, wired for Phase 04 (Scanner) / Phase 08 (Storage)
└── cli/                  composition root — the ONLY crate that wires a concrete
                          adapter into BackupService
```

Dependency direction is one-way: `cli -> application -> ports <- adapter-*`,
with `domain` underneath everything. `application` never imports an
`adapter-*` crate directly — swap `MockDeviceAdapter` for a future
`AdbDeviceAdapter` / `MtpDeviceAdapter` / iOS adapter by changing one
line in `cli/src/main.rs`.

## Try it

```bash
cargo build
cargo test

cargo run --bin phone-backup -- devices
cargo run --bin phone-backup -- device-info A1B2C3D4
```

## What's stubbed vs. real here

- `domain`, `ports`, `application`, `adapter-mock`, `cli`: fully working,
  with a seeded fake Pixel 8 device.
- `adapter-filesystem`: empty on purpose — real content lands with the
  Scanner (Phase 04) and Storage backend (Phase 08).

## Next (Phase 02 — Device Discovery)

Add `adapter-adb` implementing `ports::DevicePort` against real `adb`
(via shelling out or an ADB protocol crate), and swap it in at the
`cli` composition root. No other crate should need to change.
