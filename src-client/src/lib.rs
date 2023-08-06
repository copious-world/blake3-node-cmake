
use blake3::hash;
use wasm_nopackage::{go_live,set_plugin_name};


#[link(wasm_import_module = "mod")]
extern "C" {
    fn message_js(s: *mut i8, size: usize);
}


fn output_string(s: *mut i8, size: usize) -> () {
    unsafe {
        message_js(s,size);
    }
}

fn my_plugin_name() -> &'static str {
    let my_str : &str = "Blake3 exposed";
    return my_str;
}


///
/// free memory taken up in linear memory.
/// The original pointer offset will be found in ptr, and the size of the previously allocated block is required.
#[no_mangle]
pub fn startup() {
    go_live(output_string);
    set_plugin_name(my_plugin_name);
}



/// Given a pointer to the start of a byte array and
/// its length, read a string, create its uppercase
/// representation, then return the pointer in
/// memory to it.
#[no_mangle]
pub unsafe fn w_hash(ptr: *mut u8, len: usize) -> *mut u8 {
    // create a `Vec<u8>` from the pointer and length
    // here we could also use Rust's excellent FFI
    // libraries to read a string, but for simplicity,
    // we are using the same method as for plain byte arrays
    let data = Vec::from_raw_parts(ptr, len, len);
    // Call the hash function
    let buffer = &data[..];
    let hash_obj = hash(buffer);     // this returns a hash object
    let mut bref : [u8; 32] = hash_obj.as_bytes().to_owned();
    let ptr = bref.as_mut_ptr();
    // take ownership of the memory block where the result string
    // is written and ensure its destructor is not
    // called whe the object goes out of scope
    // at the end of the function
    std::mem::forget(data);
    let _ = bref;
    // return the pointer to the hash buffer
    // so the runtime can read data from this offset
    ptr
}



