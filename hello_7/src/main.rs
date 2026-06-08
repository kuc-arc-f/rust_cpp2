use libloading::{Library, Symbol};
use std::ffi::{c_int, CStr, CString};
use std::os::raw::c_char;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // DLLをロード
        let lib = Library::new("sample.dll")?;
        
        let test_http: Symbol<extern "C" fn() -> c_int> = lib.get(b"test_http")?;
        let res1 = test_http();
        println!("res1={}", res1);
    }
    
    Ok(())
}