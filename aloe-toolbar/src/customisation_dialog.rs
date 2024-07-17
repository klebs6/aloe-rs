crate::ix!();

pub struct ToolbarCustomisationDialog<'a> {
    base:    DialogWindow<'a>,
    toolbar: &'a mut Toolbar<'a>,
}

impl<'a> Drop for ToolbarCustomisationDialog<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            toolbar.setEditingActive (false);
        */
    }
}

impl<'a> ToolbarCustomisationDialog<'a> {

    pub fn new(
        factory:      &mut dyn ToolbarItemFactory,
        bar:          &mut Toolbar,
        option_flags: i32) -> Self {
    
        todo!();
        /*


            : DialogWindow (TRANS("Add/remove items from toolbar"), Colours::white, true, true),
              toolbar (bar)

            setContentOwned (new ToolbarCustomisationDialogCustomiserPanel (factory, toolbar, optionFlags), true);
            setResizable (true, true);
            setResizeLimits (400, 300, 1500, 1000);
            positionNearBar();
        */
    }
    
    pub fn close_button_pressed(&mut self)  {
        
        todo!();
        /*
            setVisible (false);
        */
    }
    
    pub fn can_modal_event_be_sent_to_component(&mut self, comp: *const Component) -> bool {
        
        todo!();
        /*
            return toolbar.isParentOf (comp)
                     || dynamic_cast<const ToolbarItemComponent::ItemDragAndDropOverlayComponent*> (comp) != nullptr;
        */
    }
    
    pub fn position_near_bar(&mut self)  {
        
        todo!();
        /*
            auto screenSize = toolbar.getParentMonitorArea();
            auto pos = toolbar.getScreenPosition();
            const int gap = 8;

            if (toolbar.isVertical())
            {
                if (pos.x > screenSize.getCentreX())
                    pos.x -= getWidth() - gap;
                else
                    pos.x += toolbar.getWidth() + gap;
            }
            else
            {
                pos.x += (toolbar.getWidth() - getWidth()) / 2;

                if (pos.y > screenSize.getCentreY())
                    pos.y -= getHeight() - gap;
                else
                    pos.y += toolbar.getHeight() + gap;
            }

            setTopLeftPosition (pos);
        */
    }
}
