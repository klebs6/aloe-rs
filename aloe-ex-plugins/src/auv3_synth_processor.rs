crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AUv3SynthProcessor<'a> {
    base:               AudioProcessor<'a>,
    format_manager:     AudioFormatManager,
    samples_recorded:   i32,
    last_sample_rate:   f64,
    current_recording:  AudioBuffer<f32>,
    reverb:             Reverb,
    synth:              Synthesizer,
    sound:              SynthesizerSoundPtr,
    is_recording_param: *mut AudioParameterBool,
    room_size_param:    *mut AudioParameterFloat,
    current_program:    i32,
}

pub const AUV3_SYNTH_PROCESSOR_MAX_NUM_VOICES:            usize = 5;
pub const AUV3_SYNTH_PROCESSOR_MAX_DURATION_OF_RECORDING:   f64 = 1.0;

impl<'a> Default for AUv3SynthProcessor<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : AudioProcessor (BusesProperties().withOutput ("Output", AudioChannelSet::stereo(), true)),
              currentRecording (1, 1), currentProgram (0)

            // initialize parameters
            addParameter (isRecordingParam = new AudioParameterBool  ("isRecording", "Is Recording", false));
            addParameter (roomSizeParam    = new AudioParameterFloat ("roomSize", "Room Size", 0.0f, 1.0f, 0.5f));

            formatManager.registerBasicFormats();

            for (auto i = 0; i < maxNumVoices; ++i)
                synth.addVoice (new SamplerVoice());

            loadNewSample (createAssetInputStream ("singing.ogg"), "ogg")
        */
    }
}

pub trait PrepareToPlay {

    fn prepare_to_play(
        &mut self, 
        sample_rate:                  f64,
        estimated_max_size_of_buffer: i32
    );
}

impl<'a> PrepareToPlay for AUv3SynthProcessor<'a> {

    fn prepare_to_play(&mut self, 
        sample_rate:                  f64,
        estimated_max_size_of_buffer: i32)  {
        
        todo!();
        /*
            ignoreUnused (estimatedMaxSizeOfBuffer);

            lastSampleRate = sampleRate;

            currentRecording.setSize (1, static_cast<int> (std::ceil (maxDurationOfRecording * lastSampleRate)));
            samplesRecorded = 0;

            synth.setCurrentPlaybackSampleRate (lastSampleRate);
            reverb.setSampleRate (lastSampleRate);
        */
    }
}

pub trait ProcessBlock {

    fn process_block(
        &mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut MidiBuffer
    );
}

impl<'a> ProcessBlock for AUv3SynthProcessor<'a> {

    fn process_block(&mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut MidiBuffer)  {
        
        todo!();
        /*
            Reverb::Parameters reverbParameters;
            reverbParameters.roomSize = roomSizeParam->get();

            reverb.setParameters (reverbParameters);
            synth.renderNextBlock (buffer, midiMessages, 0, buffer.getNumSamples());

            if (getMainBusNumOutputChannels() == 1)
                reverb.processMono (buffer.getWritePointer (0), buffer.getNumSamples());
            else if (getMainBusNumOutputChannels() == 2)
                reverb.processStereo (buffer.getWritePointer (0), buffer.getWritePointer (1), buffer.getNumSamples());
        */
    }
}

impl<'a> ReleaseResources for AUv3SynthProcessor<'a> {
    fn release_resources(&mut self)  {
        
        todo!();
        /*
            currentRecording.setSize (1, 1);
        */
    }
}

impl<'a> AcceptsMidi for AUv3SynthProcessor<'a> {
    fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

impl<'a> ProducesMidi for AUv3SynthProcessor<'a> {
    fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

pub trait CreateEditor {

    fn create_editor(&mut self) -> *mut AudioProcessorEditor;
}

impl<'a> CreateEditor for AUv3SynthProcessor<'a> {

    fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return new AUv3SynthEditor (*this);
        */
    }
}

impl<'a> HasEditor for AUv3SynthProcessor<'a> {
    fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

impl<'a> GetName for AUv3SynthProcessor<'a> {
    fn get_name(&self) -> String {
        
        todo!();
        /*
            return "AUv3 Synth";
        */
    }
}

pub trait GetNumPrograms {

    fn get_num_programs(&mut self) -> i32;
}

impl<'a> GetNumPrograms for AUv3SynthProcessor<'a> {

    fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
            return 4;
        */
    }
}

pub trait GetCurrentProgram {

    fn get_current_program(&mut self) -> i32;
}

impl<'a> GetCurrentProgram for AUv3SynthProcessor<'a> {
    fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
            return currentProgram;
        */
    }
}

