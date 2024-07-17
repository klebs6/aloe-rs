crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2Timer.h]

/**
   Timer for profiling. This has platform
   specific code and may not work on every
   platform.
  */
pub struct b2Timer {
    aloe_start_time: Time,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2Timer.cpp]
impl Default for b2Timer {

    fn default() -> Self {
    
        todo!();
        /*


            Reset();
        */
    }
}
    
impl b2Timer {

    /**
      | Reset the timer.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            aloeStartTime = Time::getCurrentTime();
        */
    }
    
    /**
      | Get the time since construction or the
      | last reset.
      |
      */
    pub fn get_milliseconds(&self) -> f32 {
        
        todo!();
        /*
            return static_cast<float32> ((Time::getCurrentTime() - aloeStartTime).inMilliseconds());
        */
    }
}
