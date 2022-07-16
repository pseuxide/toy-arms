use std::mem::{size_of, zeroed};
use std::ptr::copy_nonoverlapping;
use std::str::Utf8Error;
use winapi::shared::minwindef::{DWORD, HMODULE, MAX_PATH};
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::psapi::{GetModuleBaseNameA, GetModuleInformation, MODULEINFO};
use winapi::um::winnt::{CHAR, LPSTR};
use crate::cast;
use super::utils::get_module_handle;
use toy_arms::utils::read_null_terminated_string;
use smartstring::alias::String;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub handle: HMODULE,
    pub size: u32,
    pub base_address: usize,
    pub data: Vec<u8>,
}

impl Default for Module {
    fn default() -> Self {
        Module {
            name: String::new(),
            handle: 0x0 as HMODULE,
            size: 0,
            base_address: 0,
            data: vec![0u8; 80000000],
        }
    }
}

impl Module {
    pub fn from_name(module_name: &str) -> Option<Self> {
        let module_handle: HMODULE = match get_module_handle(module_name) {
            Some(e) => e,
            None => return None,
        };
        unsafe {
            let mut module_info: MODULEINFO = zeroed::<MODULEINFO>();
            GetModuleInformation(
                GetCurrentProcess(),
                module_handle,
                &mut module_info,
                size_of::<MODULEINFO>() as u32,
            );
            let mut data:Vec<u8> = Vec::with_capacity(module_info.SizeOfImage as usize);
            let data_ptr = data.as_mut_ptr();
            data.set_len(0);
            copy_nonoverlapping(module_info.lpBaseOfDll as *const u8, data_ptr, module_info.SizeOfImage as usize);
            data.set_len(module_info.SizeOfImage as usize);

            let module = Module {
                name: String::from(module_name),
                handle: module_handle,
                base_address: module_info.lpBaseOfDll as usize,
                size: module_info.SizeOfImage,
                data,
            };
            Some(module)
        }
    }

    pub fn from_handle(handle: HMODULE) -> Option<Self> {
        unsafe {
            let mut module_info: MODULEINFO = zeroed::<MODULEINFO>();
            let process_handle = GetCurrentProcess();
            GetModuleInformation(
                process_handle,
                handle,
                &mut module_info,
                size_of::<MODULEINFO>() as DWORD,
            );

            let mut name_buffer: [CHAR; MAX_PATH] = [0; MAX_PATH];
            GetModuleBaseNameA(
                GetCurrentProcess(),
                handle,
                &mut name_buffer as LPSTR,
                std::mem::size_of_val(&name_buffer) as u32,
            );

            let module_name =
                read_null_terminated_string(&mut name_buffer as *mut i8 as usize).unwrap();

            let mut data:Vec<u8> = Vec::with_capacity(module_info.SizeOfImage as usize);
            let data_ptr = data.as_mut_ptr();
            data.set_len(0);
            copy_nonoverlapping(module_info.lpBaseOfDll as *const u8, data_ptr, module_info.SizeOfImage as usize);
            data.set_len(module_info.SizeOfImage as usize);

            let module = Module {
                name: String::from(module_name),
                handle,
                base_address: module_info.lpBaseOfDll as usize,
                size: module_info.SizeOfImage,
                data,
            };
            Some(module)
        }
    }

    /// read fetches the value that given address is holding.
    /// * `base_address` - the address that is supposed to have the value you want
    #[inline]
    pub fn read<T>(&self, address: usize) -> *mut T {
        cast!(mut self.base_address as usize + address as usize, T)
    }

    /// read_string reads the string untill the null terminator that is in the given module
    /// * `address` - relative address of the head of the string.
    #[inline]
    pub fn read_string(&self, address: i32) -> Result<std::string::String, Utf8Error> {
        unsafe {
            read_null_terminated_string(self.handle as usize + address as usize)
        }
    }

}
