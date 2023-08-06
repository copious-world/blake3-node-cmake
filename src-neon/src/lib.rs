// ----
use neon::prelude::*;
use blake3::hash;
use blake3::Hasher;

use std::collections::HashMap;


struct HashKeeper {
    run_index      : u16,
    hash_instances : HashMap<u16,Hasher>,
}

impl HashKeeper {
    fn new() -> Self {
        HashKeeper {
            run_index : 0,
            hash_instances : HashMap::<u16,Hasher>::new()
        }
    }
}

static mut STREAM_HASHES : Option<HashKeeper> = None;

// let hash1 = blake3::hash(b"foobarbaz");

fn expose_hash(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let s : Handle<JsString> = cx.argument(0)?;
    let hashable : String = s.value(&mut cx);
    let hash_obj = hash(hashable.as_bytes());
    let bref : &[u8; 32] = hash_obj.as_bytes();

    Ok(JsBuffer::external(&mut cx, bref.to_vec()))
}




fn init_hash(mut cx: FunctionContext) -> JsResult<JsNumber> {
    unsafe {
        if let Some(ref mut hashm) = STREAM_HASHES {
            hashm.run_index += 1;
            let hasher = blake3::Hasher::new();
            hashm.hash_instances.insert(hashm.run_index, hasher);
            Ok(cx.number(hashm.run_index))
        } else {
            Ok(cx.number(-1))
        }    
    }
}

fn expose_update(mut cx: FunctionContext) -> JsResult<JsBoolean> {

    let i : Handle<JsNumber> = cx.argument(0)?;
    let idx : u16 = i.value(&mut cx) as u16;
    let s : Handle<JsString> = cx.argument(1)?;
    let hashable : String = s.value(&mut cx);
    let hash_obj = hash(hashable.as_bytes());
    let bref : &[u8; 32] = hash_obj.as_bytes();
    //
    unsafe {
        if let Some(ref mut hashm) = STREAM_HASHES {
            if let Some(hasher) = hashm.hash_instances.get_mut(&idx) {
                hasher.update(bref);
                Ok(cx.boolean(true))    
            } else {
                Ok(cx.boolean(false))
            }
        } else {
            Ok(cx.boolean(false))
        }    
    }

}

fn get_hash(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let i : Handle<JsNumber> = cx.argument(0)?;
    let idx : u16 = i.value(&mut cx) as u16;
    //
    unsafe {
        if let Some(ref mut hashm) = STREAM_HASHES {
            if let Some(hasher) = hashm.hash_instances.get_mut(&idx) {
                hasher.finalize();
                let hash_obj = hasher.finalize();
                let bref : &[u8; 32] = hash_obj.as_bytes();
                Ok(JsBuffer::external(&mut cx, bref.to_vec()))
            } else {
                let bref : &[u8] = "".as_bytes();
                Ok(JsBuffer::external(&mut cx, bref.to_vec()))
            }
        } else {
            let bref : &[u8] = "".as_bytes();
            Ok(JsBuffer::external(&mut cx, bref.to_vec()))
    }
    }

}


// reset(this.index)
// get_hash(this.index)




fn reset(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let i : Handle<JsNumber> = cx.argument(0)?;
    let idx : u16 = i.value(&mut cx) as u16;
    //
    unsafe {
        if let Some(ref mut hashm) = STREAM_HASHES {
            let hasher = blake3::Hasher::new();
            hashm.hash_instances.insert(idx, hasher);
            Ok(cx.boolean(true))
        } else {
            Ok(cx.boolean(false))
        }    
    }
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    //
    unsafe {
        STREAM_HASHES = Some(HashKeeper::new());
    }
    //
    cx.export_function("hash", expose_hash)?;
    cx.export_function("init", init_hash)?;
    cx.export_function("reset", reset)?;
    cx.export_function("update", expose_update)?;
    cx.export_function("get_hash", get_hash)?;
    //
    Ok(())
}




