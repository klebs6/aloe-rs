crate::ix!();

pub struct BasicControls<'a> {
    base:   Component<'a>,
    pan:    AttachedSlider<'a>,
    input:  AttachedSlider<'a>,
    output: AttachedSlider<'a>,
}

impl<'a> BasicControls<'a> {

    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : pan(editor, state.pan),
        : input(editor, state.inputGain),
        : output(editor, state.outputGain),

            addAllAndMakeVisible (*this, pan, input, output);
        */
    }
}

impl<'a> Resized for BasicControls<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), input, output, pan);
        */
    }
}

