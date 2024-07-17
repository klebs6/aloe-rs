crate::ix!();

#[no_copy]
#[leak_detector]
pub struct SimpleDeviceManagerInputLevelMeter<'a> {
    base:               Component<'a>,
    base2:              Timer,
    manager:            &'a mut AudioDeviceManager<'a>,
    input_level_getter: LevelMeterPtr,
    level:              f32, // default = 0
}

impl<'a> SimpleDeviceManagerInputLevelMeter<'a> {

    pub fn new(m: &mut AudioDeviceManager) -> Self {
    
        todo!();
        /*
        : manager(m),

            startTimerHz (20);
            inputLevelGetter = manager.getInputLevelGetter();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (isShowing())
            {
                auto newLevel = (float) inputLevelGetter->getCurrentLevel();

                if (std::abs (level - newLevel) > 0.005f)
                {
                    level = newLevel;
                    repaint();
                }
            }
            else
            {
                level = 0;
            }
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            // (add a bit of a skew to make the level more obvious)
            getLookAndFeel().drawLevelMeter (g, getWidth(), getHeight(),
                                             (float) std::exp (std::log (level) / 3.0));
        */
    }
}
