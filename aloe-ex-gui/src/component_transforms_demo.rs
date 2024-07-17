crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/ComponentTransformsDemo.h]

#[no_copy]
#[leak_detector]
pub struct ComponentTransformsDemo<'a> {
    base:     Component<'a>,
    content:  Box<Component<'a>>,
    draggers: Vec<Box<CornerDragger<'a>>>,
}

impl<'a> Default for ComponentTransformsDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            content.reset (new WidgetsDemo (true));
            addAndMakeVisible (content.get());
            content->setSize (750, 500);

            for (int i = 0; i < 3; ++i)
            {
                auto* d = new CornerDragger();
                addAndMakeVisible (draggers.add (d));
            }

            draggers.getUnchecked (0)->relativePos = { 0.10f, 0.15f };
            draggers.getUnchecked (1)->relativePos = { 0.95f, 0.05f };
            draggers.getUnchecked (2)->relativePos = { 0.05f, 0.85f };

            setSize (800, 600)
        */
    }
}

impl<'a> ComponentTransformsDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));

            g.setColour (Colours::white);
            g.setFont (15.0f);
            g.drawFittedText ("Drag the corner-points around to show how complex components can have affine-transforms applied...",
                              getLocalBounds().removeFromBottom (40).reduced (10, 0), Justification::centred, 3);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            for (auto* d : draggers)
                d->setCentrePosition (proportionOfWidth  (d->relativePos.x),
                                      proportionOfHeight (d->relativePos.y));
        */
    }
    
    pub fn child_bounds_changed(&mut self, child: *mut Component)  {
        
        todo!();
        /*
            if (dynamic_cast<CornerDragger*> (child) != nullptr)
                updateTransform();
        */
    }
    
    pub fn get_dragger_pos(&self, index: i32) -> Point<f32> {
        
        todo!();
        /*
            return draggers.getUnchecked (index)->getBounds().getCentre().toFloat();
        */
    }
    
    pub fn update_transform(&mut self)  {
        
        todo!();
        /*
            auto p0 = getDraggerPos (0);
            auto p1 = getDraggerPos (1);
            auto p2 = getDraggerPos (2);

            if (p0 != p1 && p1 != p2 && p0 != p2)
                content->setTransform (AffineTransform::fromTargetPoints (0, 0, p0.x, p0.y,
                                                                          (float) content->getWidth(), 0,  p1.x, p1.y,
                                                                          0, (float) content->getHeight(), p2.x, p2.y));
        */
    }
}