pub trait GetTailLengthSeconds {

    fn get_tail_length_seconds(&self) -> f64;
}

impl<'a> GetTailLengthSeconds for AUv3SynthProcessor<'a> {
    fn get_tail_length_seconds(&self) -> f64 {
        
        todo!();
        /*
            return 0.0;
        */
    }
}

pub trait GetProgramName {

    fn get_program_name(&mut self, index: i32) -> String;
}

impl<'a> GetProgramName for AUv3SynthProcessor<'a> {

    fn get_program_name(&mut self, index: i32) -> String {
        
        todo!();
        /*
            switch (index)
            {
                case 0:  return "Piano";
                case 1:  return "Singing";
                case 2:  return "Pinched Balloon";
                case 3:  return "Gazeebo";
                default: break;
            }

            return "<Unknown>";
        */
    }
}

pub trait SetCurrentProgram {

    fn set_current_program(&mut self, index: i32);
}

impl<'a> SetCurrentProgram for AUv3SynthProcessor<'a> {

    fn set_current_program(&mut self, index: i32)  {
        
        todo!();
        /*
            currentProgram = index;
        */
    }
}

pub trait ChangeProgramName {

    fn change_program_name(
        &mut self, 
        index: i32,
        name:  &str
    );
}

impl<'a> ChangeProgramName for AUv3SynthProcessor<'a> {

    fn change_program_name(
        &mut self, 
        index: i32,
        name:  &str
    ) {
        
        todo!();
        /*
        
        */
    }
}

pub trait GetStateInformation {

    fn get_state_information(&mut self, dest_data: &mut MemoryBlock);
}

impl<'a> GetStateInformation for AUv3SynthProcessor<'a> {
    fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            MemoryOutputStream stream (destData, true);

            stream.writeFloat (*isRecordingParam);
            stream.writeFloat (*roomSizeParam);
        */
    }
}

pub trait SetStateInformation {

    fn set_state_information(
        &mut self, 
        data:          *const c_void,
        size_in_bytes: i32
    );
}

impl<'a> SetStateInformation for AUv3SynthProcessor<'a> {

    fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            MemoryInputStream stream (data, static_cast<size_t> (sizeInBytes), false);

            isRecordingParam->setValueNotifyingHost (stream.readFloat());
            roomSizeParam->setValueNotifyingHost    (stream.readFloat());
        */
    }
}

pub trait SwapSamples {

    fn swap_samples(&mut self);
}

impl<'a> SwapSamples for AUv3SynthProcessor<'a> {

    fn swap_samples(&mut self)  {
        
        todo!();
        /*
            MemoryBlock mb;
            auto* stream = new MemoryOutputStream (mb, true);

            {
                std::unique_ptr<AudioFormatWriter> writer (formatManager.findFormatForFileExtension ("wav")->createWriterFor (stream, lastSampleRate, 1, 16,
                                                                                                                              StringPairArray(), 0));
                writer->writeFromAudioSampleBuffer (currentRecording, 0, currentRecording.getNumSamples());
                writer->flush();
                stream->flush();
            }

            loadNewSampleBinary (mb.getData(), static_cast<int> (mb.getSize()), "wav");
        */
    }
}

pub trait IsBusesLayoutSupported {

    fn is_buses_layout_supported(&self, layouts: &AudioProcessorBusesLayout) -> bool;
}

impl<'a> IsBusesLayoutSupported for AUv3SynthProcessor<'a> {
    fn is_buses_layout_supported(&self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            return (layouts.getMainOutputChannels() <= 2);
        */
    }
}

impl<'a> AUv3SynthProcessor<'a> {
    
    pub fn load_new_sample_binary(&mut self, 
        data:      *const c_void,
        data_size: i32,
        format:    *const u8)  {
        
        todo!();
        /*
            auto soundBuffer = std::make_unique<MemoryInputStream> (data, static_cast<std::size_t> (dataSize), false);
            loadNewSample (std::move (soundBuffer), format);
        */
    }
    
    pub fn load_new_sample<B: InputStream>(
        &mut self, 
        sound_buffer: B,
        format:       *const u8
    ) {
        
        todo!();
        /*
            std::unique_ptr<AudioFormatReader> formatReader (formatManager.findFormatForFileExtension (format)->createReaderFor (soundBuffer.release(), true));

            BigInteger midiNotes;
            midiNotes.setRange (0, 126, true);
            SynthesizerSound::Ptr newSound = new SamplerSound ("Voice", *formatReader, midiNotes, 0x40, 0.0, 0.0, 10.0);
            synth.removeSound (0);
            sound = newSound;
            synth.addSound (sound);
        */
    }
}
