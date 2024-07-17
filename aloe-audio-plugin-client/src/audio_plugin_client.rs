crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/aloe_audio_plugin_client.h]

/**
  | Config: ALOE_Vst3_CAN_REPLACE_Vst2
  | 
  | Enable this if you want your Vst3 plug-in
  | to load and save Vst2 compatible state.
  | This allows hosts to replace Vst2 plug-ins
  | with Vst3 plug-ins. If you change this
  | option then your Vst3 plug-in will be
  | incompatible with previous versions.
  |
  */
#[cfg(not(ALOE_Vst3_CAN_REPLACE_Vst2))]
pub const ALOE_Vst3_CAN_REPLACE_Vst2: usize = 1;

/**
  | Config: ALOE_FORCE_USE_LEGACY_PARAM_IDS
  | 
  | Enable this if you want to force Aloe
  | to use a continuous parameter index
  | to identify a parameter in a DAW (this
  | was the default in old versions of Aloe).
  | This is index is usually used by the DAW
  | to save automation data and enabling
  | this may mess up user's DAW projects.
  |
  */
#[cfg(not(ALOE_FORCE_USE_LEGACY_PARAM_IDS))]
pub const ALOE_FORCE_USE_LEGACY_PARAM_IDS: usize = 0;

/**
  | Config: ALOE_FORCE_LEGACY_PARAMETER_AUTOMATION_TYPE
  | 
  | Enable this if you want to force Aloe
  | to use a legacy scheme for identifying
  | plug-in parameters as either continuous
  | or discrete.
  | 
  | DAW projects with automation data written
  | by an AudioUnit, Vst3 or
  | 
  | AAX plug-in built with Aloe version
  | 5.1.1 or earlier may load incorrectly
  | when opened by an AudioUnit, Vst3 or
  | AAX plug-in built with Aloe version
  | 5.2.0 and later.
  |
  */
#[cfg(not(ALOE_FORCE_LEGACY_PARAMETER_AUTOMATION_TYPE))]
pub const ALOE_FORCE_LEGACY_PARAMETER_AUTOMATION_TYPE: usize = 0;

/**
  | Config: ALOE_USE_STUDIO_ONE_COMPATIBLE_PARAMETERS
  | 
  | Enable this if you want Aloe to use parameter
  | ids which are compatible with Studio
  | One, as Studio One ignores any parameter
  | ids which are negative.
  | 
  | Enabling this option will make Aloe
  | generate only positive parameter ids.
  | 
  | -----------
  | @note
  | 
  | if you have already released a plug-in
  | prior to Aloe 4.3.0 then enabling this
  | will change your parameter ids, making
  | your plug-in incompatible with old
  | automation data.
  |
  */
#[cfg(not(ALOE_USE_STUDIO_ONE_COMPATIBLE_PARAMETERS))]
pub const ALOE_USE_STUDIO_ONE_COMPATIBLE_PARAMETERS: usize = 1;

/**
  | Config: ALOE_AU_WRAPPERS_SAVE_PROGRAM_STATES
  | 
  | Enable this if you want to receive get/setProgramStateInformation
  | calls, instead of get/setStateInformation
  | calls, from the AU and AUv3 plug-in wrappers.
  | In Aloe version 5.4.5 and earlier this
  | was the default behaviour, so if you
  | have modified the default implementations
  | of get/setProgramStateInformation
  | (where the default implementations
  | simply call through to get/setStateInformation)
  | then you may need to enable this configuration
  | option to maintain backwards compatibility
  | with previously saved state.
  |
  */
#[cfg(not(ALOE_AU_WRAPPERS_SAVE_PROGRAM_STATES))]
pub const ALOE_AU_WRAPPERS_SAVE_PROGRAM_STATES: usize = 0;

/**
  | Config: ALOE_STANDALONE_FILTER_WINDOW_USE_KIOSK_MODE
  | 
  | Enable this if you want your standalone
  | plugin window to use kiosk mode.
  | 
  | By default, kiosk mode is enabled on
  | iOS and Android.
  |
  */
#[cfg(not(ALOE_STANDALONE_FILTER_WINDOW_USE_KIOSK_MODE))]
pub const ALOE_STANDALONE_FILTER_WINDOW_USE_KIOSK_MODE: bool = cfg![target_os="ios"] || cfg![target_os="android"];
