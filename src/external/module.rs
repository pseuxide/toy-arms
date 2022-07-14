use std::{ fmt, fmt::Debug };
use std::mem::{ size_of_val };

use winapi::{
    shared::{
        minwindef::{ HMODULE },
    },
    um::{
        winnt::{ HANDLE },
        tlhelp32::{
            MODULEENTRY32,
        }
    }
};

use crate::utils_common::read_null_terminated_string;
use smartstring::alias::String;
use crate::external::error::TAExternalError;
use crate::external::read;

#[derive(Debug)]
pub struct Module<'a> {
    pub process_handle: &'a HANDLE,
    pub module_size: u32,
    pub module_base_address: usize,
    pub module_handle: HMODULE,
    pub module_name: &'a str,
    pub module_path: String,
    pub data: Vec<u8>,
}

impl<'a> fmt::Display for Module<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module_name: {}\nmodule_size: {}\nmodule_base_address: {}\nmodule_handle: {:?}\nmodule_path: {}\nprocess_handle: {:?}", self.module_name, self.module_size, self.module_base_address, self.module_handle, self.module_path, self.process_handle)
    }
}

impl<'a> Default for Module<'a> {
    fn default() -> Self {
        Module {
            process_handle: &(0x0 as HANDLE),
            module_size: 0,
            module_base_address: 0,
            module_handle: 0x0 as HMODULE,
            module_name: "",
            module_path: String::default(),
            data: vec![0u8; 80000000],
        }
    }
}

impl<'a> Module<'a> {
    pub(crate) fn from_module_entry(process_handle: &'a HANDLE, module_entry: &MODULEENTRY32, module_name: &'a str) -> Result<Self, TAExternalError> {
        let mut module = Module {
            process_handle,
            module_size: module_entry.modBaseSize,
            module_base_address: module_entry.modBaseAddr as usize,
            module_handle: module_entry.hModule,
            module_name,
            // This is allowed because szExePath.as_ptr() is the address within module_entry variable, not the address in the target process.
            module_path: unsafe { read_null_terminated_string(module_entry.szExePath.as_ptr() as usize) }.unwrap().parse().unwrap(),
            data: vec![0u8; module_entry.modBaseSize as usize]
        };
        let ok = read::<Vec<u8>>(module.process_handle, module.module_base_address, module.data.len(), module.data.as_mut_ptr() as *mut Vec<u8>);
        match ok {
            Err(e) => Err(e),
            _ => Ok(module),
        }
    }

    pub(crate) fn ensure_data_populated(&mut self) {
        if size_of_val(&self.data) == 80000000 {
            self.data.resize(self.module_size as usize, 0u8);
        }

        if self.data.iter().all(|&val| val == 0) {
            read::<Vec<u8>>(self.process_handle, self.module_base_address, self.data.len(), self.data.as_mut_ptr() as *mut Vec<u8>);
        }
    }
}