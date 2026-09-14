use std::process::ExitCode;
use wgpu_hal as hal;

type TrueOsInstance = <hal::api::TrueOs as hal::Api>::Instance;

fn main() -> ExitCode {
    println!("TRUEOS_HAL_PROBE BEGIN: Instance::init");
    let descriptor = hal::InstanceDescriptor {
        name: "wgpu-trueos-hal-probe",
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        telemetry: None,
        display: None,
    };

    let instance = match unsafe { <TrueOsInstance as hal::Instance>::init(&descriptor) } {
        Ok(instance) => instance,
        Err(error) => {
            eprintln!("TRUEOS_HAL_PROBE FAIL: {error}");
            return ExitCode::FAILURE;
        }
    };

    let probe = instance.probe_info();
    println!(
        "TRUEOS_HAL_PROBE INFO: capabilities={:#018x} memory_used={} memory_quota={} epoch={}",
        probe.capabilities, probe.memory_used, probe.memory_quota, probe.epoch,
    );
    println!("TRUEOS_HAL_PROBE PASS: native vGPU opened, queried, and closed");
    ExitCode::SUCCESS
}
