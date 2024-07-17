crate::ix!();

#[no_copy]
#[leak_detector]
pub struct NormalComponentWrapper<'a> {
    base:   PopupMenuCustomComponent<'a>,
    width:  i32,
    height: i32,
}

impl<'a> NormalComponentWrapper<'a> {

    pub fn new(
        comp:                                         &mut Component,
        w:                                            i32,
        h:                                            i32,
        trigger_menu_item_automatically_when_clicked: bool) -> Self {
    
        todo!();
        /*


            : typename PopupMenu::PopupMenuCustomComponent (triggerMenuItemAutomaticallyWhenClicked),
                  width (w), height (h)
                addAndMakeVisible (comp);
        */
    }
    
    pub fn get_ideal_size(&mut self, 
        ideal_width:  &mut i32,
        ideal_height: &mut i32)  {
        
        todo!();
        /*
            idealWidth = width;
                idealHeight = height;
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            if (auto* child = getChildComponent (0))
                    child->setBounds (getLocalBounds());
        */
    }
}

