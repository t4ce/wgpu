# TRUEOS HAL probe

A direct test of `wgpu_hal::Instance::init` using `hal::api::TrueOs`.
It depends on wgpu-hal, without the public wgpu API or wgpu-core. No adapter
acquisition is needed for this test.

The native sequence is:

```text
HAL Instance::init
  -> integration::probe
  -> vGPU open
  -> vGPU device_info
  -> vGPU close
  -> print reported capabilities, memory usage/quota and epoch
  -> PASS
```

`TRUEOS_HAL_PROBE BEGIN` marks entry. An initialization error prints
`TRUEOS_HAL_PROBE FAIL: <native error>` and returns a nonzero exit status.
`TRUEOS_HAL_PROBE PASS` is emitted only after the complete sequence succeeds,
including successful close. Information-query failure still attempts close.

PASS proves access to the mediated vGPU service through the HAL. It does not
prove shader execution, rendering, presentation, or WebGPU adapter support.
The printed capability bits are native facts, not advertised WebGPU features.

## Build for TRUEOS

The named build command is `!cargo bpp wgpu-trueos-hal-probe`.
In a terminal, run it from the sibling TRUEOS-Blueprints repository:

```sh
cargo bpp wgpu-trueos-hal-probe
```

This builds `dist/wgpu-trueos-hal-probe.bp` and publishes the named probe to
the probe catalog. It does not launch the probe on the rig. The `bpp` alias
selects the probes catalog, whose `wgpu-trueos-hal-probe` entry points directly
to this standalone example in the wgpu checkout.

For a local build without publication:

```sh
TRUEOS_BLUEPRINT_SKIP_APPS_PUBLISH=1 cargo bpp wgpu-trueos-hal-probe
```

After publication, run `probe wgpu-trueos-hal-probe` in Shell2 Apps mode.

Retain the BEGIN, INFO and PASS/FAIL output from the actual TRUEOS run as
runtime evidence. A successful build is only compile/link validation.

## Host checks

From the wgpu repository:

```sh
cargo +nightly-2026-07-10 test -p wgpu-trueos-hal-probe
cargo +nightly-2026-07-10 clippy -p wgpu-trueos-hal-probe --tests
cargo +nightly-2026-07-10 run -p wgpu-trueos-hal-probe
```

The last command is expected to fail on Linux with the explicit native-service
unavailable message. It never substitutes a host GPU for the TRUEOS probe.

## Following milestones

Split Instance, Adapter, Device, Queue and Surface into distinct ownership types
before implementing their lifecycles. Introduce concrete surface textures,
views and timeline-backed fences alongside those implementations. Keep native
capabilities separate from the operations actually exposed through the HAL.
None of those later resources is required to run this initialization probe.
