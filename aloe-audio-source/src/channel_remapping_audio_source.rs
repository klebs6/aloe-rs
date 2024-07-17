crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_ChannelRemappingAudioSource.h]

/**
  | An AudioSource that takes the audio
  | from another source, and re-maps its
  | input and output channels to a different
  | arrangement.
  | 
  | You can use this to increase or decrease
  | the number of channels that an audio
  | source uses, or to re-order those channels.
  | 
  | Call the reset() method before using
  | it to set up a default mapping, and then
  | the setInputChannelMapping() and
  | setOutputChannelMapping() methods
  | to create an appropriate mapping, otherwise
  | no channels will be connected and it'll
  | produce silence.
  | 
  | @see AudioSource
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ChannelRemappingAudioSource {
    source:                      OptionalScopedPointer<dyn AudioSource>,
    remapped_inputs:             Vec<i32>,
    remapped_outputs:            Vec<i32>,
    required_number_of_channels: i32,
    buffer:                      AudioBuffer<f32>,
    remapped_info:               AudioSourceChannelInfo,
    lock:                        CriticalSection,
}

impl AudioSource for ChannelRemappingAudioSource {}

impl PrepareToPlayAudioSource for ChannelRemappingAudioSource {

    fn prepare_to_play(&mut self, 
        samples_per_block_expected: i32,
        sample_rate:                f64)  {
        
        todo!();
        /*
            source->prepareToPlay (samplesPerBlockExpected, sampleRate);
        */
    }
}
    
impl ReleaseResources for ChannelRemappingAudioSource {

    fn release_resources(&mut self)  {
        
        todo!();
        /*
            source->releaseResources();
        */
    }
}
    
impl GetNextAudioBlock for ChannelRemappingAudioSource {

    fn get_next_audio_block(&mut self, buffer_to_fill: &AudioSourceChannelInfo)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        buffer.setSize (requiredNumberOfChannels, bufferToFill.numSamples, false, false, true);

        const int numChans = bufferToFill.buffer->getNumChannels();

        for (int i = 0; i < buffer.getNumChannels(); ++i)
        {
            const int remappedChan = getRemappedInputChannel (i);

            if (remappedChan >= 0 && remappedChan < numChans)
            {
                buffer.copyFrom (i, 0, *bufferToFill.buffer,
                                 remappedChan,
                                 bufferToFill.startSample,
                                 bufferToFill.numSamples);
            }
            else
            {
                buffer.clear (i, 0, bufferToFill.numSamples);
            }
        }

        remappedInfo.numSamples = bufferToFill.numSamples;

        source->getNextAudioBlock (remappedInfo);

        bufferToFill.clearActiveBufferRegion();

        for (int i = 0; i < requiredNumberOfChannels; ++i)
        {
            const int remappedChan = getRemappedOutputChannel (i);

            if (remappedChan >= 0 && remappedChan < numChans)
            {
                bufferToFill.buffer->addFrom (remappedChan, bufferToFill.startSample,
                                              buffer, i, 0, bufferToFill.numSamples);

            }
        }
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/sources/aloe_ChannelRemappingAudioSource.cpp]
impl ChannelRemappingAudioSource {

    /**
      | Creates a remapping source that will
      | pass on audio from the given input.
      | 
      | -----------
      | @param source
      | 
      | the input source to use. Make sure that
      | this doesn't get deleted before the
      | ChannelRemappingAudioSource object
      | ----------
      | @param deleteSourceWhenDeleted
      | 
      | if true, the input source will be deleted
      | when this object is deleted, if false,
      | the caller is responsible for its deletion
      |
      */
    pub fn new(
        source:                     *mut dyn AudioSource,
        delete_source_when_deleted: bool) -> Self {
    
        todo!();
        /*
        : source(source_, deleteSourceWhenDeleted),
        : required_number_of_channels(2),

            remappedInfo.buffer = &buffer;
        remappedInfo.startSample = 0;
        */
    }
    
    /**
      | Specifies a number of channels that
      | this audio source must produce from
      | its getNextAudioBlock() callback.
      |
      */
    pub fn set_number_of_channels_to_produce(&mut self, required_number_of_channels: i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        requiredNumberOfChannels = requiredNumberOfChannels_;
        */
    }
    
