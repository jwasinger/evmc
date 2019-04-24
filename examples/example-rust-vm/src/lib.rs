extern crate evmc_vm;

use evmc_vm::evmc_sys as ffi;

extern "C" fn execute(
    instance: *mut ffi::evmc_instance,
    context: *mut ffi::evmc_context,
    rev: ffi::evmc_revision,
    msg: *const ffi::evmc_message,
    code: *const u8,
    code_size: usize,
) -> ffi::evmc_result {
    let result = evmc_vm::ExecutionResult::new(
        ffi::evmc_status_code::EVMC_SUCCESS,
        66,
        None,
        ffi::evmc_address { bytes: [0u8; 20] },
    );
    result.into()
}

extern "C" fn get_capabilities(
    instance: *mut ffi::evmc_instance,
) -> ffi::evmc_capabilities_flagset {
    ffi::evmc_capabilities::EVMC_CAPABILITY_EVM1 as u32
}

extern "C" fn destroy(instance: *mut ffi::evmc_instance) {
    drop(unsafe { Box::from_raw(instance) })
}

#[no_mangle]
pub extern "C" fn evmc_create_examplerustvm() -> *const ffi::evmc_instance {
    let ret = ffi::evmc_instance {
        abi_version: ffi::EVMC_ABI_VERSION as i32,
        destroy: Some(destroy),
        execute: Some(execute),
        get_capabilities: Some(get_capabilities),
        set_option: None,
        set_tracer: None,
        name: {
            let c_str =
                std::ffi::CString::new("ExampleRustVM").expect("Failed to build EVMC name string");
            c_str.into_raw() as *const i8
        },
        version: {
            let c_str = std::ffi::CString::new("1.0").expect("Failed to build EVMC version string");
            c_str.into_raw() as *const i8
        },
    };
    Box::into_raw(Box::new(ret))
}