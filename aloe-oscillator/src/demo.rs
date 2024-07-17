crate::ix!();

pub struct OscillatorDemo<'a> {
    base:                  Component<'a>,
    file_reader_component: AudioFileReaderComponent<'a /*, OscillatorDemoDSP<'a> */>,
}

impl<'a> Default for OscillatorDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (fileReaderComponent);
            setSize (750, 500)
        */
    }
}

impl<'a> OscillatorDemo<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            fileReaderComponent.setBounds (getLocalBounds());
        */
    }
}

