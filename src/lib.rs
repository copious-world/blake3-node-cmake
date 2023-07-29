use std::mem;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};


#[link(wasm_import_module = "mod")]
extern "C" {
    fn alert(s: *mut i8, size: usize);
}



#[no_mangle]
pub extern "C" fn alloc() -> *mut c_void {
    let mut buf = Vec::with_capacity(1024);
    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}


#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut c_void) {
    let _ = Vec::from_raw_parts(ptr, 0, 1024);
}



pub unsafe fn convert_str(input: &str) -> *mut c_char {
    let c_str = CString::new(input).unwrap().into_raw();
    return c_str;
}



#[no_mangle]
pub unsafe extern "C" fn greet(ptr: *mut u8) {
    let str_content = CStr::from_ptr(ptr as *const i8).to_str().unwrap();
    let mut string_content = String::from("Hello, ");

    string_content.push_str(str_content);
    string_content.push_str("!");
    let alertable = string_content.clone();

    let c_headers = CString::new(string_content).unwrap();


    let bytes = c_headers.as_bytes_with_nul();

    let pointer = convert_str(&alertable.as_str());
    alert(pointer,alertable.len());
    drop(CString::from_raw(pointer));


    let header_bytes = std::slice::from_raw_parts_mut(ptr, 1024);
    header_bytes[..bytes.len()].copy_from_slice(bytes);
}




static PLUGIN_NAME: &'static str = "Test Plugin";

#[no_mangle]
pub extern "C" fn plugin_name() -> *mut c_char {
    let s = CString::new(PLUGIN_NAME).unwrap();
    s.into_raw()
}

#[no_mangle]
pub fn plugin_name_len() -> usize {
    PLUGIN_NAME.len()
}


