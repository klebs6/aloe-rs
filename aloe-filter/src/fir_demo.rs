crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DSP/FIRFilterDemo.h]
pub struct FIRFilterDemo<'a> {
    base:                  Component<'a>,
    file_reader_component: AudioFileReaderComponent<'a /*<FIRFilterDemoDSP>*/ >,
}

impl<'a> Default for FIRFilterDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (fileReaderComponent);
            setSize (750, 500)
        */
    }
}

impl<'a> FIRFilterDemo<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            fileReaderComponent.setBounds (getLocalBounds());
        */
    }
}
