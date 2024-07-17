crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/GainPluginDemo.h]

#[no_copy]
#[leak_detector]
pub struct GainProcessor<'a> {
    base: AudioProcessor<'a>,
    gain: *mut AudioParameterFloat,
}

impl<'a> Default for GainProcessor<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : AudioProcessor (BusesProperties().withInput  ("Input",  AudioChannelSet::stereo())
                                               .withOutput ("Output", AudioChannelSet::stereo()))

            addParameter (gain = new AudioParameterFloat ("gain", "Gain", 0.0f, 1.0f, 0.5f))
        */
    }
}

impl<'a> GainProcessor<'a> {
    
    pub fn prepare_to_play(&mut self, _0: f64, _1: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn release_resources(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn process_block(&mut self, 
        buffer: &mut AudioBuffer<f32>,
        _1:     &mut MidiBuffer)  {
        
        todo!();
        /*
            buffer.applyGain (*gain);
        */
    }
    
    pub fn process_block_f64(
        &mut self, 
        buffer: &mut AudioBuffer<f64>,
        _1:     &mut MidiBuffer
    ) {
        
        todo!();
        /*
            buffer.applyGain ((float) *gain);
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
            return "Gain PlugIn";
        */
    }
    
    pub fn accepts_midi(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn produces_midi(&self) -> bool {
        
        todo!();
        /*
            return false;
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
            MemoryOutputStream (destData, true).writeFloat (*gain);
        */
    }
    
    pub fn set_state_information(&mut self, 
        data:          *const c_void,
        size_in_bytes: i32)  {
        
        todo!();
        /*
            gain->setValueNotifyingHost (MemoryInputStream (data, static_cast<size_t> (sizeInBytes), false).readFloat());
        */
    }
    
    pub fn is_buses_layout_supported(&self, layouts: &AudioProcessorBusesLayout) -> bool {
        
        todo!();
        /*
            const auto& mainInLayout  = layouts.getChannelSet (true,  0);
            const auto& mainOutLayout = layouts.getChannelSet (false, 0);

            return (mainInLayout == mainOutLayout && (! mainInLayout.isDisabled()));
        */
    }
}
