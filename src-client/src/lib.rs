use std::mem;
use std::ffi::CString;
//use std::ffi::CStr;
use std::os::raw::{c_char};
use blake3::hash;





#[no_mangle]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}

/*
const sum = result.instance.exports.add(1, 2);
console.log(`1 + 2 = ${sum}`);
*/



/*
cargo install blake3
cargo build --target wasm32-unknown-unknown --release


https://github.com/radu-matei/wasm-memory/blob/main/src/lib.rs
https://depth-first.com/articles/2020/07/07/rust-and-webassembly-from-scratch-hello-world-with-strings/
*/


/// Allocate memory into the module's linear memory
/// and return the offset to the start of the block.
#[no_mangle]
pub fn alloc(len: usize) -> *mut u8 {
    // create a new mutable buffer with capacity `len`
    let mut buf = Vec::with_capacity(len);
    // take a mutable pointer to the buffer
    let ptr = buf.as_mut_ptr();
    // take ownership of the memory block and
    // ensure the its destructor is not
    // called when the object goes out of scope
    // at the end of the function
    mem::forget(buf);
    // return the pointer so the runtime
    // can write data at this offset
    return ptr;
}


/*
#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut c_void) {
    let _ = Vec::from_raw_parts(ptr, 0, 1024);
}
*/


#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
    let data = Vec::from_raw_parts(ptr, size, size);
    mem::drop(data);
}




/*
#[no_mangle]
pub unsafe extern "C" fn w_hash(in_ptr: *mut u8, out_ptr: *mut u8) {
    let hasher = hash(in_ptr);
    let bref : &[u8; 32] = hash_obj.as_bytes();
    out_ptr[..bref.len()].copy_from_slice(bref);
}

*/



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




/*
/// The Node.js WASI runtime requires a `_start` function
/// for instantiating the module.
#[no_mangle]
pub fn _start() {}
*/



static PLUGIN_NAME: &'static str = "Blake3 exposed";

#[no_mangle]
pub extern "C" fn plugin_name() -> *mut c_char {
    let s = CString::new(PLUGIN_NAME).unwrap();
    s.into_raw()
}

#[no_mangle]
pub fn plugin_name_len() -> usize {
    PLUGIN_NAME.len()
}


