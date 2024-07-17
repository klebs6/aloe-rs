crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/AnimationAppDemo.h]

/**
  | This component lives inside our window,
  | and this is where you should put all your
  | controls and content.
  |
  */
#[no_copy]
#[leak_detector]
pub struct AnimationAppDemo<'a> {
    base: AnimatedAppComponent<'a>,
}

impl<'a> Default for AnimationAppDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setSize (800, 600);
            setFramesPerSecond (60)
        */
    }
}

impl<'a> Update for AnimationAppDemo<'a> {

    fn update(&mut self)  {
        
        todo!();
        /*
            // This function is called at the frequency specified by the setFramesPerSecond() call
            // in the constructor. You can use it to update counters, animate values, etc.
        */
    }
}

impl<'a> Paint for AnimationAppDemo<'a> {
    
    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            // (Our component is opaque, so we must completely fill the background with a solid colour)
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));

            g.setColour (getLookAndFeel().findColour (Slider::thumbColourId));
            auto fishLength = 15;

            Path spinePath;

            for (auto i = 0; i < fishLength; ++i)
            {
                auto radius = 100 + 10 * std::sin ((float) getFrameCounter() * 0.1f + (float) i * 0.5f);

                Point<float> p ((float) getWidth()  / 2.0f + 1.5f * radius * std::sin ((float) getFrameCounter() * 0.02f + (float) i * 0.12f),
                                (float) getHeight() / 2.0f + 1.0f * radius * std::cos ((float) getFrameCounter() * 0.04f + (float) i * 0.12f));

                // draw the circles along the fish
                g.fillEllipse (p.x - (float) i, p.y - (float) i, 2.0f + 2.0f * (float) i, 2.0f + 2.0f * (float) i);

                if (i == 0)
                    spinePath.startNewSubPath (p);  // if this is the first point, start a new path..
                else
                    spinePath.lineTo (p);           // ...otherwise add the next point
            }

            // draw an outline around the path that we have created
            g.strokePath (spinePath, PathStrokeType (4.0f));
        */
    }
}

impl<'a> Resized for AnimationAppDemo<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            // This is called when this component is resized.
            // If you add any child components, this is where you should
            // update their positions.
        */
    }
}
