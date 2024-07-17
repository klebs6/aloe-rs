crate::ix!();

///------------------------------
#[no_copy]
pub struct AudioProcessorEditorListener<'a> {
    ed:   &'a mut AudioProcessorEditor<'a>,
}

impl<'a> AudioProcessorEditorListener<'a> {

    pub fn new(e: &'a mut AudioProcessorEditor<'a>) -> Self {
        Self {
            ed: e
        }
    }
}

impl<'a> ComponentListener               for AudioProcessorEditorListener<'a> { }
impl<'a> ComponentEnablementChanged      for AudioProcessorEditorListener<'a> { }
impl<'a> ComponentNameChanged            for AudioProcessorEditorListener<'a> { }
impl<'a> ComponentBeingDeleted           for AudioProcessorEditorListener<'a> { }
impl<'a> ComponentBroughtToFront         for AudioProcessorEditorListener<'a> {}
impl<'a> ComponentVisibilityChanged      for AudioProcessorEditorListener<'a> {}
impl<'a> ComponentChildrenChanged        for AudioProcessorEditorListener<'a> {}
impl<'a> ComponentParentHierarchyChanged for AudioProcessorEditorListener<'a> {

    fn component_parent_hierarchy_changed(&mut self, _0: &mut Component)  {
        
        todo!();
        /*
            ed.updatePeer();
        */
    }
}

impl<'a> ComponentMovedOrResized for AudioProcessorEditorListener<'a> {

    fn component_moved_or_resized(&mut self, 
        _0:          &mut Component,
        _1:          bool,
        was_resized: bool)  {
        
        todo!();
        /*
            ed.editorResized (wasResized);
        */
    }
}
