use libloading::{Library, Symbol};
use std::ffi::{c_int, CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut cmd = String::new();
    if args.len() < 2 {
        println!("error: no argument");
        return Ok(());
    }     
    cmd = args[1].clone();
    println!("cmd={}", cmd);

    if cmd == "add" {
        if args.len() < 3 {
            println!("error: no argument");
            return Ok(());
        }           
        let title = args[2].clone();
        println!("title={}", title);        
        unsafe {
            // DLLをロード
            let lib = Library::new("sample.dll")?;
            
            let todo_add_func: Symbol<extern "C" fn(*const c_char, *mut c_char, c_int) -> c_int> =
                lib.get(b"todo_add")?;

            // RustからC++に送信する文字列
            let send_message = CString::new(title)?;
            
            // 受信用バッファ（nバイト確保）
            let mut buffer = vec![0u8; 1024];
                
            // C++関数を呼び出し
            let ret_add = todo_add_func(
                send_message.as_ptr(),
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len() as c_int
            );
            println!("ret_add = {}", ret_add);
            
            // 受信した文字列を表示
            let received = CStr::from_bytes_until_nul(&buffer)?.to_string_lossy();
            println!("Rust Receive: {}", received);
            println!("---");        
        }

    }
    if cmd == "list" {
        unsafe {
            // DLLをロード
            let lib = Library::new("sample.dll")?;
            let todo_list_func: Symbol<extern "C" fn(*const c_char, *mut c_char, c_int) -> c_int> =
                lib.get(b"todo_list")?;

            let send_message = CString::new("")?;            
            // 受信用バッファ（nバイト確保）
            let mut buffer = vec![0u8; 2048];
            // C++関数を呼び出し
            let ret_list = todo_list_func(
                send_message.as_ptr(),
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len() as c_int
            );
            println!("ret_list = {}", ret_list);
            
            // 受信した文字列を表示
            let received = CStr::from_bytes_until_nul(&buffer)?.to_string_lossy();
            println!("Rust Receive: {}", received);
            println!("---");        
        }
    }
    if cmd == "del" {
        if args.len() < 3 {
            println!("error: no argument");
            return Ok(());
        }           
        let id_str = args[2].clone();
        println!("id_str={}", id_str); 
        let num2 = id_str.parse::<i32>().unwrap();        

        unsafe {
            let lib = Library::new("sample.dll")?;
            let todo_delete_func: Symbol<extern "C" fn(c_int) -> c_int> = lib.get(b"todo_delete")?;
            let iRet = todo_delete_func(num2);

            println!("iRet = {}", iRet);
        }
    }
    
    Ok(())
}