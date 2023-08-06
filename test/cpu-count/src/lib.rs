
use neon::prelude::*;

fn get_num_cpus(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", get_num_cpus)?;
    Ok(())
}








#[no_mangle]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}

/*
const sum = result.instance.exports.add(1, 2);
console.log(`1 + 2 = ${sum}`);
*/

