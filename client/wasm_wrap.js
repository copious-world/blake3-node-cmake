
  // ---- 
  //
  class WASMInterface {
    //
    constructor(mod_path) {
        this._mod_path = mod_path
        this.linearMemory = false
        this.mod = false
        this.shared_buffer = false
    }


    //$> get_wasm_module
    async get_wasm_module(mod_path,import_object) {
        let obj = await WebAssembly.instantiateStreaming(fetch(mod_path), import_object)
        this.linearMemory = obj.instance.exports.memory
        this.shared_buffer = this.linearMemory.buffer
        return obj.instance.exports
    }

    /**
     * 
     */
    async init(module_name) {
        //
        let importObject = {}
        //
        importObject[module_name] = {
                                        __memory_base: 0,
                                        __table_base: 0,
                                        memory: new WebAssembly.Memory({initial: 1}),
                                        "alert" : (str_offset,size) => { this.wasm_alert(str_offset,size) }
                                    }
        this.mod = await this.get_wasm_module(this._mod_path,importObject)
        let self = this
        for ( let ky in this.mod ) {
            self[ky] = this.mod[ky]
        }
        //
    }

    /**
     * 
     */
    write(string, pointer) {
        if ( this.linearMemory.buffer ) {
            const view = new Uint8Array(this.linearMemory.buffer, pointer, 1024);
            const encoder = new TextEncoder();
            view.set(encoder.encode(string));
        }
    }

    /**
     * 
     */
    read(pointer) {
        if ( this.linearMemory.buffer ) {
            const view = new Uint8Array(this.linearMemory.buffer, pointer, 1024);
            const length = view.findIndex(byte => byte === 0);
            const decoder = new TextDecoder();
            //
            return decoder.decode(new Uint8Array(this.linearMemory.buffer, pointer, length));
        }
    };


    /**
     * 
     */
    wasm_string(str_offset,size) {
        const stringBuffer = new Uint8Array(this.linearMemory.buffer, str_offset, size);
        let str = '';
        let n = stringBuffer.length;
        for ( let i=0; i < n; i++ ) {
            str += String.fromCharCode(stringBuffer[i]);
        }
        return str
    }

    /**
     * 
     */
    wasm_alert(str_offset,size) {
        let str = this.wasm_string(str_offset,size)
        alert(str)
    }


}


