//! pattern_scan_common has a common components which is referenced by external and internal variants of pattern scan mods.

use winapi::um::winnt::{MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_NOACCESS};

pub fn is_page_readable(memory_info: &MEMORY_BASIC_INFORMATION) -> bool {
    if memory_info.State != MEM_COMMIT
        || memory_info.Protect == 0x0
        || memory_info.Protect == PAGE_NOACCESS
    {
        return false;
    }
    true
}

pub fn process_pattern_from_str(pattern: &str) -> Vec<u8> {
    pattern
        .split_whitespace()
        .map(|x| {
            if x.contains('?') {
                b'\x3F'
            } else {
                u8::from_str_radix(x, 16)
                    .expect("Substring not contained within hexadecimal alphanumeric form")
            }
        })
        .collect()
}
