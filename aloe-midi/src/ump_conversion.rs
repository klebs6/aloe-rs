crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPConversion.h]

/**
  | Functions to assist conversion of UMP
  | messages to/from other formats, especially
  | older 'bytestream' formatted MidiMessages.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketConversion {

}

impl UniversalMidiPacketConversion {

    /**
      | Converts from a MIDI 1 bytestream to
      | MIDI 1 on Universal MIDI UniversalMidiPackets.
      | 
      | `callback` is a function which accepts
      | a single UniversalMidiPacketsView argument.
      |
      */
    pub fn to_midi1_with_callback<PacketCallbackFunction>(
        m:        &MidiMessage,
        callback: PacketCallbackFunction)  {
    
        todo!();
        /*
            const auto* data = m.getRawData();
            const auto firstByte = data[0];
            const auto size = m.getRawDataSize();

            if (firstByte != 0xf0)
            {
                const auto mask = [size]() -> uint32_t
                {
                    switch (size)
                    {
                        case 0: return 0xff000000;
                        case 1: return 0xffff0000;
                        case 2: return 0xffffff00;
                        case 3: return 0xffffffff;
                    }

                    return 0x00000000;
                }();

                const auto extraByte = (uint8_t) ((((firstByte & 0xf0) == 0xf0) ? 0x1 : 0x2) << 0x4);
                const PacketX1 packet { mask & Utils::bytesToWord (extraByte, data[0], data[1], data[2]) };
                callback (UniversalMidiPacketsView (packet.data()));
                return;
            }

            const auto numSysExBytes = m.getSysExDataSize();
            const auto numMessages = SysEx7::getNumPacketsRequiredForDataSize ((uint32_t) numSysExBytes);
            auto* dataOffset = m.getSysExData();

            if (numMessages <= 1)
            {
                const auto packet = Factory::makeSysExIn1Packet (0, (uint8_t) numSysExBytes, dataOffset);
                callback (UniversalMidiPacketsView (packet.data()));
                return;
            }

            constexpr auto byteIncrement = 6;

            for (auto i = numSysExBytes; i > 0; i -= byteIncrement, dataOffset += byteIncrement)
            {
                const auto func = [&]
                {
                    if (i == numSysExBytes)
                        return Factory::makeSysExStart;

                    if (i <= byteIncrement)
                        return Factory::makeSysExEnd;

                    return Factory::makeSysExContinue;
                }();

                const auto bytesNow = std::min (byteIncrement, i);
                const auto packet = func (0, (uint8_t) bytesNow, dataOffset);
                callback (UniversalMidiPacketsView (packet.data()));
            }
        */
    }

    /**
      | Converts a MidiMessage to one or more
      | messages in UMP format, using the MIDI
      | 1.0 Protocol.
      | 
      | `packets` is an out-param to allow the
      | caller to control allocation/deallocation.
      | Returning a new UniversalMidiPackets object would
      | require every call to toMidi1 to allocate.
      | With this version, no allocations will
      | occur, provided that `packets` has
      | adequate reserved space.
      |
      */
    pub fn to_midi1(
        m:       &MidiMessage,
        packets: &mut UniversalMidiPackets)  {
        
        todo!();
        /*
            toMidi1 (m, [&] (const UniversalMidiPacketsView& view) { packets.add (view); });
        */
    }

    /**
      | Widens a 7-bit MIDI 1.0 value to a 8-bit
      | MIDI 2.0 value.
      |
      */
    pub fn scale_to8(word_7bit: u8) -> u8 {
        
        todo!();
        /*
            const auto shifted = (uint8_t) (word7Bit << 0x1);
            const auto repeat = (uint8_t) (word7Bit & 0x3f);
            const auto mask = (uint8_t) (word7Bit <= 0x40 ? 0x0 : 0xff);
            return (uint8_t) (shifted | ((repeat >> 5) & mask));
        */
    }

    /**
      | Widens a 7-bit MIDI 1.0 value to a 16-bit
      | MIDI 2.0 value.
      |
      */
    pub fn scale_to16(word_7bit: u8) -> u16 {
        
        todo!();
        /*
            const auto shifted = (uint16_t) (word7Bit << 0x9);
            const auto repeat = (uint16_t) (word7Bit & 0x3f);
            const auto mask = (uint16_t) (word7Bit <= 0x40 ? 0x0 : 0xffff);
            return (uint16_t) (shifted | (((repeat << 3) | (repeat >> 3)) & mask));
        */
    }

    /**
      | Widens a 14-bit MIDI 1.0 value to a 16-bit
      | MIDI 2.0 value.
      |
      */
    pub fn scale_to16_with_u16(word_14bit: u16) -> u16 {
        
        todo!();
        /*
            const auto shifted = (uint16_t) (word14Bit << 0x2);
            const auto repeat = (uint16_t) (word14Bit & 0x1fff);
            const auto mask = (uint16_t) (word14Bit <= 0x2000 ? 0x0 : 0xffff);
            return (uint16_t) (shifted | ((repeat >> 11) & mask));
        */
    }

    /**
      | Widens a 7-bit MIDI 1.0 value to a 32-bit
      | MIDI 2.0 value.
      |
      */
    pub fn scale_to32(word_7bit: u8) -> u32 {
        
        todo!();
        /*
            const auto shifted = (uint32_t) (word7Bit << 0x19);
            const auto repeat = (uint32_t) (word7Bit & 0x3f);
            const auto mask = (uint32_t) (word7Bit <= 0x40 ? 0x0 : 0xffffffff);
            return (uint32_t) (shifted | (((repeat << 19)
                                         | (repeat << 13)
                                         | (repeat << 7)
                                         | (repeat << 1)
                                         | (repeat >> 5)) & mask));
        */
    }

    /**
      | Widens a 14-bit MIDI 1.0 value to a 32-bit
      | MIDI 2.0 value.
      |
      */
    pub fn scale_to32_with_u16(word_14bit: u16) -> u32 {
        
        todo!();
        /*
            const auto shifted = (uint32_t) (word14Bit << 0x12);
            const auto repeat = (uint32_t) (word14Bit & 0x1fff);
            const auto mask = (uint32_t) (word14Bit <= 0x2000 ? 0x0 : 0xffffffff);
            return (uint32_t) (shifted | (((repeat << 5) | (repeat >> 8)) & mask));
        */
    }

    /**
      | Narrows a 16-bit MIDI 2.0 value to a 7-bit
      | MIDI 1.0 value.
      |
      */
    pub fn scale_to7_with_u8(word_8bit: u8) -> u8 {
        
        todo!();
        /*
            return (uint8_t) (word8Bit >> 1);
        */
    }

    /**
      | Narrows a 16-bit MIDI 2.0 value to a 7-bit
      | MIDI 1.0 value.
      |
      */
    pub fn scale_to7_with_u16(word_16bit: u16) -> u8 {
        
        todo!();
        /*
            return (uint8_t) (word16Bit >> 9);
        */
    }

    /**
      | Narrows a 32-bit MIDI 2.0 value to a 7-bit
      | MIDI 1.0 value.
      |
      */
    pub fn scale_to7_with_u32(word_32bit: u32) -> u8 {
        
        todo!();
        /*
            return (uint8_t) (word32Bit >> 25);
        */
    }

    /**
      | Narrows a 32-bit MIDI 2.0 value to a 14-bit
      | MIDI 1.0 value.
      |
      */
    pub fn scale_to14_with_u16(word_16bit: u16) -> u16 {
        
        todo!();
        /*
            return (uint16_t) (word16Bit >> 2);
        */
    }

    /**
      | Narrows a 32-bit MIDI 2.0 value to a 14-bit
      | MIDI 1.0 value.
      |
      */
    pub fn scale_to14_with_u32(word_32bit: u32) -> u16 {
        
        todo!();
        /*
            return (uint16_t) (word32Bit >> 18);
        */
    }

    /**
      | Converts UMP messages which may include
      | MIDI 2.0 channel voice messages into
      | equivalent MIDI 1.0 messages (still
      | in UMP format).
      | 
      | `callback` is a function that accepts
      | a single UniversalMidiPacketsView argument and will be called
      | with each converted packet.
      | 
      | Note that not all MIDI 2.0 messages have
      | MIDI 1.0 equivalents, so such messages
      | will be ignored.
      |
      */
    pub fn midi_2to_midi_1default_translation<Callback>(
        v:        &UniversalMidiPacketsView,
        callback: Callback)  {
    
        todo!();
        /*
            const auto firstWord = v[0];

            if (Utils::getMessageType (firstWord) != 0x4)
            {
                callback (v);
                return;
            }

            const auto status = Utils::getStatus (firstWord);
            const auto typeAndGroup = (uint8_t) ((0x2 << 0x4) | Utils::getGroup (firstWord));

            switch (status)
            {
                case 0x8:   // note off
                case 0x9:   // note on
                case 0xa:   // poly pressure
                case 0xb:   // control change
                {
                    const auto statusAndChannel = (uint8_t) ((firstWord >> 0x10) & 0xff);
                    const auto byte2 = (uint8_t) ((firstWord >> 0x08) & 0xff);
                    const auto byte3 = scaleTo7 (v[1]);

                    // If this is a note-on, and the scaled byte is 0,
                    // the scaled velocity should be 1 instead of 0
                    const auto needsCorrection = status == 0x9 && byte3 == 0;
                    const auto correctedByte = (uint8_t) (needsCorrection ? 1 : byte3);

                    const auto shouldIgnore = status == 0xb && [&]
                    {
                        switch (byte2)
                        {
                            case 0:
                            case 6:
                            case 32:
                            case 38:
                            case 98:
                            case 99:
                            case 100:
                            case 101:
                                return true;
                        }

                        return false;
                    }();

                    if (shouldIgnore)
                        return;

                    const PacketX1 packet { Utils::bytesToWord (typeAndGroup,
                                                                statusAndChannel,
                                                                byte2,
                                                                correctedByte) };
                    callback (UniversalMidiPacketsView (packet.data()));
                    return;
                }

                case 0xd: // channel pressure
                {
                    const auto statusAndChannel = (uint8_t) ((firstWord >> 0x10) & 0xff);
                    const auto byte2 = scaleTo7 (v[1]);

                    const PacketX1 packet { Utils::bytesToWord (typeAndGroup,
                                                                statusAndChannel,
                                                                byte2,
                                                                0) };
                    callback (UniversalMidiPacketsView (packet.data()));
                    return;
                }

                case 0x2:   // rpn
                case 0x3:   // nrpn
                {
                    const auto ccX = (uint8_t) (status == 0x2 ? 101 : 99);
                    const auto ccY = (uint8_t) (status == 0x2 ? 100 : 98);
                    const auto statusAndChannel = (uint8_t) ((0xb << 0x4) | Utils::getChannel (firstWord));
                    const auto data = scaleTo14 (v[1]);

                    const PacketX1 packets[]
                    {
                        PacketX1 { Utils::bytesToWord (typeAndGroup, statusAndChannel, ccX, (uint8_t) ((firstWord >> 0x8) & 0x7f)) },
                        PacketX1 { Utils::bytesToWord (typeAndGroup, statusAndChannel, ccY, (uint8_t) ((firstWord >> 0x0) & 0x7f)) },
                        PacketX1 { Utils::bytesToWord (typeAndGroup, statusAndChannel, 6,   (uint8_t) ((data >> 0x7) & 0x7f)) },
                        PacketX1 { Utils::bytesToWord (typeAndGroup, statusAndChannel, 38,  (uint8_t) ((data >> 0x0) & 0x7f)) },
                    };

                    for (const auto& packet : packets)
                        callback (UniversalMidiPacketsView (packet.data()));

                    return;
                }

                case 0xc: // program change / bank select
                {
                    if (firstWord & 1)
                    {
                        const auto statusAndChannel = (uint8_t) ((0xb << 0x4) | Utils::getChannel (firstWord));
                        const auto secondWord = v[1];

                        const PacketX1 packets[]
                        {
                            PacketX1 { Utils::bytesToWord (typeAndGroup, statusAndChannel, 0,  (uint8_t) ((secondWord >> 0x8) & 0x7f)) },
                            PacketX1 { Utils::bytesToWord (typeAndGroup, statusAndChannel, 32, (uint8_t) ((secondWord >> 0x0) & 0x7f)) },
                        };

                        for (const auto& packet : packets)
                            callback (UniversalMidiPacketsView (packet.data()));
                    }

                    const auto statusAndChannel = (uint8_t) ((0xc << 0x4) | Utils::getChannel (firstWord));
                    const PacketX1 packet { Utils::bytesToWord (typeAndGroup,
                                                                statusAndChannel,
                                                                (uint8_t) ((v[1] >> 0x18) & 0x7f),
                                                                0) };
                    callback (UniversalMidiPacketsView (packet.data()));
                    return;
                }

                case 0xe: // pitch bend
                {
                    const auto data = scaleTo14 (v[1]);
                    const auto statusAndChannel = (uint8_t) ((firstWord >> 0x10) & 0xff);
                    const PacketX1 packet { Utils::bytesToWord (typeAndGroup,
                                                                statusAndChannel,
                                                                (uint8_t) (data & 0x7f),
                                                                (uint8_t) ((data >> 7) & 0x7f)) };
                    callback (UniversalMidiPacketsView (packet.data()));
                    return;
                }

                default: // other message types do not translate
                    return;
            }
        */
    }
}
