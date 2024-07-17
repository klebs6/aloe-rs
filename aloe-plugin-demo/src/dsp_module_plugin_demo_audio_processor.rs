crate::ix!();

pub struct DspModulePluginDemoAudioProcessor<'a> {
    base: DspModulePluginDemo<'a>,
}

impl<'a> DspModulePluginDemoAudioProcessor<'a> {

    pub fn create_editor(&mut self) -> *mut AudioProcessorEditor {
        
        todo!();
        /*
            return new DspModulePluginDemoEditor (*this);
        */
    }
    
    pub fn has_editor(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
