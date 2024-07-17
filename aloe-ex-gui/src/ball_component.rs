crate::ix!();

#[no_copy]
#[leak_detector]
pub struct BallComponent<'a> {
    base:     Component<'a>,
    position: Point<f32>,
    speed:    Point<f32>,
    colour:   Colour,
}

impl<'a> Paint for BallComponent<'a> {
    
    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (colour);
            g.fillEllipse (2.0f, 2.0f, (float) getWidth() - 4.0f, (float) getHeight() - 4.0f);

            g.setColour (Colours::darkgrey);
            g.drawEllipse (2.0f, 2.0f, (float) getWidth() - 4.0f, (float) getHeight() - 4.0f, 1.0f);
        */
    }
}

impl<'a> BallComponent<'a> {

    pub fn new(pos: Point<f32>) -> Self {
    
        todo!();
        /*


            : position (pos),
              speed (Random::getSystemRandom().nextFloat() *  4.0f - 2.0f,
                     Random::getSystemRandom().nextFloat() * -6.0f - 2.0f),
              colour (Colours::white)

            setSize (20, 20);
            step();
        */
    }
    
    pub fn step(&mut self) -> bool {
        
        todo!();
        /*
            position += speed;
            speed.y += 0.1f;

            setCentrePosition ((int) position.x,
                               (int) position.y);

            if (auto* parent = getParentComponent())
                return isPositiveAndBelow (position.x, (float) parent->getWidth())
                    && position.y < (float) parent->getHeight();

            return position.y < 400.0f && position.x >= -10.0f;
        */
    }
}
