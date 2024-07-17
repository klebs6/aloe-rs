crate::ix!();

///------------------------
pub struct StateVariableFilterDemo<'a> {
    base:                  Component<'a>,
    file_reader_component: AudioFileReaderComponent<'a /*, StateVariableFilterDemoDSP<'a> */>,
}

impl<'a> Default for StateVariableFilterDemo<'a> {
    
    fn default() -> Self {

        todo!();

        /*
        addAndMakeVisible (fileReaderComponent);
        setSize (750, 500)
        */
    }
}

impl<'a> StateVariableFilterDemo<'a> {

    pub fn resized(&mut self)  {
        
        todo!();

        /*
            fileReaderComponent.setBounds (getLocalBounds());
        */
    }
}
