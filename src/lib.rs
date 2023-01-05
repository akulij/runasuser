use std::mem::size_of;
use std::ptr::null;

#[cfg(windows)]
extern crate winapi;
use winapi::um::processthreadsapi::CreateProcessAsUserW;
use winapi::um::winbase::LogonUserW;
use winapi::um::errhandlingapi::GetLastError;

use winapi::ctypes::c_void;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::processthreadsapi::{PROCESS_INFORMATION, STARTUPINFOW};
use winapi::um::winbase::{LOGON32_LOGON_SERVICE, LOGON32_PROVIDER_DEFAULT};
use winapi::um::winnt::HANDLE;

mod tools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_login() {
        let username = "test";
        let pass = "pass";
        let ret = runcmd_login(username, pass, &vec!["touch", "testrunfile.txt"]);
        match ret {
            Ok(_) => (),
            Err(code) => panic!("Failed test with code {}", code.to_string())
        };
    }
}
