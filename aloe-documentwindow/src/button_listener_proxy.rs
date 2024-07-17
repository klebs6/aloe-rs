crate::ix!();

#[no_copy]
#[leak_detector]
pub struct DocumentWindowButtonListenerProxy<'a> {
    owner: &'a mut DocumentWindow<'a>,
}

impl<'a> ButtonListener for DocumentWindowButtonListenerProxy<'a> {

    fn button_clicked(&mut self, button: *mut Button)  {
        
        todo!();
        /*
            if      (button == owner.getMinimiseButton())  owner.minimiseButtonPressed();
                else if (button == owner.getMaximiseButton())  owner.maximiseButtonPressed();
                else if (button == owner.getCloseButton())     owner.closeButtonPressed();
        */
    }
}

impl<'a> DocumentWindowButtonListenerProxy<'a> {

    pub fn new(w: &mut DocumentWindow<'a>) -> Self {
    
        todo!();
        /*
        : owner(w),

        
        */
    }
}
