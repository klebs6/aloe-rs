crate::ix!();

pub struct ConvolutionControls<'a> {
    base:   Component<'a>,
    cab:    AttachedToggle<'a>,
    reverb: AttachedToggle<'a>,
    mix:    AttachedSlider<'a>,
}

impl<'a> ConvolutionControls<'a> {
    
    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : cab(editor, state.convolutionCabEnabled),
        : reverb(editor, state.convolutionReverbEnabled),
        : mix(editor, state.convolutionReverbMix),

            addAllAndMakeVisible (*this, cab, reverb, mix);
        */
    }
}

impl<'a> Resized for ConvolutionControls<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), cab, reverb, mix);
        */
    }
}
