crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DSP/GainDemo.h]

pub struct GainDemo<'a> {
    base:                  Component<'a>,
    file_reader_component: AudioFileReaderComponent<'a /*, GainDemoDSP<'a> */ >,
}

impl<'a> Default for GainDemo<'a> {
    
    fn default() -> Self {

        todo!();

        /*

            addAndMakeVisible (fileReaderComponent);
            setSize (750, 500)
        */
    }
}

impl<'a> GainDemo<'a> {

    pub fn resized(&mut self)  {
        
        todo!();

        /*
            fileReaderComponent.setBounds (getLocalBounds());
        */
    }
}
