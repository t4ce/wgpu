#[cfg(target_os = "trueos")]
use alloc::format;
use alloc::string::String;

/// Facts reported by the mediated vGPU service, not WebGPU feature promises.
#[derive(Clone, Debug)]
pub struct AdapterProbe {
    pub capabilities: u64,
    pub memory_quota: u64,
    pub memory_used: u64,
    pub epoch: u64,
}

pub(super) fn probe() -> Result<AdapterProbe, String> {
    probe_native()
}

#[cfg(target_os = "trueos")]
fn probe_native() -> Result<AdapterProbe, String> {
    use trueos_v::vgpu::{Capabilities, Device};

    let requested = Capabilities::BUFFER
        .union(Capabilities::QUEUE)
        .union(Capabilities::TIMELINE);
    let device =
        Device::open(requested).map_err(|error| format!("TRUEOS vGPU open failed: {error}"))?;
    let info = device.info();
    let closed = device.close();
    let info = info.map_err(|error| format!("TRUEOS vGPU device info failed: {error}"))?;
    closed.map_err(|error| format!("TRUEOS vGPU close failed: {error}"))?;
    if info.is_lost() {
        return Err(String::from("TRUEOS vGPU device is lost"));
    }
    Ok(AdapterProbe {
        capabilities: info.capabilities,
        memory_quota: info.memory_quota,
        memory_used: info.memory_used,
        epoch: info.epoch,
    })
}

#[cfg(not(target_os = "trueos"))]
fn probe_native() -> Result<AdapterProbe, String> {
    Err(String::from("TRUEOS vGPU is unavailable on this target"))
}
