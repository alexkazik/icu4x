// generated by diplomat-tool
import type { DataProvider } from "./DataProvider"
import type { pointer, char } from "./diplomat-runtime.d.ts";


/** Lookup of the Canonical_Combining_Class Unicode property
*
*See the [Rust documentation for `CanonicalCombiningClassMap`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalCombiningClassMap.html) for more information.
*/
export class CanonicalCombiningClassMap {
    

    get ffiValue(): pointer;


    static create(provider: DataProvider): CanonicalCombiningClassMap;

    get(ch: char): number;

    

}