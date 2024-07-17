crate::ix!();

/**
  | Maps controller numbers to ParamIDs
  |
  */
pub type StoredMidiMappingControllers = Vec<ParamID>;

/**
  | Each channel may have a different CC
  | mapping
  |
  */
pub type StoredMidiMappingChannels = [StoredMidiMappingControllers; 16];

/**
  | This class stores a plugin's preferred
  | MIDI mappings.
  | 
  | The IMidiMapping is normally an extension
  | of the IEditController which should
  | only be accessed from the UI thread.
  | If we're being strict about things,
  | then we shouldn't call IMidiMapping
  | functions from the audio thread.
  | 
  | This code is very similar to that found
  | in the audioclient demo code in the
  | 
  | Vst3 SDK repo.
  |
  */
pub struct StoredMidiMapping {
    channels: StoredMidiMappingChannels,
}

impl Default for StoredMidiMapping {
    
    fn default() -> Self {
        todo!();
        /*


            for (auto& channel : channels)
                channel.resize (typename VstkCountCtrlNumber);
        */
    }
}

impl StoredMidiMapping {

    pub fn store_mappings(&mut self, mapping: &mut dyn IMidiMapping)  {
        
        todo!();
        /*
            for (size_t channelIndex = 0; channelIndex < channels.size(); ++channelIndex)
                storeControllers (mapping, channels[channelIndex], channelIndex);
        */
    }

    /**
      | Returns kNoParamId if there is no mapping
      | for this controller.
      |
      */
    pub fn get_mapping(
        &self, 
        channel:    i16,
        controller: CtrlNumber

    ) -> ParamID {
        
        todo!();
        /*
            return channels[(size_t) channel][(size_t) controller];
        */
    }
    
    pub fn store_controllers(
        mapping:       &mut dyn IMidiMapping,
        channel:       &mut StoredMidiMappingControllers,
        channel_index: usize)  {
        
        todo!();
        /*
            for (size_t controllerIndex = 0; controllerIndex < channel.size(); ++controllerIndex)
                channel[controllerIndex] = getSingleMapping (mapping, channelIndex, controllerIndex);
        */
    }
    
    pub fn get_single_mapping(
        mapping:          &mut dyn IMidiMapping,
        channel_index:    usize,
        controller_index: usize

    ) -> ParamID {
        
        todo!();
        /*
            typename VstParamID result{};
            const auto returnCode = mapping.getMidiControllerAssignment (0,
                                                                         (i16) channelIndex,
                                                                         (typename VstCtrlNumber) controllerIndex,
                                                                         result);

            return returnCode == typename kResultTrue ? result : typename VstkNoParamId;
        */
    }
}
