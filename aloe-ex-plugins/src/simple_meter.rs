crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/InterAppAudioEffectPluginDemo.h]

/**
  | A very simple decaying meter.
  |
  */
#[no_copy]
#[leak_detector]
pub struct SimpleMeter<'a> {
    base:      Component<'a>,
    base2:     Timer,
    max_level: Atomic<f32>, // default = { 0.0 }
    level:     f32,         // default = 0
}

impl<'a> Default for SimpleMeter<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            startTimerHz (30)
        */
    }
}

impl<'a> SimpleMeter<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::transparentBlack);

            auto area = g.getClipBounds();
            g.setColour (getLookAndFeel().findColour (Slider::thumbColourId));
            g.fillRoundedRectangle (area.toFloat(), 6.0);

            auto unfilledHeight = area.getHeight() * (1.0 - level);
            g.reduceClipRegion (area.getX(), area.getY(),
                                area.getWidth(), (int) unfilledHeight);
            g.setColour (getLookAndFeel().findColour (Slider::trackColourId));
            g.fillRoundedRectangle (area.toFloat(), 6.0);
        */
    }

    /**
      | Called from the audio thread.
      |
      */
    pub fn update(&mut self, new_level: f32)  {
        
        todo!();
        /*
            // We don't care if maxLevel gets set to zero (in timerCallback) between the
            // load and the assignment.
            maxLevel = jmax (maxLevel.load(), newLevel);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            auto callbackLevel = maxLevel.exchange (0.0);

            float decayFactor = 0.95f;

            if (callbackLevel > level)
                level = callbackLevel;
            else if (level > 0.001)
                level *= decayFactor;
            else
                level = 0;

            repaint();
        */
    }
}
