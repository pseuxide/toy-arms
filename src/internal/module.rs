use std::mem::{size_of, zeroed};
use std::ptr::copy_nonoverlapping;
use std::str::Utf8Error;
use winapi::shared::minwindef::HMODULE;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::psapi::{GetModuleInformation, MODULEINFO};
use crate::cast;
use crate::internal::pattern_scan::boyer_moore_horspool;
use crate::internal::utils::get_module_handle;
use crate::utils_common::read_null_terminated_string;

#[derive(Debug)]
pub struct Module<'a> {
    pub module_name: &'a str,
    pub module_handle: HMODULE,
    pub module_size: u32,
    pub module_base_address: usize,
    pub data: Vec<u8>,
}

impl<'a> Default for Module<'a> {
    fn default() -> Self {
        Module {
            module_name: "",
            module_handle: 0x0 as HMODULE,
            module_size: 0,
            module_base_address: 0,
            data: vec![0u8; 80000000],
        }
    }
}

impl<'a> Module<'a> {
    pub fn from_module_name(module_name: &'a str) -> Option<Self> {
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
                module_name,
                module_handle,
                module_base_address: module_info.lpBaseOfDll as usize,
                module_size: module_info.SizeOfImage,
                data,
            };
            Some(module)
        }
    }

    /// read fetches the value that given address is holding.
    /// * `base_address` - the address that is supposed to have the value you want
    #[inline]
    pub fn read<T>(&self, address: u32) -> *mut T {
        cast!(mut self.module_handle as usize + address as usize, T)
    }

    /// read_string reads the string untill the null terminator that is in the given module
    /// * `address` - relative address of the head of the string.
    #[inline]
    pub fn read_string(&self, address: i32) -> Result<String, Utf8Error> {
        unsafe {
            read_null_terminated_string(self.module_handle as usize + address as usize)
        }
    }

    /// find_pattern scans over entire module and returns the address if there is matched byte pattern in module.
    /// * `pattern` - pattern string you're looking for. format: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF"
    #[inline]
    pub fn find_pattern(&self, pattern: &str) -> Option<usize> {
        let base = self.module_base_address;
        let end = self.module_base_address + self.module_size as usize;
        unsafe { boyer_moore_horspool(pattern, base, end).map(|e| e as usize) }
    }

    /// pattern scan basically be for calculating offset of some value. It adds the offset to the pattern-matched address, dereferences, and add the `extra`.
    /// * `pattern` - pattern string you're looking for. format: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF"
    /// * `offset` - offset of the address from pattern's base.
    /// * `extra` - offset of the address from dereferenced address.
    #[inline]
    pub fn pattern_scan(&self, pattern: &str, offset: isize, extra: usize) -> Option<usize> {
        unsafe {
            let address = self.find_pattern(pattern)?;
            let address = (address as *mut u8).offset(offset) as *mut usize;
            // calculate relative address
            Some(*address - self.module_base_address + extra)
        }
    }
}