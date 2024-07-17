crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ScrollbarButton<'a> {
    base:      Button<'a>,
    direction: i32,
    owner:     &'a mut ScrollBar<'a>,
}

impl<'a> ScrollbarButton<'a> {

    pub fn new(
        direc: i32,
        s:     &mut ScrollBar) -> Self {
    
        todo!();
        /*
        : button(String()),
        : direction(direc),
        : owner(s),

            setWantsKeyboardFocus (false);
        */
    }
    
    pub fn paint_button(&mut self, 
        g:    &mut Graphics,
        over: bool,
        down: bool)  {
        
        todo!();
        /*
            getLookAndFeel().drawScrollbarButton (g, owner, getWidth(), getHeight(),
                                                      direction, owner.isVertical(), over, down);
        */
    }
    
    pub fn clicked(&mut self)  {
        
        todo!();
        /*
            owner.moveScrollbarInSteps ((direction == 1 || direction == 2) ? 1 : -1);
        */
    }
}
