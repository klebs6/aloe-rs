crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/ArpeggiatorPluginDemo.h]

#[no_copy]
#[leak_detector]
pub struct Arpeggiator<'a> {
    base:            AudioProcessor<'a>,
    speed:           *mut AudioParameterFloat,
    current_note:    i32,
    last_note_value: i32,
    time:            i32,
    rate:            f32,
    notes:           SortedSet<i32>,
}

impl<'a> Default for Arpeggiator<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : AudioProcessor (BusesProperties()) // add no audio buses at all

            addParameter (speed = new AudioParameterFloat ("speed", "Arpeggiator Speed", 0.0, 1.0, 0.5))
        */
    }
}

impl<'a> Arpeggiator<'a> {
    
    pub fn prepare_to_play(&mut self, 
        sample_rate:       f64,
        samples_per_block: i32)  {
        
        todo!();
        /*
            ignoreUnused (samplesPerBlock);

            notes.clear();
            currentNote = 0;
            lastNoteValue = -1;
            time = 0;
            rate = static_cast<float> (sampleRate);
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn process_block(&mut self, 
        buffer: &mut AudioBuffer<f32>,
        midi:   &mut MidiBuffer)  {
        
        todo!();
        /*
            // the audio buffer in a midi effect will have zero channels!
            jassert (buffer.getNumChannels() == 0);

            // however we use the buffer to get timing information
            auto numSamples = buffer.getNumSamples();

            // get note duration
            auto noteDuration = static_cast<int> (std::ceil (rate * 0.25f * (0.1f + (1.0f - (*speed)))));

            for (const auto metadata : midi)
            {
                const auto msg = metadata.getMessage();
                if      (msg.isNoteOn())  notes.add (msg.getNoteNumber());
                else if (msg.isNoteOff()) notes.removeValue (msg.getNoteNumber());
            }

            midi.clear();

            if ((time + numSamples) >= noteDuration)
            {
                auto offset = jmax (0, jmin ((int) (noteDuration - time), numSamples - 1));

                if (lastNoteValue > 0)
                {
                    midi.addEvent (MidiMessage::noteOff (1, lastNoteValue), offset);
                    lastNoteValue = -1;
                }

                if (notes.size() > 0)
                {
                    currentNote = (currentNote + 1) % notes.size();
                    lastNoteValue = notes[currentNote];
                    midi.addEvent (MidiMessage::noteOn  (1, lastNoteValue, (uint8) 127), offset);
                }

            }

            time = (time + numSamples) % noteDuration;
        */
    }
    
    pub fn is_midi_effect(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return new GenericAudioProcessorEditor (*this);
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return "Arpeggiator";
        */
    }
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_tail_length_seconds(&self) -> f64 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
            return 1;
        */
    }
    
    pub fn get_current_program(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
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
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            MemoryOutputStream (destData, true).writeFloat (*speed);
        */
    }
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            speed->setValueNotifyingHost (MemoryInputStream (data, static_cast<size_t> (sizeInBytes), false).readFloat());
        */
    }
}
