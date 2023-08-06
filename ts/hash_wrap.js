


class Hasher extends WASMInterface {


    constructor(mod_path) {
        super(mod_path);
    }

    hash(str) {
        const [pointer,size] = this.write(str)        // write does the encoding
        let hashresult = this.w_hash(pointer)
        this.dealloc(pointer,size)
        return this.read_Uint8Array(hashresult)
    }


}



window.Hasher = Hasher
