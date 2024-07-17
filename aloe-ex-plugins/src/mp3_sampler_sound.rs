crate::ix!();

/**
  | A class which contains all the information
  | related to sample-playback, such as sample
  | data, loop points, and loop kind.
  |
  | It is expected that multiple sampler voices
  | will maintain pointers to a single instance of
  | this class, to avoid redundant duplication of
  | sample data in memory.
  */
pub struct MPESamplerSound {
    sample:                 Box<Sample>,
    centre_frequency_in_hz: f64, // default = { 440.0  }
    loop_points:            Range<f64>,
    loop_mode:              LoopMode, // default = { LoopMode::none  }
}

impl MPESamplerSound {

    pub fn set_sample(&mut self, value: Box<Sample>)  {
        
        todo!();
        /*
            sample = move (value);
            setLoopPointsInSeconds (loopPoints);
        */
    }
    
    pub fn get_sample(&self) -> *mut Sample {
        
        todo!();
        /*
            return sample.get();
        */
    }
    
    pub fn set_loop_points_in_seconds(&mut self, value: Range<f64>)  {
        
        todo!();
        /*
            loopPoints = sample == nullptr ? value
                                           : Range<double> (0, sample->getLength() / sample->getSampleRate())
                                                            .constrainRange (value);
        */
    }
    
    pub fn get_loop_points_in_seconds(&self) -> Range<f64> {
        
        todo!();
        /*
            return loopPoints;
        */
    }
    
    pub fn set_centre_frequency_in_hz(&mut self, centre: f64)  {
        
        todo!();
        /*
            centreFrequencyInHz = centre;
        */
    }
    
    pub fn get_centre_frequency_in_hz(&self) -> f64 {
        
        todo!();
        /*
            return centreFrequencyInHz;
        */
    }
    
    pub fn set_loop_mode(&mut self, ty: LoopMode)  {
        
        todo!();
        /*
            loopMode = type;
        */
    }
    
    pub fn get_loop_mode(&self) -> LoopMode {
        
        todo!();
        /*
            return loopMode;
        */
    }
}
