pub mod module {
    use std::convert::TryInto;
    use std::fmt::Debug;
    use std::mem::zeroed;
    use std::mem::size_of;
    use regex::bytes::Regex;
    use crate::external::module::Module;
    use crate::external::read;

    impl<'a> Module<'a> {
        fn generate_regex(&self, pattern: &str) -> Option<Regex> {
            let mut regex = pattern
                .split_whitespace()
                .map(|val| if val == "?" { ".".to_string() } else { format!("\\x{}", val)}).collect::<Vec<_>>().join("");
            regex.insert_str(0, "(?s-u)");
            Regex::new(&regex).ok()
        }

        pub fn find_pattern(&mut self, pattern: &str) -> Option<usize> {
            self.generate_regex(pattern)
                .and_then(|f| f.find(&self.data)).and_then(|f| Some(f.start()))
        }
        // pattern scan basically be for calculating offset of some value. It adds the offset to the pattern-matched address, dereferences, and add the `extra`.
        // * `pattern` - pattern string you're looking for. format: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF"
        // * `offset` - offset of the address from pattern's base.
        // * `extra` - offset of the address from dereferenced address.
        pub fn pattern_scan<T>(&mut self, pattern: &str, offset: usize, extra: usize) -> Option<T>
            where T: std::ops::Add<Output = T>,
                  T: std::ops::Sub<Output = T>,
                  T: std::convert::TryFrom<usize>,
                  <T as std::convert::TryFrom<usize>>::Error: Debug,
        {
            let address = self.find_pattern(pattern)?;
            let address = address + offset;
            let mut target_buffer: T = unsafe { zeroed::<T>() };
            read::<T>(self.process_handle, self.module_base_address + address, size_of::<T>(), &mut target_buffer as *mut T).expect("READ FAILED IN PATTERN SCAN");
            Some( target_buffer - self.module_base_address.try_into().unwrap() + extra.try_into().unwrap())
        }
    }
}