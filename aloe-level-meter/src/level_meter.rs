crate::ix!();

/**
  | A simple reference-counted struct
  | that holds a level-meter value that
  | can be read using getCurrentLevel().
  | 
  | This is used to ensure that the level
  | processing code is only executed when
  | something holds a reference to one of
  | these objects and will be bypassed otherwise.
  | 
  | @see getInputLevelGetter, getOutputLevelGetter
  |
  */
pub struct LevelMeter {
    level: Atomic<f32>,
}

impl Default for LevelMeter {

    fn default() -> Self {
        Self {
            level: Atomic::<f32>::new(0.0),
        }
    }
}

pub type LevelMeterPtr = Rc<RefCell<LevelMeter>>;

impl LevelMeter {

    pub fn update_level(
        &mut self, 
        channel_data: *const *const f32,
        num_channels: i32,
        num_samples:  i32)  {

        todo!();
        /*
            if (getReferenceCount() <= 1)
            return;

        auto localLevel = level.get();

        if (numChannels > 0)
        {
            for (int j = 0; j < numSamples; ++j)
            {
                float s = 0;

                for (int i = 0; i < numChannels; ++i)
                    s += std::abs (channelData[i][j]);

                s /= (float) numChannels;

                const float decayFactor = 0.99992f;

                if (s > localLevel)
                    localLevel = s;
                else if (localLevel > 0.001f)
                    localLevel *= decayFactor;
                else
                    localLevel = 0;
            }
        }
        else
        {
            localLevel = 0;
        }

        level = localLevel;
        */
    }
    
    pub fn get_current_level(&self) -> f64 {
        
        todo!();
        /*
            jassert (getReferenceCount() > 1);
        return level.get();
        */
    }
}
