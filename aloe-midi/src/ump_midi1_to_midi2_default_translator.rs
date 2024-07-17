crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPMidi1ToMidi2DefaultTranslator.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPMidi1ToMidi2DefaultTranslator.cpp]

/**
  | Translates a series of MIDI 1 Universal
  | MIDI Packets to corresponding MIDI
  | 2 packets.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsMidi1ToMidi2DefaultTranslator {
    group_accumulators: [UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorChannelAccumulators; 16],
    group_banks:        [UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorChannelBanks; 16],
}

impl Default for UniversalMidiPacketsMidi1ToMidi2DefaultTranslator {

    fn default() -> Self {
        todo!();
    }
}

impl UniversalMidiPacketsMidi1ToMidi2DefaultTranslator {

    /**
      | Converts MIDI 1 Universal MIDI Packets
      | to corresponding MIDI 2 packets, calling
      | `callback` with each converted packet.
      | 
      | In some cases (such as RPN/NRPN messages)
      | multiple MIDI 1 packets will convert
      | to a single MIDI 2 packet. In these cases,
      | the translator will accumulate the
      | full message internally, and send a
      | single callback with the completed
      | message, once all the individual MIDI
      | 1 packets have been processed.
      |
      */
    pub fn dispatch<PacketCallback>(&mut self, 
        v:        &UniversalMidiPacketsView,
        callback: PacketCallback)  {
    
        todo!();
        /*
            const auto firstWord = v[0];
            const auto messageType = Utils::getMessageType (firstWord);

            if (messageType != 0x2)
            {
                callback (v);
                return;
            }

            const UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorHelperValues helperValues
            {
                (uint8_t) ((0x4 << 0x4) | Utils::getGroup (firstWord)),
                (uint8_t) ((firstWord >> 0x10) & 0xff),
                (uint8_t) ((firstWord >> 0x08) & 0x7f),
                (uint8_t) ((firstWord >> 0x00) & 0x7f),
            };

            switch (Utils::getStatus (firstWord))
            {
                case 0x8:
                case 0x9:
                {
                    const auto packet = processNoteOnOrOff (helperValues);
                    callback (UniversalMidiPacketsView (packet.data()));
                    return;
                }

                case 0xa:
                {
                    const auto packet = processPolyPressure (helperValues);
                    callback (UniversalMidiPacketsView (packet.data()));
                    return;
                }

                case 0xb:
                {
                    UniversalMidiPacketX2 packet;

                    if (processControlChange (helperValues, packet))
                        callback (UniversalMidiPacketsView (packet.data()));

                    return;
                }

                case 0xc:
                {
                    const auto packet = processProgramChange (helperValues);
                    callback (UniversalMidiPacketsView (packet.data()));
                    return;
                }

                case 0xd:
                {
                    const auto packet = processChannelPressure (helperValues);
                    callback (UniversalMidiPacketsView (packet.data()));
                    return;
                }

                case 0xe:
                {
                    const auto packet = processPitchBend (helperValues);
                    callback (UniversalMidiPacketsView (packet.data()));
                    return;
                }
            }
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            groupAccumulators = {};
            groupBanks = {};
        */
    }
    
    pub fn process_note_on_or_off(&mut self, helpers: UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorHelperValues) -> UniversalMidiPacketX2 {
        
        todo!();
        /*
            const auto velocity = helpers.byte2;
        const auto needsConversion = (helpers.byte0 >> 0x4) == 0x9 && velocity == 0;
        const auto firstByte = needsConversion ? (uint8_t) ((0x8 << 0x4) | (helpers.byte0 & 0xf))
                                               : helpers.byte0;

        return UniversalMidiPacketX2
        {
            Utils::bytesToWord (helpers.typeAndGroup, firstByte, helpers.byte1, 0),
            (uint32_t) (Conversion::scaleTo16 (velocity) << 0x10)
        };
        */
    }
    
    pub fn process_poly_pressure(&mut self, helpers: UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorHelperValues) -> UniversalMidiPacketX2 {
        
        todo!();
        /*
            return UniversalMidiPacketX2
        {
            Utils::bytesToWord (helpers.typeAndGroup, helpers.byte0, helpers.byte1, 0),
            Conversion::scaleTo32 (helpers.byte2)
        };
        */
    }
    
    pub fn process_control_change(&mut self, 
        helpers: UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorHelperValues,
        packet:  &mut UniversalMidiPacketX2) -> bool {
        
        todo!();
        /*
            const auto statusAndChannel = helpers.byte0;
        const auto cc               = helpers.byte1;

        const auto shouldAccumulate = [&]
        {
            switch (cc)
            {
                case 6:
                case 38:
                case 98:
                case 99:
                case 100:
                case 101:
                    return true;
            }

            return false;
        }();

        const auto group   = (uint8_t) (helpers.typeAndGroup & 0xf);
        const auto channel = (uint8_t) (statusAndChannel & 0xf);
        const auto byte    = helpers.byte2;

        if (shouldAccumulate)
        {
            auto& accumulator = groupAccumulators[group][channel];

            if (accumulator.addByte (cc, byte))
            {
                const auto& bytes = accumulator.getBytes();
                const auto bank   = bytes[0];
                const auto index  = bytes[1];
                const auto msb    = bytes[2];
                const auto lsb    = bytes[3];

                const auto value = (uint16_t) (((msb & 0x7f) << 7) | (lsb & 0x7f));

                const auto newStatus = (uint8_t) (accumulator.getKind() == UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorPnKind::nrpn ? 0x3 : 0x2);

                packet = UniversalMidiPacketX2
                {
                    Utils::bytesToWord (helpers.typeAndGroup, (uint8_t) ((newStatus << 0x4) | channel), bank, index),
                    Conversion::scaleTo32 (value)
                };
                return true;
            }

            return false;
        }

        if (cc == 0)
        {
            groupBanks[group][channel].setMsb (byte);
            return false;
        }

        if (cc == 32)
        {
            groupBanks[group][channel].setLsb (byte);
            return false;
        }

        packet = UniversalMidiPacketX2
        {
            Utils::bytesToWord (helpers.typeAndGroup, statusAndChannel, cc, 0),
            Conversion::scaleTo32 (helpers.byte2)
        };
        return true;
        */
    }
    
    pub fn process_program_change(&self, helpers: UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorHelperValues) -> UniversalMidiPacketX2 {
        
        todo!();
        /*
            const auto group   = (uint8_t) (helpers.typeAndGroup & 0xf);
        const auto channel = (uint8_t) (helpers.byte0 & 0xf);
        const auto bank    = groupBanks[group][channel];
        const auto valid   = bank.isValid();

        return UniversalMidiPacketX2
        {
            Utils::bytesToWord (helpers.typeAndGroup, helpers.byte0, 0, valid ? 1 : 0),
            Utils::bytesToWord (helpers.byte1, 0, valid ? bank.getMsb() : 0, valid ? bank.getLsb() : 0)
        };
        */
    }
    
    pub fn process_channel_pressure(&mut self, helpers: UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorHelperValues) -> UniversalMidiPacketX2 {
        
        todo!();
        /*
            return UniversalMidiPacketX2
        {
            Utils::bytesToWord (helpers.typeAndGroup, helpers.byte0, 0, 0),
            Conversion::scaleTo32 (helpers.byte1)
        };
        */
    }
    
    pub fn process_pitch_bend(&mut self, helpers: UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorHelperValues) -> UniversalMidiPacketX2 {
        
        todo!();
        /*
            const auto lsb   = helpers.byte1;
        const auto msb   = helpers.byte2;
        const auto value = (uint16_t) (msb << 7 | lsb);

        return UniversalMidiPacketX2
        {
            Utils::bytesToWord (helpers.typeAndGroup, helpers.byte0, 0, 0),
            Conversion::scaleTo32 (value)
        };
        */
    }
}
