use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use regex::bytes::Regex;
use winapi::shared::minwindef::DWORD;
use crate::internal::module::Module;


impl<'a> Module<'a> {
    fn generate_regex(&self, pattern: &str) -> Option<Regex> {
        let mut regex = pattern
            .split_whitespace()
            .map(|val| if val == "?" { ".".to_string() } else { format!("\\x{}", val) }).collect::<Vec<_>>().join("");
        regex.insert_str(0, "(?s-u)");
        Regex::new(&regex).ok()
    }

    /// find_pattern scans over entire module and returns the address if there is matched byte pattern in module.
    /// * `pattern` - pattern string you're looking for. format: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF"
    #[inline]
    pub fn find_pattern(&self, pattern: &str) -> Option<usize> {
        self.generate_regex(pattern)
            .and_then(|f| f.find(&self.data)).and_then(|f| Some(f.start()))
    }

    /// pattern scan basically be for calculating offset of some value. It adds the offset to the pattern-matched address, dereferences, and add the `extra`.
    /// * `pattern` - pattern string you're looking for. format: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF"
    /// * `offset` - offset of the address from pattern's base.
    /// * `extra` - offset of the address from dereferenced address.
    #[inline]
    pub fn pattern_scan(&mut self, pattern: &str, offset: usize, extra: usize) -> Option<usize> {
        let address = self.find_pattern(pattern)?;
        let address = address + offset;
        let pointed_at = self.read::<usize>(address);
        unsafe {
            // calculate relative address
            Some(*pointed_at as usize - self.module_base_address + extra)
        }
    }
}