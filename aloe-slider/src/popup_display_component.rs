crate::ix!();

#[no_copy]
#[leak_detector]
pub struct PopupDisplayComponent<'a> {
    base:  BubbleComponent<'a>,
    base2: Timer,
    owner: &'a mut Slider<'a>,
    font:  Font,
    text:  String,
}

impl<'a> Drop for PopupDisplayComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (owner.impl != nullptr)
                    owner.impl->lastPopupDismissal = Time::getMillisecondCounterHiRes();
        */
    }
}

impl<'a> PopupDisplayComponent<'a> {

    pub fn new(
        s:             &mut Slider,
        is_on_desktop: bool) -> Self {
    
        todo!();
        /*
        : owner(s),
        : font(s.getLookAndFeel().getSliderPopupFont (s)),

            if (isOnDesktop)
                    setTransform (AffineTransform::scale (Component::getApproximateScaleFactorForComponent (&s)));

                setAlwaysOnTop (true);
                setAllowedPlacement (owner.getLookAndFeel().getSliderPopupPlacement (s));
                setLookAndFeel (&s.getLookAndFeel());
        */
    }
    
    pub fn paint_content(&mut self, 
        g: &mut Graphics,
        w: i32,
        h: i32)  {
        
        todo!();
        /*
            g.setFont (font);
                g.setColour (owner.findColour (TooltipWindow::textColourId, true));
                g.drawFittedText (text, Rectangle<int> (w, h), Justification::centred, 1);
        */
    }
    
    pub fn get_content_size(&mut self, 
        w: &mut i32,
        h: &mut i32)  {
        
        todo!();
        /*
            w = font.getStringWidth (text) + 18;
                h = (int) (font.getHeight() * 1.6f);
        */
    }
    
    pub fn update_position(&mut self, new_text: &String)  {
        
        todo!();
        /*
            text = newText;
                BubbleComponent::setPosition (&owner);
                repaint();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();
                owner.impl->popupDisplay.reset();
        */
    }
}
