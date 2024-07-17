crate::ix!();

/**
  | This is a small star button that is put
  | inside one of the tabs. You can use this
  | technique to create things like "close tab"
  | buttons, etc.
  */
pub struct CustomTabButton<'a> {
    base:                             Component<'a>,
    running_componen_transforms_demo: bool,
    bubble_message:                   Box<BubbleMessageComponent<'a>>,
}

impl<'a> CustomTabButton<'a> {

    pub fn new(is_running_componen_transforms_demo: bool) -> Self {
    
        todo!();
        /*
        : running_componen_transforms_demo(isRunningComponenTransformsDemo),

            setSize (20, 20);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            Path star;
                star.addStar ({}, 7, 1.0f, 2.0f);

                g.setColour (Colours::green);
                g.fillPath (star, star.getTransformToScaleToFit (getLocalBounds().reduced (2).toFloat(), true));
        */
    }
    
    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            showBubbleMessage (*this,
                                   "This is a custom tab component\n"
                                   "\n"
                                   "You can use these to implement things like close-buttons "
                                   "or status displays for your tabs.",
                                   bubbleMessage,
                                   runningComponenTransformsDemo);
        */
    }
}
