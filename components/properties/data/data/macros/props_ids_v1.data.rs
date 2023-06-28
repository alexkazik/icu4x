// @generated
/// Implement [`DataProvider<IdStartV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_ids_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_IDS_V1: &'static <icu_properties::provider::IdStartV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0a\0\0\0{\0\0\0\xAA\0\0\0\xAB\0\0\0\xB5\0\0\0\xB6\0\0\0\xBA\0\0\0\xBB\0\0\0\xC0\0\0\0\xD7\0\0\0\xD8\0\0\0\xF7\0\0\0\xF8\0\0\0\xC2\x02\0\0\xC6\x02\0\0\xD2\x02\0\0\xE0\x02\0\0\xE5\x02\0\0\xEC\x02\0\0\xED\x02\0\0\xEE\x02\0\0\xEF\x02\0\0p\x03\0\0u\x03\0\0v\x03\0\0x\x03\0\0z\x03\0\0~\x03\0\0\x7F\x03\0\0\x80\x03\0\0\x86\x03\0\0\x87\x03\0\0\x88\x03\0\0\x8B\x03\0\0\x8C\x03\0\0\x8D\x03\0\0\x8E\x03\0\0\xA2\x03\0\0\xA3\x03\0\0\xF6\x03\0\0\xF7\x03\0\0\x82\x04\0\0\x8A\x04\0\x000\x05\0\x001\x05\0\0W\x05\0\0Y\x05\0\0Z\x05\0\0`\x05\0\0\x89\x05\0\0\xD0\x05\0\0\xEB\x05\0\0\xEF\x05\0\0\xF3\x05\0\0 \x06\0\0K\x06\0\0n\x06\0\0p\x06\0\0q\x06\0\0\xD4\x06\0\0\xD5\x06\0\0\xD6\x06\0\0\xE5\x06\0\0\xE7\x06\0\0\xEE\x06\0\0\xF0\x06\0\0\xFA\x06\0\0\xFD\x06\0\0\xFF\x06\0\0\0\x07\0\0\x10\x07\0\0\x11\x07\0\0\x12\x07\0\x000\x07\0\0M\x07\0\0\xA6\x07\0\0\xB1\x07\0\0\xB2\x07\0\0\xCA\x07\0\0\xEB\x07\0\0\xF4\x07\0\0\xF6\x07\0\0\xFA\x07\0\0\xFB\x07\0\0\0\x08\0\0\x16\x08\0\0\x1A\x08\0\0\x1B\x08\0\0$\x08\0\0%\x08\0\0(\x08\0\0)\x08\0\0@\x08\0\0Y\x08\0\0`\x08\0\0k\x08\0\0p\x08\0\0\x88\x08\0\0\x89\x08\0\0\x8F\x08\0\0\xA0\x08\0\0\xCA\x08\0\0\x04\t\0\0:\t\0\0=\t\0\0>\t\0\0P\t\0\0Q\t\0\0X\t\0\0b\t\0\0q\t\0\0\x81\t\0\0\x85\t\0\0\x8D\t\0\0\x8F\t\0\0\x91\t\0\0\x93\t\0\0\xA9\t\0\0\xAA\t\0\0\xB1\t\0\0\xB2\t\0\0\xB3\t\0\0\xB6\t\0\0\xBA\t\0\0\xBD\t\0\0\xBE\t\0\0\xCE\t\0\0\xCF\t\0\0\xDC\t\0\0\xDE\t\0\0\xDF\t\0\0\xE2\t\0\0\xF0\t\0\0\xF2\t\0\0\xFC\t\0\0\xFD\t\0\0\x05\n\0\0\x0B\n\0\0\x0F\n\0\0\x11\n\0\0\x13\n\0\0)\n\0\0*\n\0\x001\n\0\x002\n\0\x004\n\0\x005\n\0\x007\n\0\08\n\0\0:\n\0\0Y\n\0\0]\n\0\0^\n\0\0_\n\0\0r\n\0\0u\n\0\0\x85\n\0\0\x8E\n\0\0\x8F\n\0\0\x92\n\0\0\x93\n\0\0\xA9\n\0\0\xAA\n\0\0\xB1\n\0\0\xB2\n\0\0\xB4\n\0\0\xB5\n\0\0\xBA\n\0\0\xBD\n\0\0\xBE\n\0\0\xD0\n\0\0\xD1\n\0\0\xE0\n\0\0\xE2\n\0\0\xF9\n\0\0\xFA\n\0\0\x05\x0B\0\0\r\x0B\0\0\x0F\x0B\0\0\x11\x0B\0\0\x13\x0B\0\0)\x0B\0\0*\x0B\0\x001\x0B\0\x002\x0B\0\x004\x0B\0\x005\x0B\0\0:\x0B\0\0=\x0B\0\0>\x0B\0\0\\\x0B\0\0^\x0B\0\0_\x0B\0\0b\x0B\0\0q\x0B\0\0r\x0B\0\0\x83\x0B\0\0\x84\x0B\0\0\x85\x0B\0\0\x8B\x0B\0\0\x8E\x0B\0\0\x91\x0B\0\0\x92\x0B\0\0\x96\x0B\0\0\x99\x0B\0\0\x9B\x0B\0\0\x9C\x0B\0\0\x9D\x0B\0\0\x9E\x0B\0\0\xA0\x0B\0\0\xA3\x0B\0\0\xA5\x0B\0\0\xA8\x0B\0\0\xAB\x0B\0\0\xAE\x0B\0\0\xBA\x0B\0\0\xD0\x0B\0\0\xD1\x0B\0\0\x05\x0C\0\0\r\x0C\0\0\x0E\x0C\0\0\x11\x0C\0\0\x12\x0C\0\0)\x0C\0\0*\x0C\0\0:\x0C\0\0=\x0C\0\0>\x0C\0\0X\x0C\0\0[\x0C\0\0]\x0C\0\0^\x0C\0\0`\x0C\0\0b\x0C\0\0\x80\x0C\0\0\x81\x0C\0\0\x85\x0C\0\0\x8D\x0C\0\0\x8E\x0C\0\0\x91\x0C\0\0\x92\x0C\0\0\xA9\x0C\0\0\xAA\x0C\0\0\xB4\x0C\0\0\xB5\x0C\0\0\xBA\x0C\0\0\xBD\x0C\0\0\xBE\x0C\0\0\xDD\x0C\0\0\xDF\x0C\0\0\xE0\x0C\0\0\xE2\x0C\0\0\xF1\x0C\0\0\xF3\x0C\0\0\x04\r\0\0\r\r\0\0\x0E\r\0\0\x11\r\0\0\x12\r\0\0;\r\0\0=\r\0\0>\r\0\0N\r\0\0O\r\0\0T\r\0\0W\r\0\0_\r\0\0b\r\0\0z\r\0\0\x80\r\0\0\x85\r\0\0\x97\r\0\0\x9A\r\0\0\xB2\r\0\0\xB3\r\0\0\xBC\r\0\0\xBD\r\0\0\xBE\r\0\0\xC0\r\0\0\xC7\r\0\0\x01\x0E\0\x001\x0E\0\x002\x0E\0\x004\x0E\0\0@\x0E\0\0G\x0E\0\0\x81\x0E\0\0\x83\x0E\0\0\x84\x0E\0\0\x85\x0E\0\0\x86\x0E\0\0\x8B\x0E\0\0\x8C\x0E\0\0\xA4\x0E\0\0\xA5\x0E\0\0\xA6\x0E\0\0\xA7\x0E\0\0\xB1\x0E\0\0\xB2\x0E\0\0\xB4\x0E\0\0\xBD\x0E\0\0\xBE\x0E\0\0\xC0\x0E\0\0\xC5\x0E\0\0\xC6\x0E\0\0\xC7\x0E\0\0\xDC\x0E\0\0\xE0\x0E\0\0\0\x0F\0\0\x01\x0F\0\0@\x0F\0\0H\x0F\0\0I\x0F\0\0m\x0F\0\0\x88\x0F\0\0\x8D\x0F\0\0\0\x10\0\0+\x10\0\0?\x10\0\0@\x10\0\0P\x10\0\0V\x10\0\0Z\x10\0\0^\x10\0\0a\x10\0\0b\x10\0\0e\x10\0\0g\x10\0\0n\x10\0\0q\x10\0\0u\x10\0\0\x82\x10\0\0\x8E\x10\0\0\x8F\x10\0\0\xA0\x10\0\0\xC6\x10\0\0\xC7\x10\0\0\xC8\x10\0\0\xCD\x10\0\0\xCE\x10\0\0\xD0\x10\0\0\xFB\x10\0\0\xFC\x10\0\0I\x12\0\0J\x12\0\0N\x12\0\0P\x12\0\0W\x12\0\0X\x12\0\0Y\x12\0\0Z\x12\0\0^\x12\0\0`\x12\0\0\x89\x12\0\0\x8A\x12\0\0\x8E\x12\0\0\x90\x12\0\0\xB1\x12\0\0\xB2\x12\0\0\xB6\x12\0\0\xB8\x12\0\0\xBF\x12\0\0\xC0\x12\0\0\xC1\x12\0\0\xC2\x12\0\0\xC6\x12\0\0\xC8\x12\0\0\xD7\x12\0\0\xD8\x12\0\0\x11\x13\0\0\x12\x13\0\0\x16\x13\0\0\x18\x13\0\0[\x13\0\0\x80\x13\0\0\x90\x13\0\0\xA0\x13\0\0\xF6\x13\0\0\xF8\x13\0\0\xFE\x13\0\0\x01\x14\0\0m\x16\0\0o\x16\0\0\x80\x16\0\0\x81\x16\0\0\x9B\x16\0\0\xA0\x16\0\0\xEB\x16\0\0\xEE\x16\0\0\xF9\x16\0\0\0\x17\0\0\x12\x17\0\0\x1F\x17\0\x002\x17\0\0@\x17\0\0R\x17\0\0`\x17\0\0m\x17\0\0n\x17\0\0q\x17\0\0\x80\x17\0\0\xB4\x17\0\0\xD7\x17\0\0\xD8\x17\0\0\xDC\x17\0\0\xDD\x17\0\0 \x18\0\0y\x18\0\0\x80\x18\0\0\xA9\x18\0\0\xAA\x18\0\0\xAB\x18\0\0\xB0\x18\0\0\xF6\x18\0\0\0\x19\0\0\x1F\x19\0\0P\x19\0\0n\x19\0\0p\x19\0\0u\x19\0\0\x80\x19\0\0\xAC\x19\0\0\xB0\x19\0\0\xCA\x19\0\0\0\x1A\0\0\x17\x1A\0\0 \x1A\0\0U\x1A\0\0\xA7\x1A\0\0\xA8\x1A\0\0\x05\x1B\0\x004\x1B\0\0E\x1B\0\0M\x1B\0\0\x83\x1B\0\0\xA1\x1B\0\0\xAE\x1B\0\0\xB0\x1B\0\0\xBA\x1B\0\0\xE6\x1B\0\0\0\x1C\0\0$\x1C\0\0M\x1C\0\0P\x1C\0\0Z\x1C\0\0~\x1C\0\0\x80\x1C\0\0\x89\x1C\0\0\x90\x1C\0\0\xBB\x1C\0\0\xBD\x1C\0\0\xC0\x1C\0\0\xE9\x1C\0\0\xED\x1C\0\0\xEE\x1C\0\0\xF4\x1C\0\0\xF5\x1C\0\0\xF7\x1C\0\0\xFA\x1C\0\0\xFB\x1C\0\0\0\x1D\0\0\xC0\x1D\0\0\0\x1E\0\0\x16\x1F\0\0\x18\x1F\0\0\x1E\x1F\0\0 \x1F\0\0F\x1F\0\0H\x1F\0\0N\x1F\0\0P\x1F\0\0X\x1F\0\0Y\x1F\0\0Z\x1F\0\0[\x1F\0\0\\\x1F\0\0]\x1F\0\0^\x1F\0\0_\x1F\0\0~\x1F\0\0\x80\x1F\0\0\xB5\x1F\0\0\xB6\x1F\0\0\xBD\x1F\0\0\xBE\x1F\0\0\xBF\x1F\0\0\xC2\x1F\0\0\xC5\x1F\0\0\xC6\x1F\0\0\xCD\x1F\0\0\xD0\x1F\0\0\xD4\x1F\0\0\xD6\x1F\0\0\xDC\x1F\0\0\xE0\x1F\0\0\xED\x1F\0\0\xF2\x1F\0\0\xF5\x1F\0\0\xF6\x1F\0\0\xFD\x1F\0\0q \0\0r \0\0\x7F \0\0\x80 \0\0\x90 \0\0\x9D \0\0\x02!\0\0\x03!\0\0\x07!\0\0\x08!\0\0\n!\0\0\x14!\0\0\x15!\0\0\x16!\0\0\x18!\0\0\x1E!\0\0$!\0\0%!\0\0&!\0\0'!\0\0(!\0\0)!\0\0*!\0\0:!\0\0<!\0\0@!\0\0E!\0\0J!\0\0N!\0\0O!\0\0`!\0\0\x89!\0\0\0,\0\0\xE5,\0\0\xEB,\0\0\xEF,\0\0\xF2,\0\0\xF4,\0\0\0-\0\0&-\0\0'-\0\0(-\0\0--\0\0.-\0\x000-\0\0h-\0\0o-\0\0p-\0\0\x80-\0\0\x97-\0\0\xA0-\0\0\xA7-\0\0\xA8-\0\0\xAF-\0\0\xB0-\0\0\xB7-\0\0\xB8-\0\0\xBF-\0\0\xC0-\0\0\xC7-\0\0\xC8-\0\0\xCF-\0\0\xD0-\0\0\xD7-\0\0\xD8-\0\0\xDF-\0\0\x050\0\0\x080\0\0!0\0\0*0\0\x0010\0\x0060\0\080\0\0=0\0\0A0\0\0\x970\0\0\x9B0\0\0\xA00\0\0\xA10\0\0\xFB0\0\0\xFC0\0\0\x001\0\0\x051\0\x0001\0\x0011\0\0\x8F1\0\0\xA01\0\0\xC01\0\0\xF01\0\0\x002\0\0\x004\0\0\xC0M\0\0\0N\0\0\x8D\xA4\0\0\xD0\xA4\0\0\xFE\xA4\0\0\0\xA5\0\0\r\xA6\0\0\x10\xA6\0\0 \xA6\0\0*\xA6\0\0,\xA6\0\0@\xA6\0\0o\xA6\0\0\x7F\xA6\0\0\x9E\xA6\0\0\xA0\xA6\0\0\xF0\xA6\0\0\x17\xA7\0\0 \xA7\0\0\"\xA7\0\0\x89\xA7\0\0\x8B\xA7\0\0\xCB\xA7\0\0\xD0\xA7\0\0\xD2\xA7\0\0\xD3\xA7\0\0\xD4\xA7\0\0\xD5\xA7\0\0\xDA\xA7\0\0\xF2\xA7\0\0\x02\xA8\0\0\x03\xA8\0\0\x06\xA8\0\0\x07\xA8\0\0\x0B\xA8\0\0\x0C\xA8\0\0#\xA8\0\0@\xA8\0\0t\xA8\0\0\x82\xA8\0\0\xB4\xA8\0\0\xF2\xA8\0\0\xF8\xA8\0\0\xFB\xA8\0\0\xFC\xA8\0\0\xFD\xA8\0\0\xFF\xA8\0\0\n\xA9\0\0&\xA9\0\x000\xA9\0\0G\xA9\0\0`\xA9\0\0}\xA9\0\0\x84\xA9\0\0\xB3\xA9\0\0\xCF\xA9\0\0\xD0\xA9\0\0\xE0\xA9\0\0\xE5\xA9\0\0\xE6\xA9\0\0\xF0\xA9\0\0\xFA\xA9\0\0\xFF\xA9\0\0\0\xAA\0\0)\xAA\0\0@\xAA\0\0C\xAA\0\0D\xAA\0\0L\xAA\0\0`\xAA\0\0w\xAA\0\0z\xAA\0\0{\xAA\0\0~\xAA\0\0\xB0\xAA\0\0\xB1\xAA\0\0\xB2\xAA\0\0\xB5\xAA\0\0\xB7\xAA\0\0\xB9\xAA\0\0\xBE\xAA\0\0\xC0\xAA\0\0\xC1\xAA\0\0\xC2\xAA\0\0\xC3\xAA\0\0\xDB\xAA\0\0\xDE\xAA\0\0\xE0\xAA\0\0\xEB\xAA\0\0\xF2\xAA\0\0\xF5\xAA\0\0\x01\xAB\0\0\x07\xAB\0\0\t\xAB\0\0\x0F\xAB\0\0\x11\xAB\0\0\x17\xAB\0\0 \xAB\0\0'\xAB\0\0(\xAB\0\0/\xAB\0\x000\xAB\0\0[\xAB\0\0\\\xAB\0\0j\xAB\0\0p\xAB\0\0\xE3\xAB\0\0\0\xAC\0\0\xA4\xD7\0\0\xB0\xD7\0\0\xC7\xD7\0\0\xCB\xD7\0\0\xFC\xD7\0\0\0\xF9\0\0n\xFA\0\0p\xFA\0\0\xDA\xFA\0\0\0\xFB\0\0\x07\xFB\0\0\x13\xFB\0\0\x18\xFB\0\0\x1D\xFB\0\0\x1E\xFB\0\0\x1F\xFB\0\0)\xFB\0\0*\xFB\0\x007\xFB\0\08\xFB\0\0=\xFB\0\0>\xFB\0\0?\xFB\0\0@\xFB\0\0B\xFB\0\0C\xFB\0\0E\xFB\0\0F\xFB\0\0\xB2\xFB\0\0\xD3\xFB\0\0>\xFD\0\0P\xFD\0\0\x90\xFD\0\0\x92\xFD\0\0\xC8\xFD\0\0\xF0\xFD\0\0\xFC\xFD\0\0p\xFE\0\0u\xFE\0\0v\xFE\0\0\xFD\xFE\0\0!\xFF\0\0;\xFF\0\0A\xFF\0\0[\xFF\0\0f\xFF\0\0\xBF\xFF\0\0\xC2\xFF\0\0\xC8\xFF\0\0\xCA\xFF\0\0\xD0\xFF\0\0\xD2\xFF\0\0\xD8\xFF\0\0\xDA\xFF\0\0\xDD\xFF\0\0\0\0\x01\0\x0C\0\x01\0\r\0\x01\0'\0\x01\0(\0\x01\0;\0\x01\0<\0\x01\0>\0\x01\0?\0\x01\0N\0\x01\0P\0\x01\0^\0\x01\0\x80\0\x01\0\xFB\0\x01\0@\x01\x01\0u\x01\x01\0\x80\x02\x01\0\x9D\x02\x01\0\xA0\x02\x01\0\xD1\x02\x01\0\0\x03\x01\0 \x03\x01\0-\x03\x01\0K\x03\x01\0P\x03\x01\0v\x03\x01\0\x80\x03\x01\0\x9E\x03\x01\0\xA0\x03\x01\0\xC4\x03\x01\0\xC8\x03\x01\0\xD0\x03\x01\0\xD1\x03\x01\0\xD6\x03\x01\0\0\x04\x01\0\x9E\x04\x01\0\xB0\x04\x01\0\xD4\x04\x01\0\xD8\x04\x01\0\xFC\x04\x01\0\0\x05\x01\0(\x05\x01\x000\x05\x01\0d\x05\x01\0p\x05\x01\0{\x05\x01\0|\x05\x01\0\x8B\x05\x01\0\x8C\x05\x01\0\x93\x05\x01\0\x94\x05\x01\0\x96\x05\x01\0\x97\x05\x01\0\xA2\x05\x01\0\xA3\x05\x01\0\xB2\x05\x01\0\xB3\x05\x01\0\xBA\x05\x01\0\xBB\x05\x01\0\xBD\x05\x01\0\0\x06\x01\x007\x07\x01\0@\x07\x01\0V\x07\x01\0`\x07\x01\0h\x07\x01\0\x80\x07\x01\0\x86\x07\x01\0\x87\x07\x01\0\xB1\x07\x01\0\xB2\x07\x01\0\xBB\x07\x01\0\0\x08\x01\0\x06\x08\x01\0\x08\x08\x01\0\t\x08\x01\0\n\x08\x01\x006\x08\x01\x007\x08\x01\09\x08\x01\0<\x08\x01\0=\x08\x01\0?\x08\x01\0V\x08\x01\0`\x08\x01\0w\x08\x01\0\x80\x08\x01\0\x9F\x08\x01\0\xE0\x08\x01\0\xF3\x08\x01\0\xF4\x08\x01\0\xF6\x08\x01\0\0\t\x01\0\x16\t\x01\0 \t\x01\0:\t\x01\0\x80\t\x01\0\xB8\t\x01\0\xBE\t\x01\0\xC0\t\x01\0\0\n\x01\0\x01\n\x01\0\x10\n\x01\0\x14\n\x01\0\x15\n\x01\0\x18\n\x01\0\x19\n\x01\x006\n\x01\0`\n\x01\0}\n\x01\0\x80\n\x01\0\x9D\n\x01\0\xC0\n\x01\0\xC8\n\x01\0\xC9\n\x01\0\xE5\n\x01\0\0\x0B\x01\x006\x0B\x01\0@\x0B\x01\0V\x0B\x01\0`\x0B\x01\0s\x0B\x01\0\x80\x0B\x01\0\x92\x0B\x01\0\0\x0C\x01\0I\x0C\x01\0\x80\x0C\x01\0\xB3\x0C\x01\0\xC0\x0C\x01\0\xF3\x0C\x01\0\0\r\x01\0$\r\x01\0\x80\x0E\x01\0\xAA\x0E\x01\0\xB0\x0E\x01\0\xB2\x0E\x01\0\0\x0F\x01\0\x1D\x0F\x01\0'\x0F\x01\0(\x0F\x01\x000\x0F\x01\0F\x0F\x01\0p\x0F\x01\0\x82\x0F\x01\0\xB0\x0F\x01\0\xC5\x0F\x01\0\xE0\x0F\x01\0\xF7\x0F\x01\0\x03\x10\x01\08\x10\x01\0q\x10\x01\0s\x10\x01\0u\x10\x01\0v\x10\x01\0\x83\x10\x01\0\xB0\x10\x01\0\xD0\x10\x01\0\xE9\x10\x01\0\x03\x11\x01\0'\x11\x01\0D\x11\x01\0E\x11\x01\0G\x11\x01\0H\x11\x01\0P\x11\x01\0s\x11\x01\0v\x11\x01\0w\x11\x01\0\x83\x11\x01\0\xB3\x11\x01\0\xC1\x11\x01\0\xC5\x11\x01\0\xDA\x11\x01\0\xDB\x11\x01\0\xDC\x11\x01\0\xDD\x11\x01\0\0\x12\x01\0\x12\x12\x01\0\x13\x12\x01\0,\x12\x01\0?\x12\x01\0A\x12\x01\0\x80\x12\x01\0\x87\x12\x01\0\x88\x12\x01\0\x89\x12\x01\0\x8A\x12\x01\0\x8E\x12\x01\0\x8F\x12\x01\0\x9E\x12\x01\0\x9F\x12\x01\0\xA9\x12\x01\0\xB0\x12\x01\0\xDF\x12\x01\0\x05\x13\x01\0\r\x13\x01\0\x0F\x13\x01\0\x11\x13\x01\0\x13\x13\x01\0)\x13\x01\0*\x13\x01\x001\x13\x01\x002\x13\x01\x004\x13\x01\x005\x13\x01\0:\x13\x01\0=\x13\x01\0>\x13\x01\0P\x13\x01\0Q\x13\x01\0]\x13\x01\0b\x13\x01\0\0\x14\x01\x005\x14\x01\0G\x14\x01\0K\x14\x01\0_\x14\x01\0b\x14\x01\0\x80\x14\x01\0\xB0\x14\x01\0\xC4\x14\x01\0\xC6\x14\x01\0\xC7\x14\x01\0\xC8\x14\x01\0\x80\x15\x01\0\xAF\x15\x01\0\xD8\x15\x01\0\xDC\x15\x01\0\0\x16\x01\x000\x16\x01\0D\x16\x01\0E\x16\x01\0\x80\x16\x01\0\xAB\x16\x01\0\xB8\x16\x01\0\xB9\x16\x01\0\0\x17\x01\0\x1B\x17\x01\0@\x17\x01\0G\x17\x01\0\0\x18\x01\0,\x18\x01\0\xA0\x18\x01\0\xE0\x18\x01\0\xFF\x18\x01\0\x07\x19\x01\0\t\x19\x01\0\n\x19\x01\0\x0C\x19\x01\0\x14\x19\x01\0\x15\x19\x01\0\x17\x19\x01\0\x18\x19\x01\x000\x19\x01\0?\x19\x01\0@\x19\x01\0A\x19\x01\0B\x19\x01\0\xA0\x19\x01\0\xA8\x19\x01\0\xAA\x19\x01\0\xD1\x19\x01\0\xE1\x19\x01\0\xE2\x19\x01\0\xE3\x19\x01\0\xE4\x19\x01\0\0\x1A\x01\0\x01\x1A\x01\0\x0B\x1A\x01\x003\x1A\x01\0:\x1A\x01\0;\x1A\x01\0P\x1A\x01\0Q\x1A\x01\0\\\x1A\x01\0\x8A\x1A\x01\0\x9D\x1A\x01\0\x9E\x1A\x01\0\xB0\x1A\x01\0\xF9\x1A\x01\0\0\x1C\x01\0\t\x1C\x01\0\n\x1C\x01\0/\x1C\x01\0@\x1C\x01\0A\x1C\x01\0r\x1C\x01\0\x90\x1C\x01\0\0\x1D\x01\0\x07\x1D\x01\0\x08\x1D\x01\0\n\x1D\x01\0\x0B\x1D\x01\x001\x1D\x01\0F\x1D\x01\0G\x1D\x01\0`\x1D\x01\0f\x1D\x01\0g\x1D\x01\0i\x1D\x01\0j\x1D\x01\0\x8A\x1D\x01\0\x98\x1D\x01\0\x99\x1D\x01\0\xE0\x1E\x01\0\xF3\x1E\x01\0\x02\x1F\x01\0\x03\x1F\x01\0\x04\x1F\x01\0\x11\x1F\x01\0\x12\x1F\x01\x004\x1F\x01\0\xB0\x1F\x01\0\xB1\x1F\x01\0\0 \x01\0\x9A#\x01\0\0$\x01\0o$\x01\0\x80$\x01\0D%\x01\0\x90/\x01\0\xF1/\x01\0\x000\x01\x0004\x01\0A4\x01\0G4\x01\0\0D\x01\0GF\x01\0\0h\x01\09j\x01\0@j\x01\0_j\x01\0pj\x01\0\xBFj\x01\0\xD0j\x01\0\xEEj\x01\0\0k\x01\x000k\x01\0@k\x01\0Dk\x01\0ck\x01\0xk\x01\0}k\x01\0\x90k\x01\0@n\x01\0\x80n\x01\0\0o\x01\0Ko\x01\0Po\x01\0Qo\x01\0\x93o\x01\0\xA0o\x01\0\xE0o\x01\0\xE2o\x01\0\xE3o\x01\0\xE4o\x01\0\0p\x01\0\xF8\x87\x01\0\0\x88\x01\0\xD6\x8C\x01\0\0\x8D\x01\0\t\x8D\x01\0\xF0\xAF\x01\0\xF4\xAF\x01\0\xF5\xAF\x01\0\xFC\xAF\x01\0\xFD\xAF\x01\0\xFF\xAF\x01\0\0\xB0\x01\0#\xB1\x01\x002\xB1\x01\x003\xB1\x01\0P\xB1\x01\0S\xB1\x01\0U\xB1\x01\0V\xB1\x01\0d\xB1\x01\0h\xB1\x01\0p\xB1\x01\0\xFC\xB2\x01\0\0\xBC\x01\0k\xBC\x01\0p\xBC\x01\0}\xBC\x01\0\x80\xBC\x01\0\x89\xBC\x01\0\x90\xBC\x01\0\x9A\xBC\x01\0\0\xD4\x01\0U\xD4\x01\0V\xD4\x01\0\x9D\xD4\x01\0\x9E\xD4\x01\0\xA0\xD4\x01\0\xA2\xD4\x01\0\xA3\xD4\x01\0\xA5\xD4\x01\0\xA7\xD4\x01\0\xA9\xD4\x01\0\xAD\xD4\x01\0\xAE\xD4\x01\0\xBA\xD4\x01\0\xBB\xD4\x01\0\xBC\xD4\x01\0\xBD\xD4\x01\0\xC4\xD4\x01\0\xC5\xD4\x01\0\x06\xD5\x01\0\x07\xD5\x01\0\x0B\xD5\x01\0\r\xD5\x01\0\x15\xD5\x01\0\x16\xD5\x01\0\x1D\xD5\x01\0\x1E\xD5\x01\0:\xD5\x01\0;\xD5\x01\0?\xD5\x01\0@\xD5\x01\0E\xD5\x01\0F\xD5\x01\0G\xD5\x01\0J\xD5\x01\0Q\xD5\x01\0R\xD5\x01\0\xA6\xD6\x01\0\xA8\xD6\x01\0\xC1\xD6\x01\0\xC2\xD6\x01\0\xDB\xD6\x01\0\xDC\xD6\x01\0\xFB\xD6\x01\0\xFC\xD6\x01\0\x15\xD7\x01\0\x16\xD7\x01\x005\xD7\x01\x006\xD7\x01\0O\xD7\x01\0P\xD7\x01\0o\xD7\x01\0p\xD7\x01\0\x89\xD7\x01\0\x8A\xD7\x01\0\xA9\xD7\x01\0\xAA\xD7\x01\0\xC3\xD7\x01\0\xC4\xD7\x01\0\xCC\xD7\x01\0\0\xDF\x01\0\x1F\xDF\x01\0%\xDF\x01\0+\xDF\x01\x000\xE0\x01\0n\xE0\x01\0\0\xE1\x01\0-\xE1\x01\x007\xE1\x01\0>\xE1\x01\0N\xE1\x01\0O\xE1\x01\0\x90\xE2\x01\0\xAE\xE2\x01\0\xC0\xE2\x01\0\xEC\xE2\x01\0\xD0\xE4\x01\0\xEC\xE4\x01\0\xE0\xE7\x01\0\xE7\xE7\x01\0\xE8\xE7\x01\0\xEC\xE7\x01\0\xED\xE7\x01\0\xEF\xE7\x01\0\xF0\xE7\x01\0\xFF\xE7\x01\0\0\xE8\x01\0\xC5\xE8\x01\0\0\xE9\x01\0D\xE9\x01\0K\xE9\x01\0L\xE9\x01\0\0\xEE\x01\0\x04\xEE\x01\0\x05\xEE\x01\0 \xEE\x01\0!\xEE\x01\0#\xEE\x01\0$\xEE\x01\0%\xEE\x01\0'\xEE\x01\0(\xEE\x01\0)\xEE\x01\x003\xEE\x01\x004\xEE\x01\08\xEE\x01\09\xEE\x01\0:\xEE\x01\0;\xEE\x01\0<\xEE\x01\0B\xEE\x01\0C\xEE\x01\0G\xEE\x01\0H\xEE\x01\0I\xEE\x01\0J\xEE\x01\0K\xEE\x01\0L\xEE\x01\0M\xEE\x01\0P\xEE\x01\0Q\xEE\x01\0S\xEE\x01\0T\xEE\x01\0U\xEE\x01\0W\xEE\x01\0X\xEE\x01\0Y\xEE\x01\0Z\xEE\x01\0[\xEE\x01\0\\\xEE\x01\0]\xEE\x01\0^\xEE\x01\0_\xEE\x01\0`\xEE\x01\0a\xEE\x01\0c\xEE\x01\0d\xEE\x01\0e\xEE\x01\0g\xEE\x01\0k\xEE\x01\0l\xEE\x01\0s\xEE\x01\0t\xEE\x01\0x\xEE\x01\0y\xEE\x01\0}\xEE\x01\0~\xEE\x01\0\x7F\xEE\x01\0\x80\xEE\x01\0\x8A\xEE\x01\0\x8B\xEE\x01\0\x9C\xEE\x01\0\xA1\xEE\x01\0\xA4\xEE\x01\0\xA5\xEE\x01\0\xAA\xEE\x01\0\xAB\xEE\x01\0\xBC\xEE\x01\0\0\0\x02\0\xE0\xA6\x02\0\0\xA7\x02\0:\xB7\x02\0@\xB7\x02\0\x1E\xB8\x02\0 \xB8\x02\0\xA2\xCE\x02\0\xB0\xCE\x02\0\xE1\xEB\x02\0\0\xF8\x02\0\x1E\xFA\x02\0\0\0\x03\0K\x13\x03\0P\x13\x03\0\xB0#\x03\0") }, 136345u32)
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::IdStartV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::IdStartV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_IDS_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::IdStartV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
