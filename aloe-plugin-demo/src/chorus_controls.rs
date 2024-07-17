crate::ix!();

pub struct ChorusControls<'a> {
    base:     Component<'a>,
    toggle:   AttachedToggle<'a>,
    rate:     AttachedSlider<'a>,
    depth:    AttachedSlider<'a>,
    centre:   AttachedSlider<'a>,
    feedback: AttachedSlider<'a>,
    mix:      AttachedSlider<'a>,
}

impl<'a> ChorusControls<'a> {

    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : toggle(editor, state.chorusEnabled),
        : rate(editor, state.chorusRate),
        : depth(editor, state.chorusDepth),
        : centre(editor, state.chorusCentreDelay),
        : feedback(editor, state.chorusFeedback),
        : mix(editor, state.chorusMix),

            addAllAndMakeVisible (*this, toggle, rate, depth, centre, feedback, mix);
        */
    }
}

impl<'a> Resized for ChorusControls<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), toggle, rate, depth, centre, feedback, mix);
        */
    }
}

