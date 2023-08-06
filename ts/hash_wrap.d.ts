import { WASMInterface } from './wasm_wrap';
/**
 * Extends the WASM wrapper
 */
export declare class Hasher extends WASMInterface {
    w_hash: Function;
    constructor(mod_path: any);
    /**
     *
     * @param str - a javascript string to be hashed
     * @returns a Uint8Array containing the hash as bits.
     */
    hash(str: string): Uint8Array | undefined;
}
