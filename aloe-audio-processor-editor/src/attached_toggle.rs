crate::ix!();

pub struct AttachedToggle<'a> {
    base:       ComponentWithParamMenu<'a>,
    toggle:     ToggleButton<'a>,
    attachment: ButtonParameterAttachment<'a>,
}

impl<'a> AttachedToggle<'a> {

    pub fn new(
        editor_in: &mut AudioProcessorEditor,
        param_in:  &mut RangedAudioParameter) -> Self {
    
        todo!();
        /*
        : component_with_param_menu(editorIn, paramIn),
        : toggle(paramIn.name),
        : attachment(paramIn, toggle),

            toggle.addMouseListener (this, true);
                addAndMakeVisible (toggle);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            toggle.setBounds (getLocalBounds());
        */
    }
}
