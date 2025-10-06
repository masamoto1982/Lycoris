import { LycorisInterpreter } from './lycoris_core.js';

let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

const heap = new Array(128).fill(undefined);
heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

export function __wbg_lycorisinterpreter_free(arg0) {
    wasm.__wbg_lycorisinterpreter_free(arg0);
}

export function lycorisinterpreter_new() {
    const ret = wasm.lycorisinterpreter_new();
    return ret;
}

export function lycorisinterpreter_execute(arg0, arg1, arg2) {
    const ret = wasm.lycorisinterpreter_execute(arg0, arg1, arg2);
    return takeObject(ret);
}

export function lycorisinterpreter_get_stack(arg0) {
    const ret = wasm.lycorisinterpreter_get_stack(arg0);
    return takeObject(ret);
}

export function lycorisinterpreter_get_custom_words_info(arg0) {
    const ret = wasm.lycorisinterpreter_get_custom_words_info(arg0);
    return takeObject(ret);
}

export function lycorisinterpreter_reset(arg0) {
    const ret = wasm.lycorisinterpreter_reset(arg0);
    return takeObject(ret);
}

export function __wbindgen_json_parse(arg0, arg1) {
    const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}

export function __wbindgen_json_serialize(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = JSON.stringify(obj);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
}

export const __wbg_get_imports = function() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_json_parse = __wbg_json_parse;
    imports.wbg.__wbindgen_json_serialize = __wbg_json_serialize;
    return imports;
};

export const __wbg_load = function(module, imports) {
    return WebAssembly.instantiate(module, imports);
};

export const __wbg_finalize_init = function(instance, module) {
    wasm = instance.exports;
    __wbg_set_wasm(wasm);
    return wasm;
};
