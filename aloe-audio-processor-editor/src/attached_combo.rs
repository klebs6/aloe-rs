crate::ix!();

pub struct ComboWithItems<'a> {
    base: ComboBox<'a>,
}

impl<'a> ComboWithItems<'a> {

    pub fn new(param: &mut RangedAudioParameter) -> Self {
    
        todo!();
        /*
        // Adding the list here in the constructor means that the combo
        // is already populated when we construct the attachment below
        addItemList (dynamic_cast<AudioParameterChoice&> (param).choices, 1);
        */
    }
}

pub struct AttachedCombo<'a> {
    base:       ComponentWithParamMenu<'a>,
    combo:      ComboWithItems<'a>,
    label:      Label<'a>,
    attachment: ComboBoxParameterAttachment<'a>,
}

impl<'a> AttachedCombo<'a> {

    pub fn new(
        editor_in: &mut AudioProcessorEditor,
        param_in:  &mut RangedAudioParameter) -> Self {
    
        todo!();
        /*


            : ComponentWithParamMenu (editorIn, paramIn),
                  combo (paramIn),
                  label ("", paramIn.name),
                  attachment (paramIn, combo)

                combo.addMouseListener (this, true);

                addAllAndMakeVisible (*this, combo, label);

                label.attachToComponent (&combo, false);
                label.setJustificationType (Justification::centred);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            combo.setBounds (getLocalBounds().withSizeKeepingCentre (jmin (getWidth(), 150), 24));
        */
    }
}
