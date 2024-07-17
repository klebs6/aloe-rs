crate::ix!();

pub struct IIRFilterDemo<'a> {
    base:                  Component<'a>,
    file_reader_component: AudioFileReaderComponent<'a /*<IIRFilterDemoDSP>*/ >,
}

impl<'a> Default for IIRFilterDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*

            addAndMakeVisible (fileReaderComponent);
            setSize (750, 500)
        */
    }
}

impl<'a> IIRFilterDemo<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            fileReaderComponent.setBounds (getLocalBounds());
        */
    }
}

