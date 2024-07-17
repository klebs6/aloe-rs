crate::ix!();

#[no_copy]
#[leak_detector]
pub struct BouncingBallsContainer<'a> {
    base: Component<'a>,
    dragger: ComponentDragger,
    balls:   Vec<Box<BouncingBallComponent<'a>>>,
}

impl<'a> BouncingBallsContainer<'a> {

    pub fn new(num_balls: i32) -> Self {
    
        todo!();
        /*
            for (int i = 0; i < numBalls; ++i)
            {
                auto* newBall = new BouncingBallComponent();
                balls.add (newBall);
                addAndMakeVisible (newBall);
            }
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            dragger.startDraggingComponent (this, e);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            // as there's no titlebar we have to manage the dragging ourselves
            dragger.dragComponent (this, e, nullptr);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (isOpaque())
                g.fillAll (Colours::white);
            else
                g.fillAll (Colours::blue.withAlpha (0.2f));

            g.setFont (16.0f);
            g.setColour (Colours::black);
            g.drawFittedText ("This window has no titlebar and a transparent background.",
                              getLocalBounds().reduced (8, 0),
                              Justification::centred, 5);

            g.drawRect (getLocalBounds());
        */
    }
}
