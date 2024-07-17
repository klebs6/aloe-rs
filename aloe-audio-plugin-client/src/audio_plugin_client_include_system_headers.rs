crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/utility/aloe_IncludeSystemHeaders.h]

#[cfg(target_os="windows")]
pub const _WIN32_WINNT: usize = 0x500;

pub const STRICT: usize = 1;

#[cfg(any(target_os="macos",target_os="ios"))]
#[cfg(not(any(ALOE_SUPPORT_CARBON,__LP64__)))]
pub const ALOE_SUPPORT_CARBON: usize = 1;
