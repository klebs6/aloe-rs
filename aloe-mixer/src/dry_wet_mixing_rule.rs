crate::ix!();

pub enum DryWetMixingRule
{
    /**
      | dry volume is equal to 1 - wet volume
      |
      */
    linear,          

    /**
      | both dry and wet are 1 when mix is 0.5,
      | with dry decreasing to 0 above this value
      | and wet decreasing to 0 below it
      |
      */
    balanced,        

    /**
      | alternate dry/wet mixing rule using
      | the 3 dB sine panning rule
      |
      */
    sin3dB,          

    /**
      | alternate dry/wet mixing rule using
      | the 4.5 dB sine panning rule
      |
      */
    sin4p5dB,        

    /**
      | alternate dry/wet mixing rule using
      | the 6 dB sine panning rule
      |
      */
    sin6dB,          

    /**
      | alternate dry/wet mixing rule using
      | the regular 3 dB panning rule
      |
      */
    squareRoot3dB,   

    /**
      | alternate dry/wet mixing rule using
      | the regular 4.5 dB panning rule
      |
      */
    squareRoot4p5dB  
}
