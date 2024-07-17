crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/MidiLoggerPluginDemo.h]

#[no_copy]
#[leak_detector]
pub struct MidiLoggerPluginDemoProcessor<'a> {
    base: &'a mut dyn AudioProcessorInterface,
    base2: Timer,
    state: ValueTree, // default = { "state"  }
    queue: MidiQueue,

    /**
      | The data to show in the UI. We keep it
      | around in the processor so that the view is
      | persistent even when the plugin UI is closed
      | and reopened.
      */
    model: MidiListModel,
}

impl<'a> Default for MidiLoggerPluginDemoProcessor<'a> {
    
    fn default() -> Self {
        todo!();
        /*
            : AudioProcessor (getBusesLayout())

            state.addChild ({ "uiState", { { "width",  500 }, { "height", 300 } }, {} }, -1, nullptr);
            startTimerHz (60)
        */
    }
}

impl<'a> Drop for MidiLoggerPluginDemoProcessor<'a> {

    fn drop(&mut self) {
        todo!();
        /*      stopTimer();  */
    }
}

impl<'a> MidiLoggerPluginDemoProcessor<'a> {
    
    pub fn process_block_f32(
        &mut self, 
        audio: &mut AudioBuffer<f32>,
        midi:  &mut MidiBuffer

    ) {
        
        todo!();
        /*
            process (audio, midi);
        */
    }
    
    pub fn process_block(
        &mut self, 
        audio: &mut AudioBuffer<f64>,
        midi:  &mut MidiBuffer

    ) {
        
        todo!();
        /*
            process (audio, midi);
        */
    }
    
    pub fn is_buses_layout_supported(&self, _0: &dyn AudioProcessorBusesLayoutInterface) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn is_midi_effect(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn create_editor(&mut self) -> *mut dyn AudioProcessorEditorInterface {
        
        todo!();
        /*
            return new Editor (*this);
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return "MIDI Logger";
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
            return 0.0;
        */
    }
    
    pub fn get_num_programs(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
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
    
    pub fn get_state_information(&mut self, dest_data: &mut MemoryBlock)  {
        
        todo!();
        /*
            if (auto xmlState = state.createXml())
                copyXmlToBinary (*xmlState, destData);
        */
    }
    
    pub fn set_state_information(&mut self, 
        data: *const c_void,
        size: i32)  {
        
        todo!();
        /*
            if (auto xmlState = getXmlFromBinary (data, size))
                state = ValueTree::fromXml (*xmlState);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            std::vector<MidiMessage> messages;
            queue.pop (std::back_inserter (messages));
            model.addMessages (messages.begin(), messages.end());
        */
    }
    
    pub fn process<Element>(&mut self, 
        audio: &mut AudioBuffer<Element>,
        midi:  &mut MidiBuffer)  {
    
        todo!();
        /*
            audio.clear();
            queue.push (midi);
        */
    }
    
    pub fn get_buses_layout() -> &'a dyn AudioProcessorBusesPropertiesInterface {
        
        todo!();
        /*
            // Live doesn't like to load midi-only plugins, so we add an audio output there.
            return PluginHostType().isAbletonLive() ? AudioProcessorBusesProperties().withOutput ("out", AudioChannelSet::stereo())
                                                    : AudioProcessorBusesProperties();
        */
    }
}
