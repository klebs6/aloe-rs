crate::ix!();

pub enum PannerRule
{
    /**
      | regular 6 dB or linear panning rule,
      | allows the panned sound to be perceived
      | as having a constant level when summed
      | to mono
      |
      */
    Linear,          

    /**
      | both left and right are 1 when pan value
      | is 0, with left decreasing to 0 above
      | this value and right decreasing to 0
      | below it
      |
      */
    Balanced,        

    /**
      | alternate version of the regular 3 dB
      | panning rule with a sine curve
      |
      */
    Sin3dB,          

    /**
      | alternate version of the regular 4.5
      | dB panning rule with a sine curve
      |
      */
    Sin4p5dB,        

    /**
      | alternate version of the regular 6 dB
      | panning rule with a sine curve
      |
      */
    Sin6dB,          

    /**
      | regular 3 dB or constant power panning
      | rule, allows the panned sound to be perceived
      | as having a constant level regardless
      | of the pan position
      |
      */
    SquareRoot3dB,   

    /**
      | regular 4.5 dB panning rule, a compromise
      | option between 3 dB and 6 dB panning rules
      |
      */
    SquareRoot4p5dB,  
}
