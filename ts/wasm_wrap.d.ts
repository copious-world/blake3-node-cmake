declare type ExportValue = any;
/**
 *  This provides a basic interface to some WASM module,
 *  where the module will make use of a basic allocation and deallocation.
 *  The interface will provide one based callback for the module for the purpose of logging.
 */
export declare class WASMInterface {
    _mod_path: String;
    linearMemory: boolean | ExportValue;
    mod: boolean | Object;
    encoder: TextEncoder;
    decoder: TextDecoder;
    alloc: (size: number) => number;
    dealloc: (pointer: number, size: number) => void;
    plugin_name: Function;
    plugin_name_len: Function;
    constructor(mod_path: any);
    /**
     *
     * @param mod_path - a string indicating the server URI path to a resource
     * @param import_object - an object conforming to WASM import decriptor format
     * @returns {Object} - this is a map from keys to ExportValue
     */
    get_wasm_module(mod_path: any, import_object: any): Promise<WebAssembly.Exports>;
    /**
     *
     * @param module_name - a string indicating the server URI path to a resource
     *
     * Creates an Import Object for use by the module.
     * The import object sets the memory base, the table base, and creates a new WebAssembly.Memory.
     * The "alert" method is provided to the WASM for use. (This may be implemented in any way the application sees fit:
     * override `wasm_alert`).
     *
     * After a WASM stream is instantiated, the exported module object will update the current class
     * with keys representing the exported functions of the WASM source.
     *
     */
    init(module_name: any): Promise<void>;
    /**
     *
     * @param {string} str - a string from JS to be written to the WASM memory page.
     * @returns {Array} - a pair:
     *  * *first* = the pointer (offset) into WASM memory where the string will reside (-1 on error)
     *  * *second* = returns the size of the region, which may differ from the size of the string.
     */
    write(str: string): [number, number];
    /**
     *
     * @param pointer - the memory offset into the WASM page where text data is located
     * @param size  - the size of the section previously returned by alloc.
     * @returns {string} - the JS string derived from encoding the bytes stored at the pointer.
     */
    read(pointer: any, size: any): string;
    /**
     *
     * @param pointer - the memory offset into the WASM page where text data is located
     * @param size - the size of the section previously returned by alloc.
     * @returns {Uint8Array} - the bytes stored in the section without decoding
     */
    read_Uint8Array(pointer: any, size: any): Uint8Array;
    /**
     *
     * @param {number} str_offset - an offset passed by the module indicating where the string is stored
     * @param {number} size - the size of the string.
     * @returns {string} - the string returned from WASM storage.
     */
    wasm_string(str_offset: any, size: any): string;
    /**
     * A method provided to the module allowing a string the module generates to be used for alert or logging.
     *
     * calss `wasm_string`
     *
     * @param {number} str_offset - an offset passed by the module indicating where the string is stored
     * @param {number} size - the size of the string.
     */
    wasm_alert(str_offset: any, size: any): void;
    plugin_name_str(): string;
}
export {};
