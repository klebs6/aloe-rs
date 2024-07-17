crate::ix!();

pub struct PhaserControls<'a> {
    base:     Component<'a>,
    toggle:   AttachedToggle<'a>,
    rate:     AttachedSlider<'a>,
    depth:    AttachedSlider<'a>,
    centre:   AttachedSlider<'a>,
    feedback: AttachedSlider<'a>,
    mix:      AttachedSlider<'a>,
}

impl<'a> PhaserControls<'a> {

    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : toggle(editor, state.phaserEnabled),
        : rate(editor, state.phaserRate),
        : depth(editor, state.phaserDepth),
        : centre(editor, state.phaserCentreFrequency),
        : feedback(editor, state.phaserFeedback),
        : mix(editor, state.phaserMix),

            addAllAndMakeVisible (*this, toggle, rate, depth, centre, feedback, mix);
        */
    }
}
    
impl<'a> Resized for PhaserControls<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), toggle, rate, depth, centre, feedback, mix);
        */
    }
}
