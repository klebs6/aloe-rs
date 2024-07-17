crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DSP/ConvolutionDemo.h]
pub struct ConvolutionDemo<'a> {
    base:                  Component<'a>,
    file_reader_component: AudioFileReaderComponent<'a /*, ConvolutionDemoDSP*/ >,
}

impl<'a> Default for ConvolutionDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*
            addAndMakeVisible (fileReaderComponent);
            setSize (750, 500)
        */
    }
}

impl<'a> ConvolutionDemo<'a> {
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            fileReaderComponent.setBounds (getLocalBounds());
        */
    }
}
