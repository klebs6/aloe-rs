/*!
  | Helpful types and functions for interacting
  | with Universal MIDI Packets.
  | 
  | @tags{Audio}
  |
  */

crate::ix!();

/**
  | Joins 4 bytes into a single 32-bit word.
  |
  */
pub fn universal_midi_packets_bytes_to_word(
    a: u8,
    b: u8,
    c: u8,
    d: u8) -> u32 {
    
    todo!();
    /*
        return uint32_t (a << 0x18 | b << 0x10 | c << 0x08 | d << 0x00);
    */
}

pub fn universal_midi_packets_get_message_type(w: u32) -> u8 {
    
    todo!();
    /*
        return U4<0>::get (w);
    */
}

pub fn universal_midi_packets_get_group(w: u32) -> u8 {
    
    todo!();
    /*
        return U4<1>::get (w);
    */
}

pub fn universal_midi_packets_get_status(w: u32) -> u8 {
    
    todo!();
    /*
        return U4<2>::get (w);
    */
}

pub fn universal_midi_packets_get_channel(w: u32) -> u8 {
    
    todo!();
    /*
        return U4<3>::get (w);
    */
}

/**
  | Returns the expected number of 32-bit
  | words in a Universal MIDI Packet, given
  | the first word of the packet.
  | 
  | The result will be between 1 and 4 inclusive.
  | A result of 1 means that the word is itself
  | a complete packet.
  |
  */
pub fn universal_midi_packets_get_num_words_for_message_type(mt: u32) -> u32 {
    
    todo!();
    /*
        switch (Utils::getMessageType (mt))
    {
        case 0x0:
        case 0x1:
        case 0x2:
        case 0x6:
        case 0x7:
            return 1;
        case 0x3:
        case 0x4:
        case 0x8:
        case 0x9:
        case 0xa:
            return 2;
        case 0xb:
        case 0xc:
            return 3;
        case 0x5:
        case 0xd:
        case 0xe:
        case 0xf:
            return 4;
    }

    jassertfalse;
    return 1;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPUtils.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPUtils.cpp]

/**
  | Helper functions for setting/getting
  | 4-bit ranges inside a 32-bit word.
  |
  */
pub struct U4<const INDEX: u32> { }

impl<const INDEX: u32> U4<INDEX> {

    pub const shift: u32 = (0x1c as u32) - INDEX * 4;
    
    pub fn set(
        word:  u32,
        value: u8) -> u32 {
        
        todo!();
        /*
            return (word & ~((uint32_t) 0xf << shift)) | (uint32_t) ((value & 0xf) << shift);
        */
    }
    
    pub fn get(word: u32) -> u8 {
        
        todo!();
        /*
            return (uint8_t) ((word >> shift) & 0xf);
        */
    }
}

/**
  | Helper functions for setting/getting
  | 8-bit ranges inside a 32-bit word.
  |
  */
pub struct U8<const INDEX: u32> { }

impl<const INDEX: u32> U8<INDEX> {

    pub const shift: u32 = (0x18 as u32) - INDEX * 8;

    pub fn set(
        word:  u32,
        value: u8) -> u32 {
        
        todo!();
        /*
            return (word & ~((uint32_t) 0xff << shift)) | (uint32_t) (value << shift);
        */
    }
    
    pub fn get(word: u32) -> u8 {
        
        todo!();
        /*
            return (uint8_t) ((word >> shift) & 0xff);
        */
    }
}

/**
  | Helper functions for setting/getting
  | 16-bit ranges inside a 32-bit word.
  |
  */
pub struct U16<const INDEX: u32> { }

impl<const INDEX: u32> U16<INDEX> {

    pub const shift: u32 = (0x10 as u32) - INDEX * 16;

    pub fn set(
        word:  u32,
        value: u16) -> u32 {
        
        todo!();
        /*
            return (word & ~((uint32_t) 0xffff << shift)) | (uint32_t) (value << shift);
        */
    }
    
    pub fn get(word: u32) -> u16 {
        
        todo!();
        /*
            return (uint16_t) ((word >> shift) & 0xffff);
        */
    }
}
