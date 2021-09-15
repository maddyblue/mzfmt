/* tslint:disable */
/* eslint-disable */
/**
* @param {string} str
* @param {number} width
* @returns {string}
*/
export function pretty_str(str: string, width: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly pretty_str: (a: number, b: number, c: number, d: number) => void;
  readonly rust_psm_on_stack: (a: number, b: number, c: number, d: number) => void;
  readonly rust_psm_stack_direction: () => number;
  readonly rust_psm_stack_pointer: () => number;
  readonly rust_psm_replace_stack: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
