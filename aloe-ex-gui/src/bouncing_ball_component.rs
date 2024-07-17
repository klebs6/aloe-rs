crate::ix!();

#[no_copy]
#[leak_detector]
pub struct BouncingBallComponent<'a> {
    base:        Component<'a>,
    base2:       Timer,
    colour:      Colour,
    ball_bounds: Rectangle<f32>,
    direction:   Point<f32>,
}

impl<'a> Default for BouncingBallComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setInterceptsMouseClicks (false, false);

            Random random;

            auto size = 10.0f + (float) random.nextInt (30);

            ballBounds.setBounds (random.nextFloat() * 100.0f,
                                  random.nextFloat() * 100.0f,
                                  size, size);

            direction.x = random.nextFloat() * 8.0f - 4.0f;
            direction.y = random.nextFloat() * 8.0f - 4.0f;

            colour = Colour ((uint32) random.nextInt())
                        .withAlpha (0.5f)
                        .withBrightness (0.7f);

            startTimer (60)
        */
    }
}

impl<'a> BouncingBallComponent<'a> {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (colour);
            g.fillEllipse (ballBounds - getPosition().toFloat());
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            ballBounds += direction;

            if (ballBounds.getX() < 0)                              direction.x =  std::abs (direction.x);
            if (ballBounds.getY() < 0)                              direction.y =  std::abs (direction.y);
            if (ballBounds.getRight()  > (float) getParentWidth())  direction.x = -std::abs (direction.x);
            if (ballBounds.getBottom() > (float) getParentHeight()) direction.y = -std::abs (direction.y);

            setBounds (ballBounds.getSmallestIntegerContainer());
        */
    }
}
