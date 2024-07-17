crate::ix!();

#[cfg(Vst_VERSION_LT_0x030608)]
macro_rules! k_ambi1st_orderacn {
    () => {
        /*
                kBFormat
        */
    }
}

#[inline] pub fn to_string128(
    result: String128,
    source: &str
) {
    
    todo!();
        /*
            typename UString (result, 128).assign (toString (source));
        */
}

#[cfg(target_os="windows")]
pub const DEFAULT_VST3WINDOW_TYPE: FIDString = PLATFORM_TYPE_HWND;

#[cfg(target_os="macos")]
pub const DEFAULT_VST3WINDOW_TYPE: FIDString = PLATFORM_TYPE_NS_VIEW;

#[cfg(any(target_os="linux",target_os="bsd"))]
pub const DEFAULT_VST3WINDOW_TYPE: FIDString = PLATFORM_TYPE_X11_EMBED_WINDOW_ID;
