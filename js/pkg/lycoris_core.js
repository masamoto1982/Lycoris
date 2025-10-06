import * as wasm from './lycoris_core_bg.wasm';
import { __wbg_set_wasm } from './lycoris_core_bg.js';
__wbg_set_wasm(wasm);
export * from './lycoris_core_bg.js';

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;
let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
};

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;
    const mem = getUint8Memory0();
    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

/**
 * Lycoris Interpreter WebAssembly Interface
 */
export class LycorisInterpreter {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LycorisInterpreter.prototype);
        obj.__wbg_ptr = ptr;
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_lycorisinterpreter_free(ptr);
    }

    /**
     * @returns {LycorisInterpreter}
     */
    constructor() {
        const ret = wasm.lycorisinterpreter_new();
        return LycorisInterpreter.__wrap(ret);
    }

    /**
     * @param {string} code
     * @returns {Promise<any>}
     */
    execute(code) {
        const ptr0 = passStringToWasm0(code, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.lycorisinterpreter_execute(this.__wbg_ptr, ptr0, len0);
        return ret;
    }

    /**
     * @returns {any}
     */
    get_stack() {
        const ret = wasm.lycorisinterpreter_get_stack(this.__wbg_ptr);
        return ret;
    }

    /**
     * @returns {any}
     */
    get_custom_words_info() {
        const ret = wasm.lycorisinterpreter_get_custom_words_info(this.__wbg_ptr);
        return ret;
    }

    /**
     * @returns {any}
     */
    reset() {
        const ret = wasm.lycorisinterpreter_reset(this.__wbg_ptr);
        return ret;
    }
}

export default async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('lycoris_core_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}
