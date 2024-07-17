crate::ix!();

pub struct CompressorControls<'a> {
    base:      Component<'a>,
    toggle:    AttachedToggle<'a>,
    threshold: AttachedSlider<'a>,
    ratio:     AttachedSlider<'a>,
    attack:    AttachedSlider<'a>,
    release:   AttachedSlider<'a>,
}

impl<'a> CompressorControls<'a> {

    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : toggle(editor, state.compressorEnabled),
        : threshold(editor, state.compressorThreshold),
        : ratio(editor, state.compressorRatio),
        : attack(editor, state.compressorAttack),
        : release(editor, state.compressorRelease),

            addAllAndMakeVisible (*this, toggle, threshold, ratio, attack, release);
        */
    }
}

impl<'a> Resized for CompressorControls<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), toggle, threshold, ratio, attack, release);
        */
    }
}
