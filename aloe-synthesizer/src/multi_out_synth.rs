crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/MultiOutSynthPluginDemo.h]

#[no_copy]
#[leak_detector]
pub struct MultiOutSynth<'a> {
    base:           &'a mut dyn AudioProcessorInterface,
    format_manager: &'a mut dyn AudioFormatManagerInterface,
    synth:          Vec<Box<Synthesizer>>,
    sound:          SynthesizerSoundPtr,
}

pub const MULTI_OUT_SYNTH_MAX_MIDI_CHANNEL:    usize = 16;
pub const MULTI_OUT_SYNTH_MAX_NUMBER_OF_VOICES: usize = 5;

impl<'a> Default for MultiOutSynth<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : AudioProcessor (BusesProperties()
                              .withOutput ("Output #1",  AudioChannelSet::stereo(), true)
                              .withOutput ("Output #2",  AudioChannelSet::stereo(), false)
                              .withOutput ("Output #3",  AudioChannelSet::stereo(), false)
                              .withOutput ("Output #4",  AudioChannelSet::stereo(), false)
                              .withOutput ("Output #5",  AudioChannelSet::stereo(), false)
                              .withOutput ("Output #6",  AudioChannelSet::stereo(), false)
                              .withOutput ("Output #7",  AudioChannelSet::stereo(), false)
                              .withOutput ("Output #8",  AudioChannelSet::stereo(), false)
                              .withOutput ("Output #9",  AudioChannelSet::stereo(), false)
                              .withOutput ("Output #10", AudioChannelSet::stereo(), false)
                              .withOutput ("Output #11", AudioChannelSet::stereo(), false)
                              .withOutput ("Output #12", AudioChannelSet::stereo(), false)
                              .withOutput ("Output #13", AudioChannelSet::stereo(), false)
                              .withOutput ("Output #14", AudioChannelSet::stereo(), false)
                              .withOutput ("Output #15", AudioChannelSet::stereo(), false)
                              .withOutput ("Output #16", AudioChannelSet::stereo(), false))

            // initialize other stuff (not related to buses)
            formatManager.registerBasicFormats();

            for (int midiChannel = 0; midiChannel < maxMidiChannel; ++midiChannel)
            {
                synth.add (new Synthesizer());

                for (int i = 0; i < maxNumberOfVoices; ++i)
                    synth[midiChannel]->addVoice (new SamplerVoice());
            }

            loadNewSample (createAssetInputStream ("singing.ogg"), "ogg")
        */
    }
}

impl<'a> MultiOutSynth<'a> {

    pub fn can_add_bus(&self, is_input: bool) -> bool {
        
        todo!();
        /*
            return ! isInput;
        */
    }
    
    pub fn can_remove_bus(&self, is_input: bool) -> bool {
        
        todo!();
        /*
            return ! isInput;
        */
    }
    
    pub fn prepare_to_play(&mut self, 
        new_sample_rate:   f64,
        samples_per_block: i32)  {
        
        todo!();
        /*
            ignoreUnused (samplesPerBlock);

            for (auto* s : synth)
                s->setCurrentPlaybackSampleRate (newSampleRate);
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn process_block(&mut self, 
        buffer:      &mut AudioBuffer<f32>,
        midi_buffer: &mut MidiBuffer)  {
        
        todo!();
        /*
            auto busCount = getBusCount (false);

            for (auto busNr = 0; busNr < busCount; ++busNr)
            {
                if (synth.size() <= busNr)
                    continue;

                auto midiChannelBuffer = filterMidiMessagesForChannel (midiBuffer, busNr + 1);
                auto audioBusBuffer = getBusBuffer (buffer, false, busNr);

                synth [busNr]->renderNextBlock (audioBusBuffer, midiChannelBuffer, 0, audioBusBuffer.getNumSamples());
            }
        */
    }
    
    pub fn create_editor(&mut self) -> Box<dyn AudioProcessorEditorInterface> {
        
        todo!();
        /*
            return new GenericAudioProcessorEditor (*this);
        */
    }
    
    pub fn has_editor(&self)              -> bool         { true }
    pub fn get_name(&self)                -> &'static str { "Multi Out Synth PlugIn" }
    pub fn accepts_midi(&self)            -> bool         { false }
    pub fn produces_midi(&self)           -> bool         { false }
    pub fn get_tail_length_seconds(&self) -> f64          { 0.0 }
    pub fn get_num_programs(&mut self)    -> i32          { 1 }
    pub fn get_current_program(&mut self) -> i32          { 0 }
    
    pub fn set_current_program(&mut self, _0: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_program_name(&mut self, _0: i32) -> String {
        
        todo!();
        /*
            return "None";
        */
    }
    
    pub fn change_program_name(&mut self, 
        _0: i32,
        _1: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn is_buses_layout_supported(&self, layout: &dyn AudioProcessorBusesLayoutInterface) -> bool {
        
        todo!();
        /*
            for (const auto& bus : layout.outputBuses)
                if (bus != AudioChannelSet::stereo())
                    return false;

            return layout.inputBuses.isEmpty() && 1 <= layout.outputBuses.size();
        */
    }
    
    pub fn get_state_information(&mut self, _0: &mut MemoryBlock)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn set_state_information(&mut self, 
        _0: *const c_void,
        _1: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn filter_midi_messages_for_channel(
        input:   &MidiBuffer,
        channel: i32) -> MidiBuffer {
        
        todo!();
        /*
            MidiBuffer output;

            for (const auto metadata : input)
            {
                const auto message = metadata.getMessage();

                if (message.getChannel() == channel)
                    output.addEvent (message, metadata.samplePosition);
            }

            return output;
        */
    }
    
    pub fn load_new_sample<R: Read>(
        &mut self, 
        sound_buffer: &R,
        format:       *const u8

    ) {

        todo!();
        /*
            std::unique_ptr<AudioFormatReader> formatReader (formatManager.findFormatForFileExtension (format)->createReaderFor (soundBuffer.release(), true));

            BigInteger midiNotes;
            midiNotes.setRange (0, 126, true);
            SynthesizerSound::Ptr newSound = new SamplerSound ("Voice", *formatReader, midiNotes, 0x40, 0.0, 0.0, 10.0);

            for (auto* s : synth)
                s->removeSound (0);

            sound = newSound;

            for (auto* s : synth)
                s->addSound (sound);
        */
    }
}
