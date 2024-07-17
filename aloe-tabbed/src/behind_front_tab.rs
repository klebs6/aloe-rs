crate::ix!();

#[no_copy]
pub struct TabbedButtonBarBehindFrontTabComp<'a> {
    base:  Component<'a>,
    owner: &'a mut TabbedButtonBar<'a>,
}

impl<'a> TabbedButtonBarBehindFrontTabComp<'a> {
    
    pub fn new(tb: &mut TabbedButtonBar) -> Self {
    
        todo!();
        /*
        : owner(tb),

            setInterceptsMouseClicks (false, false);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawTabAreaBehindFrontButton (owner, g, getWidth(), getHeight());
        */
    }
    
    pub fn enablement_changed(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
}

 
