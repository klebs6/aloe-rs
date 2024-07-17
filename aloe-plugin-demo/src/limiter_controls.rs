crate::ix!();

pub struct LimiterControls<'a> {
    base:      Component<'a>,
    toggle:    AttachedToggle<'a>,
    threshold: AttachedSlider<'a>,
    release:   AttachedSlider<'a>,
}

impl<'a> LimiterControls<'a> {

    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : toggle(editor, state.limiterEnabled),
        : threshold(editor, state.limiterThreshold),
        : release(editor, state.limiterRelease),

            addAllAndMakeVisible (*this, toggle, threshold, release);
        */
    }
}

impl<'a> Resized for LimiterControls<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), toggle, threshold, release);
        */
    }
}
