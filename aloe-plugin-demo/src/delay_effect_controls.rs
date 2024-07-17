crate::ix!();

pub struct DelayEffectControls<'a> {
    base:     Component<'a>,
    toggle:   AttachedToggle<'a>,
    ty:       AttachedCombo<'a>,
    value:    AttachedSlider<'a>,
    smooth:   AttachedSlider<'a>,
    lowpass:  AttachedSlider<'a>,
    feedback: AttachedSlider<'a>,
    mix:      AttachedSlider<'a>,
}

impl<'a> DelayEffectControls<'a> {

    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : toggle(editor, state.delayEffectEnabled),
        : ty(editor, state.delayEffectType),
        : value(editor, state.delayEffectValue),
        : smooth(editor, state.delayEffectSmoothing),
        : lowpass(editor, state.delayEffectLowpass),
        : feedback(editor, state.delayEffectFeedback),
        : mix(editor, state.delayEffectMix),

            addAllAndMakeVisible (*this, toggle, type, value, smooth, lowpass, feedback, mix);
        */
    }
}

impl<'a> Resized for DelayEffectControls<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), toggle, type, value, smooth, lowpass, feedback, mix);
        */
    }
}
