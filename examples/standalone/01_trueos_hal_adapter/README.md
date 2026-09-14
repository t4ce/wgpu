# TRUEOS HAL adapter enumeration

Stage 2 follows the permanent `00_trueos_hal_probe` initialization diagnostic.
This standalone executable calls HAL init and enumerate_adapters directly. It
has no dependency on wgpu or wgpu-core and never opens a device.

Expected successful native output includes:

```text
TRUEOS_HAL_ADAPTER BEGIN
TRUEOS_HAL_ADAPTER INFO: count=1 backend=TrueOs name="TRUEOS vGPU" ...
TRUEOS_HAL_ADAPTER NATIVE: capabilities=... memory_used=... memory_quota=... epoch=...
TRUEOS_HAL_ADAPTER PASS
```

The probe fails with a nonzero exit status on initialization failure, an
unexpected adapter count/backend, or unexpected resource capability claims.
Linux reports the native service as unavailable and must not print PASS.

The adapter owns a snapshot of the native facts. Native capability bits and
memory quota are not translated into WebGPU feature bits or buffer capacity.
Features, downlevel flags and cooperative-matrix configurations are empty;
all resource maxima are zero. Alignment fields are nonzero sentinels pending
actual resource support. Device opening is still explicitly unsupported.

From the TRUEOS-Blueprints repository, the named command is:

```text
!cargo bpp wgpu-trueos-hal-adapter
```

In a terminal, use `cargo bpp wgpu-trueos-hal-adapter`. It builds
`dist/wgpu-trueos-hal-adapter.bp` and publishes the named probe. To build
without publication:

```sh
TRUEOS_BLUEPRINT_SKIP_APPS_PUBLISH=1 cargo bpp wgpu-trueos-hal-adapter
```

After publication, use `probe wgpu-trueos-hal-adapter` in Shell2 Apps mode.
Retain the output from that run as Stage-2 native evidence. Compilation and
host tests do not establish that the adapter probe has passed on hardware.

The diagnostic ladder remains:

1. `00_trueos_hal_probe`: HAL/native initialization and cleanup.
2. `01_trueos_hal_adapter`: HAL enumeration and capability mapping.
3. Future core adapter probe: request_adapter through wgpu-core.
4. Later: native device opening, fences, buffers, and request_device.

Host checks, from the wgpu repository:

```sh
cargo +nightly-2026-07-10 test -p wgpu-hal --no-default-features --features trueos --lib trueos
cargo +nightly-2026-07-10 test -p wgpu-trueos-hal-probe -p wgpu-trueos-hal-adapter
cargo +nightly-2026-07-10 clippy -p wgpu-trueos-hal-adapter --tests
```
