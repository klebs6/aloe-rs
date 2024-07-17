crate::ix!();

pub struct SIMDRegisterDemo<'a> {
    base:                  Component<'a>,
    file_reader_component: AudioFileReaderComponent<'a/*,SIMDRegisterDemoDSP<'a>*/>,
}

impl<'a> Default for SIMDRegisterDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (fileReaderComponent);
            setSize (750, 500)
        */
    }
}

impl<'a> SIMDRegisterDemo<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            fileReaderComponent.setBounds (getLocalBounds());
        */
    }
}