    /**
      | Clears any mapped channels.
      | 
      | After this, no channels are mapped,
      | so this object will produce silence.
      | Create some mappings with setInputChannelMapping()
      | and setOutputChannelMapping().
      |
      */
    pub fn clear_all_mappings(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        remappedInputs.clear();
        remappedOutputs.clear();
        */
    }
    
    /**
      | Creates an input channel mapping.
      | 
      | When the getNextAudioBlock() method
      | is called, the data in channel sourceChannelIndex
      | of the incoming data will be sent to destChannelIndex
      | of our input source.
      | 
      | -----------
      | @param destChannelIndex
      | 
      | the index of an input channel in our input
      | audio source (i.e. the source specified
      | when this object was created).
      | ----------
      | @param sourceChannelIndex
      | 
      | the index of the input channel in the
      | incoming audio data buffer during our
      | getNextAudioBlock() callback
      |
      */
    pub fn set_input_channel_mapping(&mut self, 
        dest_index:   i32,
        source_index: i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        while (remappedInputs.size() < destIndex)
            remappedInputs.add (-1);

        remappedInputs.set (destIndex, sourceIndex);
        */
    }
    
    /**
      | Creates an output channel mapping.
      | 
      | When the getNextAudioBlock() method
      | is called, the data returned in channel
      | sourceChannelIndex by our input audio
      | source will be copied to channel destChannelIndex
      | of the final buffer.
      | 
      | -----------
      | @param sourceChannelIndex
      | 
      | the index of an output channel coming
      | from our input audio source (i.e. the
      | source specified when this object was
      | created).
      | ----------
      | @param destChannelIndex
      | 
      | the index of the output channel in the
      | incoming audio data buffer during our
      | getNextAudioBlock() callback
      |
      */
    pub fn set_output_channel_mapping(&mut self, 
        source_index: i32,
        dest_index:   i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        while (remappedOutputs.size() < sourceIndex)
            remappedOutputs.add (-1);

        remappedOutputs.set (sourceIndex, destIndex);
        */
    }
    
    /**
      | Returns the channel from our input that
      | will be sent to channel inputChannelIndex
      | of our input audio source.
      |
      */
    pub fn get_remapped_input_channel(&self, input_channel_index: i32) -> i32 {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        if (inputChannelIndex >= 0 && inputChannelIndex < remappedInputs.size())
            return remappedInputs.getUnchecked (inputChannelIndex);

        return -1;
        */
    }
    
    /**
      | Returns the output channel to which
      | channel outputChannelIndex of our
      | input audio source will be sent to.
      |
      */
    pub fn get_remapped_output_channel(&self, output_channel_index: i32) -> i32 {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        if (outputChannelIndex >= 0 && outputChannelIndex < remappedOutputs.size())
            return remappedOutputs .getUnchecked (outputChannelIndex);

        return -1;
        */
    }
    
    /**
      | Returns an XML object to encapsulate
      | the state of the mappings. @see restoreFromXml
      |
      */
    pub fn create_xml(&self) -> Box<XmlElement> {
        
        todo!();
        /*
            auto e = std::make_unique<XmlElement> ("MAPPINGS");
        String ins, outs;

        const ScopedLock sl (lock);

        for (int i = 0; i < remappedInputs.size(); ++i)
            ins << remappedInputs.getUnchecked(i) << ' ';

        for (int i = 0; i < remappedOutputs.size(); ++i)
            outs << remappedOutputs.getUnchecked(i) << ' ';

        e->setAttribute ("inputs", ins.trimEnd());
        e->setAttribute ("outputs", outs.trimEnd());

        return e;
        */
    }
    
    /**
      | Restores the mappings from an XML object
      | created by createXML(). @see createXml
      |
      */
    pub fn restore_from_xml(&mut self, e: &XmlElement)  {
        
        todo!();
        /*
            if (e.hasTagName ("MAPPINGS"))
        {
            const ScopedLock sl (lock);

            clearAllMappings();

            StringArray ins, outs;
            ins.addTokens (e.getStringAttribute ("inputs"), false);
            outs.addTokens (e.getStringAttribute ("outputs"), false);

            for (int i = 0; i < ins.size(); ++i)
                remappedInputs.add (ins[i].getIntValue());

            for (int i = 0; i < outs.size(); ++i)
                remappedOutputs.add (outs[i].getIntValue());
        }
        */
    }
}
