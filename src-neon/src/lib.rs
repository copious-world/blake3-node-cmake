// ----
use neon::prelude::*;
use blake3::hash;


// let hash1 = blake3::hash(b"foobarbaz");

fn expose_hash(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let s : Handle<JsString> = cx.argument(0)?;
    let hashable : String = s.value(&mut cx);
    let hash_obj = hash(hashable.as_bytes());
    let bref : &[u8; 32] = hash_obj.as_bytes();

    Ok(JsBuffer::external(&mut cx, bref.to_vec()))
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hash", expose_hash)?;
    Ok(())
}





/*
// Hash an input all at once.
let hash1 = blake3::hash(b"foobarbaz");

// Hash an input incrementally.
let mut hasher = blake3::Hasher::new();
hasher.update(b"foo");
hasher.update(b"bar");
hasher.update(b"baz");
let hash2 = hasher.finalize();
assert_eq!(hash1, hash2);

// Extended output. OutputReader also implements Read and Seek.
let mut output = [0; 1000];
let mut output_reader = hasher.finalize_xof();
output_reader.fill(&mut output);
assert_eq!(hash1, output[..32]);

// Print a hash as hex.
println!("{}", hash1);
*/