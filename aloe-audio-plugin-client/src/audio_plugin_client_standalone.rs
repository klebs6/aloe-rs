crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/aloe_audio_plugin_client_Standalone.cpp]

/**
  #error To compile AudioUnitv3 and/or Standalone
  plug-ins, you need to add the aloe_audio_utils
  and aloe_audio_devices modules!
  */
#[cfg(feature = "plugin-build-standalone")]
static_assert!{ALOE_MODULE_AVAILABLE_aloe_audio_utils}

#[cfg(feature = "plugin-build-standalone")]
#[cfg(ALOE_USE_CUSTOM_PLUGIN_STANDALONE_APP)]
lazy_static!{
    /*
       extern aloe::ALOEApplicationBase* aloe_CreateApplication();
       */
}

#[cfg(feature = "plugin-build-standalone")]
#[cfg(ALOE_USE_CUSTOM_PLUGIN_STANDALONE_APP)]
#[cfg(target_os="ios")]
lazy_static!{
    /*
       extern void* aloe_GetIOSCustomDelegateClass();
       */
}

#[cfg(feature = "plugin-build-standalone")]
#[cfg(not(ALOE_USE_CUSTOM_PLUGIN_STANDALONE_APP))]
aloe_create_application_define!{ aloe::StandaloneFilterApp }

#[cfg(feature = "plugin-build-standalone")]
ALOE_MAIN_FUNCTION_DEFINITION!{}
