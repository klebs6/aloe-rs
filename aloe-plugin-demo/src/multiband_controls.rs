crate::ix!();

pub struct MultiBandControls<'a> {
    base:    Component<'a>,
    toggle:  AttachedToggle<'a>,
    low:     AttachedSlider<'a>,
    high:    AttachedSlider<'a>,
    l_rfreq: AttachedSlider<'a>,
}

impl<'a> MultiBandControls<'a> {

    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : toggle(editor, state.multiBandEnabled),
        : low(editor, state.multiBandLowVolume),
        : high(editor, state.multiBandHighVolume),
        : l_rfreq(editor, state.multiBandFreq),

            addAllAndMakeVisible (*this, toggle, low, high, lRFreq);
        */
    }
}

impl<'a> Resized for MultiBandControls<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), toggle, lRFreq, low, high);
        */
    }
}
