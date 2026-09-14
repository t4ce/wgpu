# TRUEOS HAL bring-up

This module is the start of a native backend in the local wgpu fork. Its API
marker is `wgpu_hal::api::TrueOs`, with its own `Backend::TrueOs` and
`Backends::TRUEOS` identities. The opt-in `trueos` feature is forwarded through
wgpu and wgpu-core to wgpu-hal.

## Implemented boundary

`Instance::init` calls `integration::probe()`. On TRUEOS the bridge uses the
canonical `v::vgpu` API from the sibling TRUEOS-Blueprints checkout:

1. Open a mediated device requesting buffer, queue and timeline capabilities.
2. Read its native capabilities, memory quota/usage and epoch.
3. Close the temporary device, including when the information query fails.
4. Retain the reported facts in `Context::probe_info()`.

No PCI identity is fabricated. Native capability bits are not translated into
WebGPU promises. In particular, a native COMPUTE bit does not establish that
the generic WGSL shader, binding and dispatch path exists.

On other targets initialization returns an unavailable error. The backend
still compiles there so its contract and feature plumbing can be checked
without linking native TRUEOS symbols.

## Current limits

The six method-bearing HAL traits and Api type table provide the implementation
locations for subsequent work. Successful initialization now enumerates exactly
one `Adapter` containing the native probe snapshot. Its identity is TRUEOS vGPU
with backend TrueOs; PCI IDs remain unknown (zero), and device type is Other.
Features, downlevel flags and cooperative-matrix configurations are empty.
All resource maxima are zero, including max_buffer_size: native memory quota
is not exposed as usable WebGPU buffer capacity.

Alignment fields use nonzero sentinels and ShaderModel uses its lowest enum
value (Sm2 has no corresponding “no shaders” variant). These are inert
representation values, not measured ABI alignments or shader support.
`Adapter::open()` continues to reject every request. This backend does not yet execute compute,
render or copy work, or create a UI4 presentation surface. Resource operations
are scaffolding, not advertised functionality.

The existing TRUEOS service exposes buffers, queues, timelines and specialized
render/compute workloads. General shader compilation, compute pipeline creation,
bindings and dispatch need a defined native integration contract before the
hello-compute example can execute through this backend. Buffer mapping and
readback must also obey the HAL's synchronization and pointer-lifetime rules.

## Direct native initialization test

`examples/standalone/00_trueos_hal_probe` calls the HAL directly and reports
PASS after native open/query/close succeeds. It preserves initialization errors
and does not depend on wgpu-core or adapter acquisition. See its README for
Blueprint build and runtime evidence instructions. This is the first native
service test, before implementing any additional HAL resources.

## Validation

From the wgpu repository:

```sh
cargo +nightly-2026-07-10 test -p wgpu-hal --no-default-features --features trueos --lib trueos
cargo +nightly-2026-07-10 check -p wgpu --no-default-features --features std,wgsl,trueos
```

The sibling `TRUEOS-Blueprints/apps/wgpu-hello-compute` enables the feature on
TRUEOS and prints the HAL probe facts. It still cannot run compute; the
adapter exposes no compute capability and cannot be opened. Build it from the TRUEOS-Blueprints root:

```sh
TRUEOS_BLUEPRINT_SKIP_APPS_PUBLISH=1 cargo bp wgpu-hello-compute
```

Compilation and packaging do not prove native service execution; that requires
a separate run on TRUEOS hardware or an appropriate VM.

## HAL adapter enumeration test

`examples/standalone/01_trueos_hal_adapter` is a separate permanent diagnostic:
init, enumerate, require exactly one TrueOs adapter, check the conservative
capabilities, print its native facts, then emit `TRUEOS_HAL_ADAPTER PASS`.
Build with `cargo bpp wgpu-trueos-hal-adapter` from TRUEOS-Blueprints.
The original `00_trueos_hal_probe` remains unchanged.

Stage 1 has user-reported native PASS evidence: capabilities 0x7, memory_used 0,
memory_quota 33554432 and epoch 6. Stage 2 requires its own native run.
The next diagnostic after HAL enumeration is core request_adapter; device
opening, fences, buffers and request_device follow separately.
