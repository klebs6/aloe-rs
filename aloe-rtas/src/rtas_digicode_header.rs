crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/RTAS/aloe_RTAS_DigiCode_Header.h]

#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const kCompileAsCodeResource: usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const kBuildStandAlone:       usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const kNoDSP:                 usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const kNoDAE:                 usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const kNoSDS:                 usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const kNoViews:               usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const kUseDSPCodeDecode:      usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const WIN32:                  usize = 1;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const WINDOWS_VERSION:        usize = 1;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const PLUGIN_SDK_BUILD:       usize = 1;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const PLUGIN_SDK_DIRECTMIDI:  usize = 1;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(_MSC_VER)]      pub const _STDINT_H:              usize = 1;

#[cfg(AloePlugin_Build_RTAS)] #[cfg(not(_MSC_VER))] pub const kCompileAsCodeResource: usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(not(_MSC_VER))] pub const kNoDSP:                 usize = 1;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(not(_MSC_VER))] pub const kNoDAE:                 usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(not(_MSC_VER))] pub const kNoSDS:                 usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(not(_MSC_VER))] pub const kNoViews:               usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(not(_MSC_VER))] pub const kUseDSPCodeDecode:      usize = 0;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(not(_MSC_VER))] pub const MAC_VERSION:            usize = 1;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(not(_MSC_VER))] pub const PLUGIN_SDK_BUILD:       usize = 1;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(not(_MSC_VER))] pub const PLUGIN_SDK_DIRECTMIDI:  usize = 1;
#[cfg(AloePlugin_Build_RTAS)] #[cfg(not(_MSC_VER))] pub const DIGI_PASCAL:             bool = true;
