use std::str::Utf8Error;

#[inline]
pub unsafe fn read_null_terminated_string(base_address: usize) -> Result<String, Utf8Error> {
    let len = (0..500).take_while(|&i| *(base_address as *const u8).offset(i) != 0 ).count();
    let slice = std::slice::from_raw_parts(base_address as *const u8, len);

    match String::from_utf8(slice.to_vec()) {
        Ok(val) => Ok(val),
        Err(e) => return Err(e.utf8_error())
    }
}
