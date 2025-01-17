// generated by diplomat-tool
import type { DataProvider } from "./DataProvider"
import type { pointer, char } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `DecomposingNormalizer`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html) for more information.
*/
export class DecomposingNormalizer {
    

    get ffiValue(): pointer;


    static createNfd(provider: DataProvider): DecomposingNormalizer;

    static createNfkd(provider: DataProvider): DecomposingNormalizer;

    normalize(s: string): string;

    isNormalized(s: string): boolean;

    isNormalizedUtf16(s: string): boolean;

    isNormalizedUpTo(s: string): number;

    isNormalizedUtf16UpTo(s: string): number;

    

}