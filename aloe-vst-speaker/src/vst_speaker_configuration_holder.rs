crate::ix!();

/**
  | Class to hold a speaker configuration
  |
  */
#[leak_detector]
pub struct VstSpeakerConfigurationHolder {
    storage: HeapBlock<SpeakerArrangement>,
}

impl Default for VstSpeakerConfigurationHolder {
    
    fn default() -> Self {
        todo!();
        /*


            clear();
        */
    }
}

impl From<&SpeakerArrangement> for VstSpeakerConfigurationHolder {
    
    fn from(vst_config: &SpeakerArrangement) -> Self {
    
        todo!();
        /*


            operator= (vstConfig);
        */
    }
}

impl From<&VstSpeakerConfigurationHolder> for VstSpeakerConfigurationHolder {
    
    fn from(other: &VstSpeakerConfigurationHolder) -> Self {
    
        todo!();
        /*


            operator= (other.get());
        */
    }
}

impl VstSpeakerConfigurationHolder {
    
    pub fn from_move(other: VstSpeakerConfigurationHolder) -> Self {
    
        todo!();
        /*
        : storage(std::move (other.storage)),

            other.clear();
        */
    }
}

impl From<&AudioChannelSet> for VstSpeakerConfigurationHolder {
    
    fn from(channels: &AudioChannelSet) -> Self {
    
        todo!();
        /*


            auto numberOfChannels = channels.size();
                typename Vst2::VstSpeakerArrangement& dst = *allocate (numberOfChannels);

                dst.type = channelSetToVstArrangementType (channels);
                dst.numChannels = numberOfChannels;

                for (int i = 0; i < dst.numChannels; ++i)
                {
                    typename Vst2::VstSpeakerProperties& speaker = dst.speakers[i];

                    zeromem (&speaker, sizeof (typename Vst2::VstSpeakerProperties));
                    speaker.type = getSpeakerType (channels.getTypeOfChannel (i));
                }
        */
    }
}

impl VstSpeakerConfigurationHolder {
    
    pub fn assign_from_config(&mut self, vst_config: &VstSpeakerConfigurationHolder) -> &mut VstSpeakerConfigurationHolder {
        
        todo!();
        /*
            return operator=(vstConfig.get());
        */
    }
    
    pub fn assign_from_arrangement(&mut self, vst_config: &SpeakerArrangement) -> &mut VstSpeakerConfigurationHolder {
        
        todo!();
        /*
            typename Vst2::VstSpeakerArrangement& dst = *allocate (vstConfig.numChannels);

                dst.type             = vstConfig.type;
                dst.numChannels      = vstConfig.numChannels;

                for (int i = 0; i < dst.numChannels; ++i)
                    dst.speakers[i] = vstConfig.speakers[i];

                return *this;
        */
    }
    
    pub fn assign_from(&mut self, vst_config: VstSpeakerConfigurationHolder) -> &mut VstSpeakerConfigurationHolder {
        
        todo!();
        /*
            storage = std::move (vstConfig.storage);
                vstConfig.clear();

                return *this;
        */
    }
    
    pub fn get(&self) -> &SpeakerArrangement {
        
        todo!();
        /*
            return *storage.get();
        */
    }
    
    pub fn allocate(&mut self, num_channels: i32) -> *mut SpeakerArrangement {
        
        todo!();
        /*
            auto arrangementSize = (size_t) (jmax (8, numChannels) - 8) * sizeof (typename Vst2::VstSpeakerProperties)
                                        + sizeof (typename Vst2::VstSpeakerArrangement);

                storage.malloc (1, arrangementSize);
                return storage.get();
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            typename Vst2::VstSpeakerArrangement& dst = *allocate (0);

                dst.type = typename Vst2::kSpeakerArrEmpty;
                dst.numChannels = 0;
        */
    }
}
