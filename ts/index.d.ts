declare const blake3: any;
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
declare class Blake3 {
    index: number;
    constructor();
    /**
     *
     * @param {string|object} data - If data is an object it will be passed to JSON.stringify.
     * @returns a buffer containing the hash value as u8.
     */
    hash(data: any): any;
    /**
     * Initialize a hash for use in hash updating.
     * This method sets the index for the hash. If a
     */
    init_running_hash(): void;
    /**
     *
     * @param {string|object} data - If data is an object it will be passed to JSON.stringify.
     */
    update(data: any): void;
    /**
     * This calls the `finalize` method of the internal hash method takes from the Blake3 rust module.
     * @returns a buffer containing the hash value as u8.
     */
    get_hash(): any;
    /**
     * Starts the hash over for the update sequence. (The index stored privately within this class will be the same.)
     * @returns true if the hash index is current and a new internal hasher can be constructed
     */
    reset(): boolean;
    /**
     * Removes the internal representation of the updating object, freeing the internal index.
     * @returns the boolean indicating status of removal
     */
    remove(): any;
}
