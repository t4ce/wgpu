use std::process::ExitCode;
use wgpu_hal as hal;

type TrueOsInstance = <hal::api::TrueOs as hal::Api>::Instance;

fn run() -> Result<(), String> {
    let descriptor = hal::InstanceDescriptor {
        name: "wgpu-trueos-hal-adapter",
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        telemetry: None,
        display: None,
    };
    let instance = unsafe { <TrueOsInstance as hal::Instance>::init(&descriptor) }
        .map_err(|error| error.to_string())?;
    let adapters =
        unsafe { <TrueOsInstance as hal::Instance>::enumerate_adapters(&instance, None) };
    if adapters.len() != 1 {
        return Err(format!("expected one adapter, got {}", adapters.len()));
    }
    let exposed = &adapters[0];
    if exposed.info.backend != <hal::api::TrueOs as hal::Api>::VARIANT {
        return Err(format!("unexpected backend: {:?}", exposed.info.backend));
    }
    if !exposed.features.is_empty()
        || !exposed.capabilities.downlevel.flags.is_empty()
        || exposed.capabilities.limits.max_buffer_size != 0
        || exposed
            .capabilities
            .limits
            .max_compute_invocations_per_workgroup
            != 0
    {
        return Err(String::from(
            "unexpected resource capabilities at enumeration-only stage",
        ));
    }
    let probe = exposed.adapter.probe_info();
    println!(
        "TRUEOS_HAL_ADAPTER INFO: count={} backend={:?} name={:?} features={:?} downlevel={:?}",
        adapters.len(),
        exposed.info.backend,
        exposed.info.name,
        exposed.features,
        exposed.capabilities.downlevel.flags,
    );
    println!(
        "TRUEOS_HAL_ADAPTER NATIVE: capabilities={:#018x} memory_used={} memory_quota={} epoch={}",
        probe.capabilities, probe.memory_used, probe.memory_quota, probe.epoch,
    );
    Ok(())
}

fn main() -> ExitCode {
    println!("TRUEOS_HAL_ADAPTER BEGIN");
    match run() {
        Ok(()) => {
            println!("TRUEOS_HAL_ADAPTER PASS");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("TRUEOS_HAL_ADAPTER FAIL: {error}");
            ExitCode::FAILURE
        }
    }
}
