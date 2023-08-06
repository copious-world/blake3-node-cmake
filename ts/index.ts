'use strict';
const blake3 = require('./src-neon/index.node');


// For 32 bit hashes, we can have one module that passes parameters using Nan.
// For larger data types, we can set up buffers. Another module can be used for the larger values.
// So, this will skip 64 bit hashes. For instance xxhash3 might be a better target with ARM support.



/// --- Blake3
/**
 * A wrapper class for use in node.js projects.
 * This module exports the class definition. The appliation program will create a new class as such:
 * 
 * ```
 * let hasher[i] = new Blake3()
 * hasher[i].init_running_hash() // call this if the update is to be used.
 * ```
 * 
 * Otherwise, for just getting hashes:
 * 
 * ```
 * let hasher = new Blake3()
 * let hash = hasher.hash("this is some data")
 * ```
 * 
 */
class Blake3 {

    index : number

    constructor() {
        this.index = -1;    // might use #index
    }

    /**
     * 
     * @param {string|object} data - If data is an object it will be passed to JSON.stringify.
     * @returns a buffer containing the hash value as u8.
     */
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

    /**
     * Initialize a hash for use in hash updating.
     * This method sets the index for the hash. If a 
     */
    init_running_hash() {
        this.index = blake3.init()
    }

    /**
     * 
     * @param {string|object} data - If data is an object it will be passed to JSON.stringify.
     */
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

    /**
     * This calls the `finalize` method of the internal hash method takes from the Blake3 rust module.
     * @returns a buffer containing the hash value as u8.
     */
    get_hash() {
        if ( this.index < 0 ) return false;
        return blake3.get_hash(this.index)
    }

    /**
     * Starts the hash over for the update sequence. (The index stored privately within this class will be the same.)
     * @returns true if the hash index is current and a new internal hasher can be constructed
     */
    reset() {
        if ( ( this.index >= 0 ) && blake3.reset(this.index) ) {   // the seed goes back in to do initialization...
            return true
        } else {
            this.index = -1;
            return false
        }
    }

    /**
     * Removes the internal representation of the updating object, freeing the internal index.
     * @returns the boolean indicating status of removal 
     */
    remove() {
        if ( this.index < 0 ) return false;
        return blake3.remove(this.index)
    }

}



module.exports.Blake3 = Blake3



