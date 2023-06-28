// @generated
/// Implement [`DataProvider<BasicEmojiV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_basic_emoji_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_BASIC_EMOJI_V1: &'static <icu_properties::provider::BasicEmojiV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                unsafe {
                    #[allow(unused_unsafe)]
                    icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x1A#\0\0\x1C#\0\0\xE9#\0\0\xED#\0\0\xF0#\0\0\xF1#\0\0\xF3#\0\0\xF4#\0\0\xFD%\0\0\xFF%\0\0\x14&\0\0\x16&\0\0H&\0\0T&\0\0\x7F&\0\0\x80&\0\0\x93&\0\0\x94&\0\0\xA1&\0\0\xA2&\0\0\xAA&\0\0\xAC&\0\0\xBD&\0\0\xBF&\0\0\xC4&\0\0\xC6&\0\0\xCE&\0\0\xCF&\0\0\xD4&\0\0\xD5&\0\0\xEA&\0\0\xEB&\0\0\xF2&\0\0\xF4&\0\0\xF5&\0\0\xF6&\0\0\xFA&\0\0\xFB&\0\0\xFD&\0\0\xFE&\0\0\x05'\0\0\x06'\0\0\n'\0\0\x0C'\0\0('\0\0)'\0\0L'\0\0M'\0\0N'\0\0O'\0\0S'\0\0V'\0\0W'\0\0X'\0\0\x95'\0\0\x98'\0\0\xB0'\0\0\xB1'\0\0\xBF'\0\0\xC0'\0\0\x1B+\0\0\x1D+\0\0P+\0\0Q+\0\0U+\0\0V+\0\0\x04\xF0\x01\0\x05\xF0\x01\0\xCF\xF0\x01\0\xD0\xF0\x01\0\x8E\xF1\x01\0\x8F\xF1\x01\0\x91\xF1\x01\0\x9B\xF1\x01\0\x01\xF2\x01\0\x02\xF2\x01\0\x1A\xF2\x01\0\x1B\xF2\x01\0/\xF2\x01\x000\xF2\x01\x002\xF2\x01\x007\xF2\x01\08\xF2\x01\0;\xF2\x01\0P\xF2\x01\0R\xF2\x01\0\0\xF3\x01\0!\xF3\x01\0-\xF3\x01\x006\xF3\x01\x007\xF3\x01\0}\xF3\x01\0~\xF3\x01\0\x94\xF3\x01\0\xA0\xF3\x01\0\xCB\xF3\x01\0\xCF\xF3\x01\0\xD4\xF3\x01\0\xE0\xF3\x01\0\xF1\xF3\x01\0\xF4\xF3\x01\0\xF5\xF3\x01\0\xF8\xF3\x01\0?\xF4\x01\0@\xF4\x01\0A\xF4\x01\0B\xF4\x01\0\xFD\xF4\x01\0\xFF\xF4\x01\0>\xF5\x01\0K\xF5\x01\0O\xF5\x01\0P\xF5\x01\0h\xF5\x01\0z\xF5\x01\0{\xF5\x01\0\x95\xF5\x01\0\x97\xF5\x01\0\xA4\xF5\x01\0\xA5\xF5\x01\0\xFB\xF5\x01\0P\xF6\x01\0\x80\xF6\x01\0\xC6\xF6\x01\0\xCC\xF6\x01\0\xCD\xF6\x01\0\xD0\xF6\x01\0\xD3\xF6\x01\0\xD5\xF6\x01\0\xD8\xF6\x01\0\xDC\xF6\x01\0\xE0\xF6\x01\0\xEB\xF6\x01\0\xED\xF6\x01\0\xF4\xF6\x01\0\xFD\xF6\x01\0\xE0\xF7\x01\0\xEC\xF7\x01\0\xF0\xF7\x01\0\xF1\xF7\x01\0\x0C\xF9\x01\0;\xF9\x01\0<\xF9\x01\0F\xF9\x01\0G\xF9\x01\0\0\xFA\x01\0p\xFA\x01\0}\xFA\x01\0\x80\xFA\x01\0\x89\xFA\x01\0\x90\xFA\x01\0\xBE\xFA\x01\0\xBF\xFA\x01\0\xC6\xFA\x01\0\xCE\xFA\x01\0\xDC\xFA\x01\0\xE0\xFA\x01\0\xE9\xFA\x01\0\xF0\xFA\x01\0\xF9\xFA\x01\0") }, 1179u32)
                },
                unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\xCF\0\0\0\0\0\x05\0\n\0\x10\0\x16\0\x1C\0\"\0(\0.\x004\0:\0@\0F\0L\0R\0X\0^\0d\0j\0p\0v\0|\0\x82\0\x88\0\x8E\0\x94\0\x9A\0\xA0\0\xA6\0\xAC\0\xB2\0\xB8\0\xBE\0\xC4\0\xCA\0\xD0\0\xD6\0\xDC\0\xE2\0\xE8\0\xEE\0\xF4\0\xFA\0\0\x01\x06\x01\x0C\x01\x12\x01\x18\x01\x1E\x01$\x01*\x010\x016\x01<\x01B\x01H\x01N\x01T\x01Z\x01`\x01f\x01l\x01r\x01x\x01~\x01\x84\x01\x8A\x01\x90\x01\x96\x01\x9C\x01\xA2\x01\xA8\x01\xAE\x01\xB4\x01\xBA\x01\xC0\x01\xC6\x01\xCC\x01\xD2\x01\xD8\x01\xDE\x01\xE4\x01\xEA\x01\xF0\x01\xF6\x01\xFC\x01\x02\x02\x08\x02\x0E\x02\x14\x02\x1A\x02 \x02&\x02,\x022\x028\x02>\x02D\x02J\x02P\x02V\x02\\\x02b\x02h\x02n\x02t\x02z\x02\x80\x02\x86\x02\x8C\x02\x92\x02\x99\x02\xA0\x02\xA7\x02\xAE\x02\xB5\x02\xBC\x02\xC3\x02\xCA\x02\xD1\x02\xD8\x02\xDF\x02\xE6\x02\xED\x02\xF4\x02\xFB\x02\x02\x03\t\x03\x10\x03\x17\x03\x1E\x03%\x03,\x033\x03:\x03A\x03H\x03O\x03V\x03]\x03d\x03k\x03r\x03y\x03\x80\x03\x87\x03\x8E\x03\x95\x03\x9C\x03\xA3\x03\xAA\x03\xB1\x03\xB8\x03\xBF\x03\xC6\x03\xCD\x03\xD4\x03\xDB\x03\xE2\x03\xE9\x03\xF0\x03\xF7\x03\xFE\x03\x05\x04\x0C\x04\x13\x04\x1A\x04!\x04(\x04/\x046\x04=\x04D\x04K\x04R\x04Y\x04`\x04g\x04n\x04u\x04|\x04\x83\x04\x8A\x04\x91\x04\x98\x04\x9F\x04\xA6\x04\xAD\x04\xB4\x04\xBB\x04\xC2\x04\xC9\x04\xD0\x04\xD7\x04\xDE\x04\xE5\x04\xEC\x04\xF3\x04\xFA\x04\x01\x05\x08\x05\x0F\x05\x16\x05\x1D\x05$\x05+\x052\x05\xC2\xA9\xEF\xB8\x8F\xC2\xAE\xEF\xB8\x8F\xE2\x80\xBC\xEF\xB8\x8F\xE2\x81\x89\xEF\xB8\x8F\xE2\x84\xA2\xEF\xB8\x8F\xE2\x84\xB9\xEF\xB8\x8F\xE2\x86\x94\xEF\xB8\x8F\xE2\x86\x95\xEF\xB8\x8F\xE2\x86\x96\xEF\xB8\x8F\xE2\x86\x97\xEF\xB8\x8F\xE2\x86\x98\xEF\xB8\x8F\xE2\x86\x99\xEF\xB8\x8F\xE2\x86\xA9\xEF\xB8\x8F\xE2\x86\xAA\xEF\xB8\x8F\xE2\x8C\xA8\xEF\xB8\x8F\xE2\x8F\x8F\xEF\xB8\x8F\xE2\x8F\xAD\xEF\xB8\x8F\xE2\x8F\xAE\xEF\xB8\x8F\xE2\x8F\xAF\xEF\xB8\x8F\xE2\x8F\xB1\xEF\xB8\x8F\xE2\x8F\xB2\xEF\xB8\x8F\xE2\x8F\xB8\xEF\xB8\x8F\xE2\x8F\xB9\xEF\xB8\x8F\xE2\x8F\xBA\xEF\xB8\x8F\xE2\x93\x82\xEF\xB8\x8F\xE2\x96\xAA\xEF\xB8\x8F\xE2\x96\xAB\xEF\xB8\x8F\xE2\x96\xB6\xEF\xB8\x8F\xE2\x97\x80\xEF\xB8\x8F\xE2\x97\xBB\xEF\xB8\x8F\xE2\x97\xBC\xEF\xB8\x8F\xE2\x98\x80\xEF\xB8\x8F\xE2\x98\x81\xEF\xB8\x8F\xE2\x98\x82\xEF\xB8\x8F\xE2\x98\x83\xEF\xB8\x8F\xE2\x98\x84\xEF\xB8\x8F\xE2\x98\x8E\xEF\xB8\x8F\xE2\x98\x91\xEF\xB8\x8F\xE2\x98\x98\xEF\xB8\x8F\xE2\x98\x9D\xEF\xB8\x8F\xE2\x98\xA0\xEF\xB8\x8F\xE2\x98\xA2\xEF\xB8\x8F\xE2\x98\xA3\xEF\xB8\x8F\xE2\x98\xA6\xEF\xB8\x8F\xE2\x98\xAA\xEF\xB8\x8F\xE2\x98\xAE\xEF\xB8\x8F\xE2\x98\xAF\xEF\xB8\x8F\xE2\x98\xB8\xEF\xB8\x8F\xE2\x98\xB9\xEF\xB8\x8F\xE2\x98\xBA\xEF\xB8\x8F\xE2\x99\x80\xEF\xB8\x8F\xE2\x99\x82\xEF\xB8\x8F\xE2\x99\x9F\xEF\xB8\x8F\xE2\x99\xA0\xEF\xB8\x8F\xE2\x99\xA3\xEF\xB8\x8F\xE2\x99\xA5\xEF\xB8\x8F\xE2\x99\xA6\xEF\xB8\x8F\xE2\x99\xA8\xEF\xB8\x8F\xE2\x99\xBB\xEF\xB8\x8F\xE2\x99\xBE\xEF\xB8\x8F\xE2\x9A\x92\xEF\xB8\x8F\xE2\x9A\x94\xEF\xB8\x8F\xE2\x9A\x95\xEF\xB8\x8F\xE2\x9A\x96\xEF\xB8\x8F\xE2\x9A\x97\xEF\xB8\x8F\xE2\x9A\x99\xEF\xB8\x8F\xE2\x9A\x9B\xEF\xB8\x8F\xE2\x9A\x9C\xEF\xB8\x8F\xE2\x9A\xA0\xEF\xB8\x8F\xE2\x9A\xA7\xEF\xB8\x8F\xE2\x9A\xB0\xEF\xB8\x8F\xE2\x9A\xB1\xEF\xB8\x8F\xE2\x9B\x88\xEF\xB8\x8F\xE2\x9B\x8F\xEF\xB8\x8F\xE2\x9B\x91\xEF\xB8\x8F\xE2\x9B\x93\xEF\xB8\x8F\xE2\x9B\xA9\xEF\xB8\x8F\xE2\x9B\xB0\xEF\xB8\x8F\xE2\x9B\xB1\xEF\xB8\x8F\xE2\x9B\xB4\xEF\xB8\x8F\xE2\x9B\xB7\xEF\xB8\x8F\xE2\x9B\xB8\xEF\xB8\x8F\xE2\x9B\xB9\xEF\xB8\x8F\xE2\x9C\x82\xEF\xB8\x8F\xE2\x9C\x88\xEF\xB8\x8F\xE2\x9C\x89\xEF\xB8\x8F\xE2\x9C\x8C\xEF\xB8\x8F\xE2\x9C\x8D\xEF\xB8\x8F\xE2\x9C\x8F\xEF\xB8\x8F\xE2\x9C\x92\xEF\xB8\x8F\xE2\x9C\x94\xEF\xB8\x8F\xE2\x9C\x96\xEF\xB8\x8F\xE2\x9C\x9D\xEF\xB8\x8F\xE2\x9C\xA1\xEF\xB8\x8F\xE2\x9C\xB3\xEF\xB8\x8F\xE2\x9C\xB4\xEF\xB8\x8F\xE2\x9D\x84\xEF\xB8\x8F\xE2\x9D\x87\xEF\xB8\x8F\xE2\x9D\xA3\xEF\xB8\x8F\xE2\x9D\xA4\xEF\xB8\x8F\xE2\x9E\xA1\xEF\xB8\x8F\xE2\xA4\xB4\xEF\xB8\x8F\xE2\xA4\xB5\xEF\xB8\x8F\xE2\xAC\x85\xEF\xB8\x8F\xE2\xAC\x86\xEF\xB8\x8F\xE2\xAC\x87\xEF\xB8\x8F\xE3\x80\xB0\xEF\xB8\x8F\xE3\x80\xBD\xEF\xB8\x8F\xE3\x8A\x97\xEF\xB8\x8F\xE3\x8A\x99\xEF\xB8\x8F\xF0\x9F\x85\xB0\xEF\xB8\x8F\xF0\x9F\x85\xB1\xEF\xB8\x8F\xF0\x9F\x85\xBE\xEF\xB8\x8F\xF0\x9F\x85\xBF\xEF\xB8\x8F\xF0\x9F\x88\x82\xEF\xB8\x8F\xF0\x9F\x88\xB7\xEF\xB8\x8F\xF0\x9F\x8C\xA1\xEF\xB8\x8F\xF0\x9F\x8C\xA4\xEF\xB8\x8F\xF0\x9F\x8C\xA5\xEF\xB8\x8F\xF0\x9F\x8C\xA6\xEF\xB8\x8F\xF0\x9F\x8C\xA7\xEF\xB8\x8F\xF0\x9F\x8C\xA8\xEF\xB8\x8F\xF0\x9F\x8C\xA9\xEF\xB8\x8F\xF0\x9F\x8C\xAA\xEF\xB8\x8F\xF0\x9F\x8C\xAB\xEF\xB8\x8F\xF0\x9F\x8C\xAC\xEF\xB8\x8F\xF0\x9F\x8C\xB6\xEF\xB8\x8F\xF0\x9F\x8D\xBD\xEF\xB8\x8F\xF0\x9F\x8E\x96\xEF\xB8\x8F\xF0\x9F\x8E\x97\xEF\xB8\x8F\xF0\x9F\x8E\x99\xEF\xB8\x8F\xF0\x9F\x8E\x9A\xEF\xB8\x8F\xF0\x9F\x8E\x9B\xEF\xB8\x8F\xF0\x9F\x8E\x9E\xEF\xB8\x8F\xF0\x9F\x8E\x9F\xEF\xB8\x8F\xF0\x9F\x8F\x8B\xEF\xB8\x8F\xF0\x9F\x8F\x8C\xEF\xB8\x8F\xF0\x9F\x8F\x8D\xEF\xB8\x8F\xF0\x9F\x8F\x8E\xEF\xB8\x8F\xF0\x9F\x8F\x94\xEF\xB8\x8F\xF0\x9F\x8F\x95\xEF\xB8\x8F\xF0\x9F\x8F\x96\xEF\xB8\x8F\xF0\x9F\x8F\x97\xEF\xB8\x8F\xF0\x9F\x8F\x98\xEF\xB8\x8F\xF0\x9F\x8F\x99\xEF\xB8\x8F\xF0\x9F\x8F\x9A\xEF\xB8\x8F\xF0\x9F\x8F\x9B\xEF\xB8\x8F\xF0\x9F\x8F\x9C\xEF\xB8\x8F\xF0\x9F\x8F\x9D\xEF\xB8\x8F\xF0\x9F\x8F\x9E\xEF\xB8\x8F\xF0\x9F\x8F\x9F\xEF\xB8\x8F\xF0\x9F\x8F\xB3\xEF\xB8\x8F\xF0\x9F\x8F\xB5\xEF\xB8\x8F\xF0\x9F\x8F\xB7\xEF\xB8\x8F\xF0\x9F\x90\xBF\xEF\xB8\x8F\xF0\x9F\x91\x81\xEF\xB8\x8F\xF0\x9F\x93\xBD\xEF\xB8\x8F\xF0\x9F\x95\x89\xEF\xB8\x8F\xF0\x9F\x95\x8A\xEF\xB8\x8F\xF0\x9F\x95\xAF\xEF\xB8\x8F\xF0\x9F\x95\xB0\xEF\xB8\x8F\xF0\x9F\x95\xB3\xEF\xB8\x8F\xF0\x9F\x95\xB4\xEF\xB8\x8F\xF0\x9F\x95\xB5\xEF\xB8\x8F\xF0\x9F\x95\xB6\xEF\xB8\x8F\xF0\x9F\x95\xB7\xEF\xB8\x8F\xF0\x9F\x95\xB8\xEF\xB8\x8F\xF0\x9F\x95\xB9\xEF\xB8\x8F\xF0\x9F\x96\x87\xEF\xB8\x8F\xF0\x9F\x96\x8A\xEF\xB8\x8F\xF0\x9F\x96\x8B\xEF\xB8\x8F\xF0\x9F\x96\x8C\xEF\xB8\x8F\xF0\x9F\x96\x8D\xEF\xB8\x8F\xF0\x9F\x96\x90\xEF\xB8\x8F\xF0\x9F\x96\xA5\xEF\xB8\x8F\xF0\x9F\x96\xA8\xEF\xB8\x8F\xF0\x9F\x96\xB1\xEF\xB8\x8F\xF0\x9F\x96\xB2\xEF\xB8\x8F\xF0\x9F\x96\xBC\xEF\xB8\x8F\xF0\x9F\x97\x82\xEF\xB8\x8F\xF0\x9F\x97\x83\xEF\xB8\x8F\xF0\x9F\x97\x84\xEF\xB8\x8F\xF0\x9F\x97\x91\xEF\xB8\x8F\xF0\x9F\x97\x92\xEF\xB8\x8F\xF0\x9F\x97\x93\xEF\xB8\x8F\xF0\x9F\x97\x9C\xEF\xB8\x8F\xF0\x9F\x97\x9D\xEF\xB8\x8F\xF0\x9F\x97\x9E\xEF\xB8\x8F\xF0\x9F\x97\xA1\xEF\xB8\x8F\xF0\x9F\x97\xA3\xEF\xB8\x8F\xF0\x9F\x97\xA8\xEF\xB8\x8F\xF0\x9F\x97\xAF\xEF\xB8\x8F\xF0\x9F\x97\xB3\xEF\xB8\x8F\xF0\x9F\x97\xBA\xEF\xB8\x8F\xF0\x9F\x9B\x8B\xEF\xB8\x8F\xF0\x9F\x9B\x8D\xEF\xB8\x8F\xF0\x9F\x9B\x8E\xEF\xB8\x8F\xF0\x9F\x9B\x8F\xEF\xB8\x8F\xF0\x9F\x9B\xA0\xEF\xB8\x8F\xF0\x9F\x9B\xA1\xEF\xB8\x8F\xF0\x9F\x9B\xA2\xEF\xB8\x8F\xF0\x9F\x9B\xA3\xEF\xB8\x8F\xF0\x9F\x9B\xA4\xEF\xB8\x8F\xF0\x9F\x9B\xA5\xEF\xB8\x8F\xF0\x9F\x9B\xA9\xEF\xB8\x8F\xF0\x9F\x9B\xB0\xEF\xB8\x8F\xF0\x9F\x9B\xB3\xEF\xB8\x8F") },
            ));
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::BasicEmojiV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::BasicEmojiV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_BASIC_EMOJI_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::BasicEmojiV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
