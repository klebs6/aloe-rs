crate::ix!();

pub struct DirectDelayControls<'a> {
    base:   Component<'a>,
    toggle: AttachedToggle<'a>,
    ty:     AttachedCombo<'a>,
    delay:  AttachedSlider<'a>,
    smooth: AttachedSlider<'a>,
    mix:    AttachedSlider<'a>,
}

impl<'a> DirectDelayControls<'a> {

    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : toggle(editor, state.directDelayEnabled),
        : ty(editor, state.directDelayType),
        : delay(editor, state.directDelayValue),
        : smooth(editor, state.directDelaySmoothing),
        : mix(editor, state.directDelayMix),

            addAllAndMakeVisible (*this, toggle, type, delay, smooth, mix);
        */
    }
}
    
impl<'a> Resized for DirectDelayControls<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), toggle, type, delay, smooth, mix);
        */
    }
}
