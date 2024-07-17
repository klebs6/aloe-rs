crate::ix!();

pub struct NoiseGateControls<'a> {
    base:      Component<'a>,
    toggle:    AttachedToggle<'a>,
    threshold: AttachedSlider<'a>,
    ratio:     AttachedSlider<'a>,
    attack:    AttachedSlider<'a>,
    release:   AttachedSlider<'a>,
}

impl<'a> NoiseGateControls<'a> {

    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : toggle(editor, state.noiseGateEnabled),
        : threshold(editor, state.noiseGateThreshold),
        : ratio(editor, state.noiseGateRatio),
        : attack(editor, state.noiseGateAttack),
        : release(editor, state.noiseGateRelease),

            addAllAndMakeVisible (*this, toggle, threshold, ratio, attack, release);
        */
    }
}

impl<'a> Resized for NoiseGateControls<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), toggle, threshold, ratio, attack, release);
        */
    }
}
