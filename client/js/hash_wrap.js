




class Hasher extends WASMInterface {


    constructor(mod_path) {
        super(mod_path);
    }

    hash(str) {
        const pointer =  this.alloc(str.length);
        this.write(str, pointer)        // write does the encoding
        let hashresult = this.w_hash(pointer)
        this.dealloc(pointer,str.length)
        return this.read_Uint8Array(hashresult)
    }


}



window.Hasher = Hasher