crate::ix!();

pub struct OverdriveDemo<'a> {
    base:                  Component<'a>,
    file_reader_component: AudioFileReaderComponent<'a /* OverdriveDemoDSP */>,
}

impl<'a> Default for OverdriveDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (fileReaderComponent);
            setSize (750, 500)
        */
    }
}

impl<'a> OverdriveDemo<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            fileReaderComponent.setBounds (getLocalBounds());
        */
    }
}
