crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/ftypes.h]
#[inline] pub fn steinberg_str_empty(str_: *const u16) -> bool {
    
    todo!();
        /*
            return (!str || *str == 0);
        */
}

#[inline] pub fn steinberg_str_8empty(str_: *const u8) -> bool {
    
    todo!();
        /*
            return (!str || *str == 0);
        */
}

#[inline] pub fn steinberg_str_16empty(str_: *const u16) -> bool {
    
    todo!();
        /*
            return (!str || *str == 0);
        */
}

//#define UNICODE_OFF   // disable / enable unicode

#[cfg(not(UNICODE_OFF))] pub const UNICODE: usize = 1;
#[cfg(UNICODE)]          pub const _UNICODE: usize = 1;


