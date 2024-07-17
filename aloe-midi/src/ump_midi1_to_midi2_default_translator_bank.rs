crate::ix!();

pub type UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorChannelAccumulators
= [UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorPnAccumulator; 16];

pub type UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorChannelBanks
= [UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorBank; 16];

pub struct UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorBank {

    /**
      | We use the top bit to indicate whether
      | this bank is valid.  After reading the
      | spec, it's not clear how we should
      | determine whether there are valid
      | values, so we'll just assume that the
      | bank is valid once either the lsb or
      | msb have been written.
      */
    msb: u8, // default = 0x80
    lsb: u8, // default = 0x00
}

impl UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorBank {

    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return ! (msb & 0x80);
        */
    }
    
    pub fn get_msb(&self) -> u8 {
        
        todo!();
        /*
            return msb & 0x7f;
        */
    }
    
    pub fn get_lsb(&self) -> u8 {
        
        todo!();
        /*
            return lsb & 0x7f;
        */
    }
    
    pub fn set_msb(&mut self, i: u8)  {
        
        todo!();
        /*
            msb = i & 0x7f;
        */
    }
    
    pub fn set_lsb(&mut self, i: u8)  {
        
        todo!();
        /*
            msb &= 0x7f; lsb = i & 0x7f;
        */
    }
}
