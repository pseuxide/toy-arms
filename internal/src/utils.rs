//! utils contains functions that is used across this entire crate multiple times because of its usefulness.

#[doc(hidden)]
pub use winapi::{
    shared::minwindef::BOOL,
    shared::minwindef::HINSTANCE,
    shared::minwindef::TRUE,
    shared::minwindef::{FARPROC, HMODULE},
    um::consoleapi::AllocConsole,
    um::libloaderapi::DisableThreadLibraryCalls,
    um::libloaderapi::{GetModuleHandleA, GetProcAddress},
    um::wincon::FreeConsole,
    um::winnt::DLL_PROCESS_ATTACH,
};

/// cast is a substitution of reinterpret_cast in C++.
/// * `$address` - address or variable you wanna cast.
/// * `$type` - type you want to cast $address into.
#[macro_export]
macro_rules! cast {
    // Value cast
    (mut $address:expr, $type:ident) => {
        $address as *mut $type
    };
    ($address:expr, $type:ident) => {
        $address as *const $type
    };
}

/// create_entrypoint fully automates the process of making DllMain on your behalf.
/// * `function` - function you want to run in the newly created thread.
#[macro_export]
macro_rules! create_entrypoint {
    ($function:expr) => {
        #[no_mangle]
        #[allow(non_snake_cake)]
        extern "system" fn DllMain(
            h_module: $crate::utils::HINSTANCE,
            dw_reason: u32,
            _: *const ::std::ffi::c_void,
        ) -> $crate::utils::BOOL {
            if dw_reason == $crate::utils::DLL_PROCESS_ATTACH {
                unsafe {
                    $crate::utils::DisableThreadLibraryCalls(h_module);
                }
                ::std::thread::spawn(|| {
                    if cfg!(debug_assertions) {
                        unsafe {
                            $crate::utils::AllocConsole();
                        }
                    }
                    $function();
                    if cfg!(debug_assertions) {
                        unsafe {
                            $crate::utils::FreeConsole();
                        }
                    }
                });
            }
            $crate::utils::TRUE
        }
    };
}

/// get_module_handle returns Option where should contains HMODULE.
/// This function may fail even if you put a correct dll name somehow, so it tries get handle for 100 times then returns Option
/// * `text` - name of the module you want
pub fn get_module_handle(module_name: &str) -> Option<HMODULE> {
    unsafe {
        let mut module_handle: HMODULE = GetModuleHandleA(make_lpcstr(module_name));
        for _ in 0..100 {
            if module_handle != 0 as HMODULE {
                break;
            }
            module_handle = GetModuleHandleA(make_lpcstr(module_name));
        }
        if module_handle == 0 as HMODULE {
            None
        } else {
            Some(module_handle)
        }
    }
}

pub(crate) fn make_lpcstr(text: &str) -> *const i8 {
    format!("{}{}", text, "\0").as_ptr() as *const i8
}