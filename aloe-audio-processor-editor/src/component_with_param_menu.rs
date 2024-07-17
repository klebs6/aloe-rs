crate::ix!();

pub struct ComponentWithParamMenu<'a> {
    base:   Component<'a>,
    editor: &'a mut AudioProcessorEditor<'a>,
    param:  &'a mut RangedAudioParameter,
}

impl<'a> ComponentWithParamMenu<'a> {

    pub fn new(
        editor_in: &mut AudioProcessorEditor,
        param_in:  &mut RangedAudioParameter) -> Self {
    
        todo!();
        /*
        : editor(editorIn),
        : param(paramIn),

        
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (e.mods.isRightButtonDown())
                    if (auto* c = editor.getHostContext())
                        if (auto menuInfo = c->getContextMenuForParameterIndex (&param))
                            menuInfo->getEquivalentPopupMenu().showMenuAsync ({});
        */
    }
}
