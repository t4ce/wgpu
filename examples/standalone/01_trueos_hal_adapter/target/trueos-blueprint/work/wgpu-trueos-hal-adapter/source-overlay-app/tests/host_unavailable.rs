#![cfg(not(target_os = "trueos"))]

#[test]
fn unavailable_native_service_is_a_failed_probe() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_wgpu-trueos-hal-adapter"))
        .output()
        .expect("probe executable should run");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(!output.status.success());
    assert!(stdout.contains("TRUEOS_HAL_ADAPTER BEGIN"));
    assert!(!stdout.contains("TRUEOS_HAL_ADAPTER PASS"));
    assert!(stderr.contains("TRUEOS_HAL_ADAPTER FAIL: TRUEOS vGPU is unavailable on this target"));
}
