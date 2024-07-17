crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPSysEx7.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPSysEx7.cpp]

/**
  | Holds the bytes from a single SysEx-7
  | packet.
  |
  */
pub struct UniversalMidiPacketsSysEx7PacketBytes
{
    data: [u8; 6],
    size: u8,
}

/**
  | The different kinds of UMP SysEx-7 message.
  |
  */
#[repr(u8)]
pub enum UniversalMidiPacketsSysEx7Kind 
{
    /**
      | The whole message fits in a single 2-word
      | packet.
      |
      */
    complete     = 0,

    /**
      | The packet begins a SysEx message that
      | will continue in subsequent packets.
      |
      */
    begin        = 1,

    /**
      | The packet is a continuation of an ongoing
      | SysEx message.
      |
      */
    continuation = 2,

    /**
      | The packet terminates an ongoing SysEx
      | message.
      |
      */
    end          = 3
}

/**
  | This struct acts as a single-file namespace
  | for Univeral MIDI Packet functionality
  | related to 7-bit SysEx.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsSysEx7 {

}

impl UniversalMidiPacketsSysEx7 {

    /**
      | Returns the number of 64-bit packets
      | required to hold a series of SysEx bytes.
      | 
      | The number passed to this function should
      | exclude the leading/trailing SysEx
      | bytes used in an old midi bytestream,
      | as these are not required when using
      | Universal MIDI Packets.
      |
      */
    pub fn get_num_packets_required_for_data_size(&mut self, size: u32) -> u32 {
        
        todo!();
        /*
            constexpr auto denom = 6;
        return (size / denom) + ((size % denom) != 0);
        */
    }
    
    /**
      | Extracts the data bytes from a 64-bit
      | data message.
      |
      */
    pub fn get_data_bytes(&mut self, packet: &UniversalMidiPacketX2) -> UniversalMidiPacketsSysEx7PacketBytes {
        
        todo!();
        /*
            const auto numBytes = Utils::getChannel (packet[0]);
        constexpr uint8_t maxBytes = 6;
        jassert (numBytes <= maxBytes);

        return
        {
            { packet.getU8<2>(),
              packet.getU8<3>(),
              packet.getU8<4>(),
              packet.getU8<5>(),
              packet.getU8<6>(),
              packet.getU8<7>() },
            jmin (numBytes, maxBytes)
        };
        */
    }
}
