crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/utility/aloe_CreatePluginFilter.h]

/**
  | Somewhere in the codebase of your plugin,
  | you need to implement this function
  | and make it return a new instance of the
  | filter subclass that you're building.
  |
  */
#[ALOE_CALLTYPE]
pub fn create_plugin_filter<'a>() -> *mut AudioProcessor<'a> {
    
    todo!();
        /*
        
        */
}

#[ALOE_CALLTYPE]
#[inline] pub fn create_plugin_filter_of_type<'a>(ty: AudioProcessorWrapperType) -> *mut AudioProcessor<'a> {
    
    todo!();
        /*
            AudioProcessor::setTypeOfNextNewPlugin (type);
        AudioProcessor* const pluginInstance = ::createPluginFilter();
        AudioProcessor::setTypeOfNextNewPlugin (AudioProcessor::wrapperType_Undefined);

        // your createPluginFilter() method must return an object!
        jassert (pluginInstance != nullptr && pluginInstance->wrapperType == type);

        return pluginInstance;
        */
}
