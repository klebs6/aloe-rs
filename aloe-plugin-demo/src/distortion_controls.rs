crate::ix!();

pub struct DistortionControls<'a> {
    base:         Component<'a>,
    toggle:       AttachedToggle<'a>,
    lowpass:      AttachedSlider<'a>,
    highpass:     AttachedSlider<'a>,
    mix:          AttachedSlider<'a>,
    gain:         AttachedSlider<'a>,
    compv:        AttachedSlider<'a>,
    ty:           AttachedCombo<'a>,
    oversampling: AttachedCombo<'a>,
}

impl<'a> DistortionControls<'a> {

    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : toggle(editor, state.distortionEnabled),
        : lowpass(editor, state.distortionLowpass),
        : highpass(editor, state.distortionHighpass),
        : mix(editor, state.distortionMix),
        : gain(editor, state.distortionInGain),
        : compv(editor, state.distortionCompGain),
        : ty(editor, state.distortionType),
        : oversampling(editor, state.distortionOversampler),

            addAllAndMakeVisible (*this, toggle, type, lowpass, highpass, mix, gain, compv, oversampling);
        */
    }
}

impl<'a> Resized for DistortionControls<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), toggle, type, gain, highpass, lowpass, compv, mix, oversampling);
        */
    }
}

