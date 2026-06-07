use libloading::{Library, Symbol};
use std::ffi::{c_int, CStr, CString};
use std::os::raw::c_char;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // DLLをロード
        let lib = Library::new("sample.dll")?;
        
        // 1. 整数加算テスト
        let add_func: Symbol<extern "C" fn(c_int, c_int) -> c_int> = lib.get(b"add")?;
        let sum = add_func(10, 20);
        println!("10 + 20 = {}", sum);
        println!("---");
        
        // 2. 文字列送受信テスト
        let process_func: Symbol<extern "C" fn(*const c_char, *mut c_char, c_int)> = 
            lib.get(b"process_string")?;
        
        // RustからC++に送信する文字列
        //let send_message = CString::new("Rustからのメッセージです")?;
        let send_message = CString::new("Rust from message")?;
        
        // 受信用バッファ（256バイト確保）
        let mut buffer = vec![0u8; 256];
        
        // C++関数を呼び出し
        process_func(
            send_message.as_ptr(),
            buffer.as_mut_ptr() as *mut c_char,
            buffer.len() as c_int
        );
        
        // 受信した文字列を表示
        let received = CStr::from_bytes_until_nul(&buffer)?.to_string_lossy();
        //println!("Rustが受信: {}", received);
        println!("Rust Receive: {}", received);
        println!("---");
        
        // 3. 大文字変換テスト
        let uppercase_func: Symbol<extern "C" fn(*const c_char, *mut c_char, c_int)> = 
            lib.get(b"to_uppercase")?;
        
        let input = CString::new("hello world from rust!")?;
        let mut upper_buffer = vec![0u8; 256];
        
        uppercase_func(
            input.as_ptr(),
            upper_buffer.as_mut_ptr() as *mut c_char,
            upper_buffer.len() as c_int
        );
        
        let upper_result = CStr::from_bytes_until_nul(&upper_buffer)?.to_string_lossy();
        //println!("大文字変換結果: {}", upper_result);
        println!("Upper change result: {}", upper_result);
        println!("---");
        
        // 4. 文字列連結テスト
        let concat_func: Symbol<extern "C" fn(*const c_char, *const c_char, *mut c_char, c_int)> = 
            lib.get(b"concatenate_strings")?;
        
        let str1 = CString::new("Rust + ")?;
        //let str2 = CString::new("C++ = パワフル！")?;
        let str2 = CString::new("C++ = Good!")?;
        let mut concat_buffer = vec![0u8; 512];
        
        concat_func(
            str1.as_ptr(),
            str2.as_ptr(),
            concat_buffer.as_mut_ptr() as *mut c_char,
            concat_buffer.len() as c_int
        );
        
        let concat_result = CStr::from_bytes_until_nul(&concat_buffer)?.to_string_lossy();
        //println!("連結結果: {}", concat_result);
        println!("plus-result: {}", concat_result);
    }
    
    Ok(())
}