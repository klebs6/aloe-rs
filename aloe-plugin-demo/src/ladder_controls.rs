crate::ix!();

pub struct LadderControls<'a> {
    base:      Component<'a>,
    toggle:    AttachedToggle<'a>,
    mode:      AttachedCombo<'a>,
    freq:      AttachedSlider<'a>,
    resonance: AttachedSlider<'a>,
    drive:     AttachedSlider<'a>,
}

impl<'a> LadderControls<'a> {

    pub fn new(
        editor: &mut AudioProcessorEditor,
        state:  &DspModulePluginDemoParameterReferences

    ) -> Self {
    
        todo!();
        /*
        : toggle(editor, state.ladderEnabled),
        : mode(editor, state.ladderMode),
        : freq(editor, state.ladderCutoff),
        : resonance(editor, state.ladderResonance),
        : drive(editor, state.ladderDrive),

            addAllAndMakeVisible (*this, toggle, mode, freq, resonance, drive);
        */
    }
}

impl<'a> Resized for LadderControls<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            performLayout (getLocalBounds(), toggle, mode, freq, resonance, drive);
        */
    }
}
