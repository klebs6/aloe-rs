crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DSP/WaveShaperTanhDemo.h]

pub struct WaveShaperTanhDemoDSP<'a> {
    shapers:            [WaveShaper<f32>; 2],             //{ { std::tanh }, { FastMathApproximations::tanh } };
    current_shaper_idx: i32,                              // default = 0
    accuracy:           ChoiceParameter<'a>,                  //{{ "No Approximation", "Use fast-math approximation" }, 1, "Accuracy" };
    parameters:         Vec<*mut DSPDemoParameterBase<'a>>,   //{ &accuracy };  // no params for this demo
}

impl<'a> WaveShaperTanhDemoDSP<'a> {

    pub fn prepare(&mut self, _0: &ProcessSpec)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn process(&mut self, context: &ProcessContextReplacing<f32>)  {
        
        todo!();
        /*
            shapers[currentShaperIdx].process (context);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            for (auto&& shaper : shapers)
                shaper.reset();
        */
    }
    
    pub fn update_parameters(&mut self)  {
        
        todo!();
        /*
            currentShaperIdx = jmin (numElementsInArray (shapers), accuracy.getCurrentSelectedID() - 1);
        */
    }
}

///---------------------------
pub struct WaveShaperTanhDemo<'a> {
    base:                  Component<'a>,
    file_reader_component: AudioFileReaderComponent<'a /*, WaveShaperTanhDemoDSP<'a> */>,
}

impl<'a> Default for WaveShaperTanhDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*

            addAndMakeVisible (fileReaderComponent);
            setSize (750, 500)
        */
    }
}

impl<'a> WaveShaperTanhDemo<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            fileReaderComponent.setBounds (getLocalBounds());
        */
    }
}
