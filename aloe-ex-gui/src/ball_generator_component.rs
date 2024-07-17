crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/AnimationDemo.h]

/**
  | This will be the source of our balls and
  | can be dragged around.
  |
  */
#[no_copy]
#[leak_detector]
pub struct BallGeneratorComponent<'a> {
    base:        Component<'a>,
    constrainer: ComponentBoundsConstrainer,
    dragger:     ComponentDragger,
}

impl<'a> Default for BallGeneratorComponent<'a> {
    fn default() -> Self {
        todo!();
    }
}

impl<'a> BallGeneratorComponent<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto area = getLocalBounds().reduced (2);

            g.setColour (Colours::orange);
            g.drawRoundedRectangle (area.toFloat(), 10.0f, 2.0f);

            g.setColour (findColour (TextButton::textColourOffId));
            g.drawFittedText ("Drag Me!", area, Justification::centred, 1);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            // Just set the limits of our constrainer so that we don't drag ourselves off the screen
            constrainer.setMinimumOnscreenAmounts (getHeight(), getWidth(),
                                                   getHeight(), getWidth());
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            // Prepares our dragger to drag this Component
            dragger.startDraggingComponent (this, e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            // Moves this Component according to the mouse drag event and applies our constraints to it
            dragger.dragComponent (this, e, &constrainer);
        */
    }
}
