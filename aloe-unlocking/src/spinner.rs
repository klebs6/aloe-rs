crate::ix!();

pub struct Spinner<'a> {
    base:  Component<'a>,
    base2: Timer,
}

impl<'a> Default for Spinner<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            startTimer (1000 / 50);
        */
    }
}

impl<'a> Spinner<'a> {

    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawSpinningWaitAnimation (g, Colours::darkgrey, 0, 0, getWidth(), getHeight());
        */
    }
}
