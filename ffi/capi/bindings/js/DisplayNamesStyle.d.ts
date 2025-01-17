// generated by diplomat-tool
import type { pointer, char } from "./diplomat-runtime.d.ts";

// Base enumerator definition
/** See the [Rust documentation for `Style`](https://docs.rs/icu/latest/icu/displaynames/options/enum.Style.html) for more information.
*/
export class DisplayNamesStyle {
    constructor(value : DisplayNamesStyle | string);

    get value() : string;

    get ffiValue() : number;

    static Auto : DisplayNamesStyle;

    static Narrow : DisplayNamesStyle;

    static Short : DisplayNamesStyle;

    static Long : DisplayNamesStyle;

    static Menu : DisplayNamesStyle;


    

}