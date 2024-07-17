crate::ix!();

pub enum LadderFilterMode
{
    /**
      | low-pass 12 dB/octave
      |
      */
    LPF12,  

    /**
      | high-pass 12 dB/octave
      |
      */
    HPF12,  

    /**
      | band-pass 12 dB/octave
      |
      */
    BPF12,  

    /**
      | low-pass 24 dB/octave
      |
      */
    LPF24,  

    /**
      | high-pass 24 dB/octave
      |
      */
    HPF24,  

    /**
      | band-pass 24 dB/octave
      |
      */
    BPF24,   
}
