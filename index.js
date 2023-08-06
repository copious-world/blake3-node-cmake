'use strict';
const blake3 = require('./src-neon/index.node');


// For 32 bit hashes, we can have one module that passes parameters using Nan.
// For larger data types, we can set up buffers. Another module can be used for the larger values.
// So, this will skip 64 bit hashes. For instance xxhash3 might be a better target with ARM support.



/// --- Blake3
class Blake3 extends CommonHash {
    //
    constructor() {
        this.index = -1;
    }

    hash(data) {
        if ( typeof data !== "string" ) {
            if ( typeof data === 'object' ) {
                data = JSON.stringify(data)
            } else {
                data = data.toString()
            }
        }
        return blake3.hash(data)
    }

    init_running_hash() {
        this.index = blake3.init()
    }

    update(data) {
        if ( this.index < 0 ) return;
        //
        if ( typeof data !== "string" ) {
            if ( typeof data === 'object' ) {
                data = JSON.stringify(data)
            } else {
                data = data.toString()
            }
        }
        blake3.update(this.index,data)
    }

    get_hash() {
        if ( this.index < 0 ) return false;
        return blake3.get_hash(this.index)
    }

    reset() {
        if ( ( this.index >= 0 ) && blake3.reset(this.index) ) {   // the seed goes back in to do initialization...
            return true
        } else {
            this.index = -1;
            return false
        }
    }
    //
    remove() {
        if ( this.index < 0 ) return false;
        return blake3.remove(this.index)
    }

}



module.exports.Blake3 = Blake3



