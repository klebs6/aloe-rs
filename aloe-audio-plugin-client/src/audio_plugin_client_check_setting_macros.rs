/*!
  | The following checks should cause a
  | compile error if you've forgotten to
  | define all your plugin settings properly..
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/utility/aloe_CheckSettingMacros.h]

/**
  | "You need to enable at least one plugin
  | format!"
  |
  */
#[cfg(not(any(
    ALOE_PLUGIN_BUILD_Vst,
    ALOE_PLUGIN_BUILD_Vst3,
    ALOE_PLUGIN_BUILD_AU,
    ALOE_PLUGIN_BUILD_AUV3,
    ALOE_PLUGIN_BUILD_RTAS,
    ALOE_PLUGIN_BUILD_AAX,
    ALOE_PLUGIN_BUILD_STANDALONE,
    ALOE_PLUGIN_BUILD_LV2,
    ALOE_PLUGIN_BUILD_UNITY,
)))]
compile_error!("At least one plugin type must be specified.");

/**
  | "This header should never be included
  | twice!
  | 
  | Otherwise something is wrong."
  |
  */
static_assert!{ !cfg![ALOE_CHECKSETTINGMACROS_H] }

pub const ALOE_CHECKSETTINGMACROS_H: bool = true;

/**
  | "You need to define the AloePlugin_IsSynth
  | value!"
  |
  */
static_assert!{ cfg![ALOE_PLUGIN_IS_SYNTH] }

/**
  | "You need to define the
  | 
  | AloePlugin_ManufacturerCode value!"
  |
  */
static_assert!{ cfg![ALOE_PLUGIN_MANUFACTURER_CODE] }

/**
  | "You need to define the AloePlugin_PluginCode
  | value!"
  |
  */
static_assert!{ cfg![ALOE_PLUGIN_PLUGIN_CODE] }

/**
  | "You need to define the
  | 
  | AloePlugin_ProducesMidiOutput value!"
  |
  */
static_assert!{ cfg![ALOE_PLUGIN_PRODUCES_MIDI_OUTPUT] }

/**
  | "You need to define the
  | AloePlugin_WantsMidiInput value!"
  |
  */
static_assert!{ cfg![ALOE_PLUGIN_WANTS_MIDI_INPUT] }

/**
  | "AloePlugin_Latency is now deprecated
  | - instead, call the AudioProcessor::setLatencySamples()
  | method if your plugin has a non-zero
  | delay"
  |
  */
static_assert!{ cfg![ALOE_PLUGIN_LATENCY] }

/**
  | "You need to define the
  | AloePlugin_EditorRequiresKeyboardFocus
  | value!"
  |
  */
static_assert!{ cfg![ALOE_PLUGIN_EDITOR_REQUIRES_KEYBOARD_FOCUS] }

#[cfg(any(_WIN64,all(__LP64__,any(__APPLE_CPP__,__APPLE_CC__))))]
pub const ALOE_PLUGIN_BUILD_RTAS: usize = 0;

#[cfg(not(any(any(_MSC_VER,__APPLE_CPP__),any(__APPLE_CC__,LINUX,__linux__))))]
pub const ALOE_PLUGIN_BUILD_Vst3: usize = 0;

/**
  | "You need to define the AloePlugin_LV2URI
  | value!"
  |
  */
static_assert!{ !(cfg![ALOE_PLUGIN_BUILD_LV2] && !cfg![ALOE_PLUGIN_LV2URI] ) }

/**
  | "You need to define the AloePlugin_AAXIdentifier
  | value!"
  |
  */
static_assert!{ !(cfg![ALOE_PLUGIN_BUILD_AAX] && !cfg![ALOE_PLUGIN_AAX_IDENTIFIER]) }

#[cfg(__ppc__)]
pub const ALOE_PLUGIN_BUILD_AAX: usize = 0;
