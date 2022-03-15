extern crate lazy_static;
extern crate libc;
extern crate redhook;

use libc::{c_char};
use regex::Regex;
use std::ffi::{CStr,CString};


lazy_static::lazy_static! {
    static ref LOG_FNAME_RE: Regex = Regex::new(r"\bfuzz-\d+.log\b").unwrap();
}


redhook::hook! {
    unsafe fn system(command_addr: *const c_char) -> i32 => my_function {
        assert!(!command_addr.is_null());
        let command_bytes = CStr::from_ptr(command_addr);
        let command = command_bytes.to_str().unwrap();

        let command_devnull = LOG_FNAME_RE.replace(command, "/dev/null").to_string();
        println!("libFuzzer exec'd worker with system(\"{}\" --> \"{}\")",
                 &command, &command_devnull);

        let command_devnull_bytes = CString::new(command_devnull).unwrap();
        return redhook::real!(system)(command_devnull_bytes.as_ptr());
    }
}
