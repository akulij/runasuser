pub fn encode_str(s: &str) -> *mut u16 {
    s.encode_utf16().collect::<Vec<u16>>().as_mut_ptr()
}
