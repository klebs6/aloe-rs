crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_AudioUnitPluginFormat.h]

/**
  | Implements a plugin format manager
  | for AudioUnits.
  | 
  | @tags{Audio}
  |
  */
#[cfg(all(ALOE_PLUGINHOST_AU,any(target_os="macos",target_os="ios")))]
#[no_copy]
#[leak_detector]
pub struct AudioUnitPluginFormat {
    base: AudioPluginFormat,
}

#[cfg(all(ALOE_PLUGINHOST_AU,any(target_os="macos",target_os="ios")))]
impl AudioUnitPluginFormat {

    pub fn get_format_name() -> String {
        
        todo!();
        /*
            return "AudioUnit";
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return getFormatName();
        */
    }
    
    pub fn can_scan_for_plugins(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn is_trivial_to_scan(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn find_all_types_for_file(&mut self, 
        _0:                 &mut Vec<Box<PluginDescription>>,
        file_or_identifier: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn file_might_contain_this_plugin_type(&mut self, file_or_identifier: &String) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_name_of_plugin_from_identifier(&mut self, file_or_identifier: &String) -> String {
        
        todo!();
        /*
        
        */
    }
    
    pub fn plugin_needs_rescanning(&mut self, _0: &PluginDescription) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn search_paths_for_plugins(
        &mut self, 
        _0:        &FileSearchPath,
        recursive: bool,
        _2:        bool

    ) -> Vec<String> {
        
        todo!();
        /*
        
        */
    }
    
    pub fn does_plugin_still_exist(&mut self, _0: &PluginDescription) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_default_locations_to_search(&mut self) -> FileSearchPath {
        
        todo!();
        /*
        
        */
    }
    
    pub fn create_plugin_instance(&mut self, 
        _0:                  &PluginDescription,
        initial_sample_rate: f64,
        initial_buffer_size: i32,
        _3:                  PluginCreationCallback)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn requires_unblocked_message_thread_during_creation(&self, _0: &PluginDescription) -> bool {
        
        todo!();
        /*
        
        */
    }
}

/**
  | Custom AudioUnit property used to indicate
  | MPE support
  |
  */
#[cfg(any(not(MAC_OS_X_VERSION_10_12),MAC_OS_X_VERSION_MAX_ALLOWED_LT_MAC_OS_X_VERSION_10_12))]
pub const kAudioUnitProperty_SupportsMPE: usize = 58;
