crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_AU_Shared.h]

/**
  | This macro can be set if you need to override
  | this internal name for some reason..
  |
  */
#[cfg(not(ALOE_STATE_DICTIONARY_KEY))]
pub const ALOE_STATE_DICTIONARY_KEY: &'static str = "aloePluginState";
