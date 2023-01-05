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


fn format_error_message(error_code: u32) -> String {
    format!("
Can't create process due {0} error code.
Read page https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes#system-error-codes
Невозможно создать процесс из-за ошибки с кодом {0}.
Читайте страницу https://learn.microsoft.com/ru-ru/windows/win32/debug/system-error-codes#system-error-codes
", error_code)

}

fn service_token_by_login(login: &str, password: &str) -> Result<HANDLE, (String, u32)> {
    let login_raw = tools::encode_str(login);
    let password_raw = tools::encode_str(password);
    let mut token: HANDLE = null::<c_void>().cast_mut();

    let success: i32;
    unsafe {
        success = LogonUserW(
            login_raw,
            null::<u16>().cast_mut(),
            password_raw,
            LOGON32_LOGON_SERVICE,
            LOGON32_PROVIDER_DEFAULT,
            &mut token,
        );
    };
    if success == false as i32 {
        let error_code = unsafe { GetLastError() };

        return Err((format_error_message(error_code), error_code));
    };

    Ok(token)
}

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
            Err(code) => panic!("Failed test with code {}. Info: {}", code.1, code.0),
        };
    }
}
