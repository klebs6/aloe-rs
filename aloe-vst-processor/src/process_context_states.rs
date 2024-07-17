crate::ix!();

/**
  | Transport state & other flags
  |
  */
pub enum ProcessContextStatesAndFlags
{
    /**
      | currently playing
      |
      */
    Playing          = 1 << 1,     

    /**
      | cycle is active
      |
      */
    CycleActive      = 1 << 2,     

    /**
      | currently recording
      |
      */
    Recording        = 1 << 3,     

    /**
      | systemTime contains valid information
      |
      */
    SystemTimeValid  = 1 << 8,     

    /**
      | continousTimeSamples contains valid
      | information
      |
      */
    ContTimeValid    = 1 << 17,    

    /**
      | projectTimeMusic contains valid information
      |
      */
    ProjectTimeMusicValid = 1 << 9, 

    /**
      | barPositionMusic contains valid information
      |
      */
    BarPositionValid = 1 << 11,    

    /**
      | cycleStartMusic and barPositionMusic
      | contain valid information
      |
      */
    CycleValid       = 1 << 12,    

    /**
      | tempo contains valid information
      |
      */
    TempoValid       = 1 << 10,    

    /**
      | timeSigNumerator and timeSigDenominator
      | contain valid information
      |
      */
    TimeSigValid     = 1 << 13,    

    /**
      | chord contains valid information
      |
      */
    ChordValid       = 1 << 18,    

    /**
      | smpteOffset and frameRate contain
      | valid information
      |
      */
    SmpteValid       = 1 << 14,    

    /**
      | samplesToNextClock valid
      |
      */
    ClockValid       = 1 << 15,     
}
