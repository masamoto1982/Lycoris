/* tslint:disable */
/* eslint-disable */
/**
 * Lycoris Interpreter WebAssembly Interface
 */
export class LycorisInterpreter {
  free(): void;
  /**
   * Creates a new Lycoris interpreter instance
   */
  constructor();
  /**
   * Execute Lycoris code
   * @param {string} code
   * @returns {Promise<any>}
   */
  execute(code: string): Promise<any>;
  /**
   * Get current stack state
   * @returns {any}
   */
  get_stack(): any;
  /**
   * Get custom words information
   * @returns {any}
   */
  get_custom_words_info(): any;
  /**
   * Reset interpreter state
   * @returns {any}
   */
  reset(): any;
}

/**
 * Initialize the WebAssembly module
 * @param {InitInput | Promise<InitInput>} input
 * @returns {Promise<InitOutput>}
 */
export default function init(
  input?: InitInput | Promise<InitInput>
): Promise<InitOutput>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_lycorisinterpreter_free: (a: number) => void;
  readonly lycorisinterpreter_new: () => number;
  readonly lycorisinterpreter_execute: (a: number, b: number, c: number) => number;
  readonly lycorisinterpreter_get_stack: (a: number) => number;
  readonly lycorisinterpreter_get_custom_words_info: (a: number) => number;
  readonly lycorisinterpreter_reset: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
 * Value types used in Lycoris
 */
export interface Value {
  type: 'number' | 'string' | 'boolean' | 'vector' | 'nil';
  value: any;
}

/**
 * Execution result
 */
export interface ExecuteResult {
  status: 'OK' | 'ERROR';
  output?: string;
  message?: string;
  error?: boolean;
  stack?: Value[];
}
