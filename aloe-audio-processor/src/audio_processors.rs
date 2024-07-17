crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/aloe_audio_processors.h]

/**
  | Config: ALOE_PLUGINHOST_Vst
  | 
  | Enables the Vst audio plugin hosting
  | classes. You will need to have the Vst2
  | SDK files in your header search paths.
  | You can obtain the Vst2 SDK files from
  | on older version of the Vst3 SDK.
  | 
  | @see VstPluginFormat, Vst3PluginFormat,
  | AudioPluginFormat, AudioPluginFormatManager,
  | ALOE_PLUGINHOST_AU, ALOE_PLUGINHOST_Vst3,
  | ALOE_PLUGINHOST_LADSPA
  |
  */
#[cfg(not(ALOE_PLUGINHOST_Vst))]
pub const ALOE_PLUGINHOST_Vst: usize = 0;

/**
  | Config: ALOE_PLUGINHOST_Vst3
  | 
  | Enables the Vst3 audio plugin hosting
  | classes.
  | 
  | @see VstPluginFormat, Vst3PluginFormat,
  | AudioPluginFormat, AudioPluginFormatManager,
  | ALOE_PLUGINHOST_Vst, ALOE_PLUGINHOST_AU,
  | ALOE_PLUGINHOST_LADSPA
  |
  */
#[cfg(any(
        all(all(not(ALOE_WINDOWS),not(target_os="macos")),not(all(ALOE_LINUX,not(ALOE_BSD)))),
        not(ALOE_PLUGINHOST_Vst3)
))]
pub const ALOE_PLUGINHOST_Vst3: usize = 0;

/**
  | Config: ALOE_PLUGINHOST_AU Enables
  | the AudioUnit plugin hosting classes.
  | This is Mac-only, of course.
  | 
  | @see AudioUnitPluginFormat, AudioPluginFormat,
  | AudioPluginFormatManager, ALOE_PLUGINHOST_Vst,
  | ALOE_PLUGINHOST_Vst3, ALOE_PLUGINHOST_LADSPA
  |
  */
#[cfg(not(ALOE_PLUGINHOST_AU))]
pub const ALOE_PLUGINHOST_AU: usize = 0;

/**
  | Config: ALOE_PLUGINHOST_LADSPA
  | 
  | Enables the LADSPA plugin hosting classes.
  | This is Linux-only, of course.
  | 
  | @see LADSPAPluginFormat, AudioPluginFormat,
  | AudioPluginFormatManager, ALOE_PLUGINHOST_Vst,
  | ALOE_PLUGINHOST_Vst3, ALOE_PLUGINHOST_AU
  |
  */
#[cfg(not(ALOE_PLUGINHOST_LADSPA))]
pub const ALOE_PLUGINHOST_LADSPA: usize = 0;

/**
  | Config: ALOE_CUSTOM_Vst3_SDK
  | 
  | If enabled, the embedded Vst3 SDK in
  | Aloe will not be added to the project
  | and instead you should add the path to
  | your custom Vst3 SDK to the project's
  | header search paths. Most users shouldn't
  | need to enable this and should just use
  | the version of the SDK included with
  | Aloe.
  |
  */
#[cfg(not(ALOE_CUSTOM_Vst3_SDK))]
pub const ALOE_CUSTOM_Vst3_SDK: usize = 0;

#[cfg(not(any(any(ALOE_SUPPORT_CARBON,ALOE_64BIT),target_os="ios")))]
pub const ALOE_SUPPORT_CARBON: usize = 1;

#[cfg(not(ALOE_SUPPORT_LEGACY_AUDIOPROCESSOR))]
pub const ALOE_SUPPORT_LEGACY_AUDIOPROCESSOR: usize = 1;

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/aloe_audio_processors.cpp]

pub const ALOE_CORE_INCLUDE_NATIVE_HEADERS:                           usize = 1;
pub const ALOE_CORE_INCLUDE_OBJC_HELPERS:                             usize = 1;
pub const ALOE_GUI_BASICS_INCLUDE_XHEADERS:                           usize = 1;
pub const ALOE_GUI_BASICS_INCLUDE_SCOPED_THREAD_DPI_AWARENESS_SETTER: usize = 1;
pub const ALOE_GRAPHICS_INCLUDE_COREGRAPHICS_HELPERS:                 usize = 1;


#[cfg(any(ALOE_PLUGINHOST_Vst,all(ALOE_PLUGINHOST_LADSPA,any(target_os="linux",target_os="bsd"))))]
pub fn array_contains_plugin(
        list: &Vec<Box<PluginDescription>>,
        desc: &PluginDescription) -> bool {
    
    todo!();
        /*
            for (auto* p : list)
            if (p->isDuplicateOf (desc))
                return true;

        return false;
        */
}
