pub fn encode_str(s: &str) -> *mut u16 {
    s.encode_utf16().collect::<Vec<u16>>().as_mut_ptr()
}

pub fn generate_cmd(args: &Vec<&str>) -> *mut u16 {
    let mut cmd_str: Vec<u16> = args.join(" ").as_str().encode_utf16().collect();
    cmd_str.push(0);

    cmd_str.as_mut_ptr()
}
