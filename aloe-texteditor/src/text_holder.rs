crate::ix!();

#[no_copy]
pub struct TextHolderComponent<'a> {
    base:  Component<'a>,
    base2: Timer,
    owner: &'a mut TextEditor<'a>,
}

impl<'a> ValueListener for TextHolderComponent<'a> {

    fn value_changed(&mut self, _0: &mut Value)  {
        
        todo!();
        /*
            owner.textWasChangedByValue();
        */
    }
}

impl<'a> Drop for TextHolderComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            owner.getTextValue().removeListener (this);
        */
    }
}

impl<'a> TextHolderComponent<'a> {
    
    pub fn new(ed: &mut TextEditor) -> Self {
    
        todo!();
        /*
        : owner(ed),

            setWantsKeyboardFocus (false);
            setInterceptsMouseClicks (false, true);
            setMouseCursor (MouseCursor::ParentCursor);

            owner.getTextValue().addListener (this);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            owner.drawContent (g);
        */
    }
    
    pub fn restart_timer(&mut self)  {
        
        todo!();
        /*
            startTimer (350);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            owner.timerCallbackInt();
        */
    }
}
