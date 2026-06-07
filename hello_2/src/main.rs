// src/main.rs
use libloading::{Library, Symbol};
use std::ffi::c_int;

fn main() {
    unsafe {
        // DLLをロード
        let lib = Library::new("sample.dll").expect("DLLの読み込みに失敗");
        
        // 関数を取得
        let add_func: Symbol<extern "C" fn(c_int, c_int) -> c_int> = 
            lib.get(b"add").expect("関数の取得に失敗");
        
        // 関数を呼び出し
        let result = add_func(5, 3);
        println!("5 + 3 = {}", result);
    }
}