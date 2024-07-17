crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPFactory.h]

#[repr(u8)]
pub enum UniversalMidiPacketsFactoryNoteAttributeKind
{
    none            = 0x00,
    manufacturer    = 0x01,
    profile         = 0x02,
    pitch7_9        = 0x03
}

/**
  | This struct holds functions that can
  | be used to create different kinds of
  | Universal MIDI Packet.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacketsFactory {

}

impl UniversalMidiPacketsFactory {

    pub fn make_system<'a>() -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return UniversalMidiPacketX1{}.withMessageType (1);
        */
    }
    
    pub fn makev1<'a>() -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return UniversalMidiPacketX1{}.withMessageType (2);
        */
    }
    
    pub fn makev2<'a>() -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return UniversalMidiPacketX2{}.withMessageType (4);
        */
    }
    
    pub fn make_sys_ex<'a>(
        group:     u8,
        status:    u8,
        num_bytes: u8,
        data:      *const u8) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            jassert (numBytes <= 6);

                std::array<uint8_t, 8> bytes{{}};
                bytes[0] = (0x3 << 0x4) | group;
                bytes[1] = (uint8_t) (status << 0x4) | numBytes;

                std::memcpy (bytes.data() + 2, data, numBytes);

                std::array<uint32_t, 2> words;

                size_t index = 0;

                for (auto& word : words)
                    word = ByteOrder::bigEndianInt (bytes.data() + 4 * index++);

                return UniversalMidiPacketX2 { words };
        */
    }
    
    pub fn make_sys_ex8<'a>(
        group:      u8,
        status:     u8,
        num_bytes:  u8,
        data_start: u8,
        data:       *const u8) -> UniversalMidiPacketX4<'a> {
        
        todo!();
        /*
            jassert (numBytes <= 16 - dataStart);

                std::array<uint8_t, 16> bytes{{}};
                bytes[0] = (0x5 << 0x4) | group;
                bytes[1] = (uint8_t) (status << 0x4) | numBytes;

                std::memcpy (bytes.data() + dataStart, data, numBytes);

                std::array<uint32_t, 4> words;

                size_t index = 0;

                for (auto& word : words)
                    word = ByteOrder::bigEndianInt (bytes.data() + 4 * index++);

                return UniversalMidiPacketX4 { words };
        */
    }


    pub fn make_noop<'a>(group: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return UniversalMidiPacketX1{}.withGroup (group);
        */
    }
    
    pub fn make_jr_clock<'a>(
        group: u8,
        time:  u16) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return UniversalMidiPacketX1 { time }.withStatus (1).withGroup (group);
        */
    }
    
    pub fn make_jr_timestamp<'a>(
        group: u8,
        time:  u16) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return UniversalMidiPacketX1 { time }.withStatus (2).withGroup (group);
        */
    }
    
    pub fn make_time_code<'a>(
        group: u8,
        code:  u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeSystem().withGroup (group)
                                       .withU8<1> (0xf1)
                                       .withU8<2> (code & 0x7f);
        */
    }
    
    pub fn make_song_position_pointer<'a>(
        group: u8,
        pos:   u16) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeSystem().withGroup (group)
                                       .withU8<1> (0xf2)
                                       .withU8<2> (pos & 0x7f)
                                       .withU8<3> ((pos >> 7) & 0x7f);
        */
    }
    
    pub fn make_song_select<'a>(
        group: u8,
        song:  u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeSystem().withGroup (group)
                                       .withU8<1> (0xf3)
                                       .withU8<2> (song & 0x7f);
        */
    }
    
    pub fn make_tune_request<'a>(group: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeSystem().withGroup (group)
                                       .withU8<1> (0xf6);
        */
    }
    
    pub fn make_timing_clock<'a>(group: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeSystem().withGroup (group)
                                       .withU8<1> (0xf8);
        */
    }
    
    pub fn make_start<'a>(group: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeSystem().withGroup (group)
                                       .withU8<1> (0xfa);
        */
    }
    
    pub fn make_continue<'a>(group: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeSystem().withGroup (group)
                                       .withU8<1> (0xfb);
        */
    }
    
    pub fn make_stop<'a>(group: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeSystem().withGroup (group)
                                       .withU8<1> (0xfc);
        */
    }
    
    pub fn make_active_sensing<'a>(group: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeSystem().withGroup (group)
                                       .withU8<1> (0xfe);
        */
    }
    
    pub fn make_reset<'a>(group: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeSystem().withGroup (group)
                                       .withU8<1> (0xff);
        */
    }
    
    pub fn make_note_offv1<'a>(
        group:    u8,
        channel:  u8,
        note:     u8,
        velocity: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeV1().withGroup (group)
                                   .withStatus (0x8)
                                   .withChannel (channel)
                                   .withU8<2> (note & 0x7f)
                                   .withU8<3> (velocity & 0x7f);
        */
    }
    
    pub fn make_note_onv1<'a>(
        group:    u8,
        channel:  u8,
        note:     u8,
        velocity: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeV1().withGroup (group)
                                   .withStatus (0x9)
                                   .withChannel (channel)
                                   .withU8<2> (note & 0x7f)
                                   .withU8<3> (velocity & 0x7f);
        */
    }
    
    pub fn make_poly_pressurev1<'a>(
        group:    u8,
        channel:  u8,
        note:     u8,
        pressure: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeV1().withGroup (group)
                                   .withStatus (0xa)
                                   .withChannel (channel)
                                   .withU8<2> (note & 0x7f)
                                   .withU8<3> (pressure & 0x7f);
        */
    }
    
    pub fn make_control_changev1<'a>(
        group:      u8,
        channel:    u8,
        controller: u8,
        value:      u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeV1().withGroup (group)
                                   .withStatus (0xb)
                                   .withChannel (channel)
                                   .withU8<2> (controller & 0x7f)
                                   .withU8<3> (value & 0x7f);
        */
    }
    
    pub fn make_program_changev1<'a>(
        group:   u8,
        channel: u8,
        program: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeV1().withGroup (group)
                                   .withStatus (0xc)
                                   .withChannel (channel)
                                   .withU8<2> (program & 0x7f);
        */
    }
    
    pub fn make_channel_pressurev1<'a>(
        group:    u8,
        channel:  u8,
        pressure: u8) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeV1().withGroup (group)
                                   .withStatus (0xd)
                                   .withChannel (channel)
                                   .withU8<2> (pressure & 0x7f);
        */
    }
    
    pub fn make_pitch_bend<'a>(
        group:     u8,
        channel:   u8,
        pitchbend: u16) -> UniversalMidiPacketX1<'a> {
        
        todo!();
        /*
            return Detail::makeV1().withGroup (group)
                                   .withStatus (0xe)
                                   .withChannel (channel)
                                   .withU8<2> (pitchbend & 0x7f)
                                   .withU8<3> ((pitchbend >> 7) & 0x7f);
        */
    }
    
    pub fn make_sys_ex_in_1packet<'a>(
        group:     u8,
        num_bytes: u8,
        data:      *const u8) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeSysEx (group, 0x0, numBytes, data);
        */
    }
    
    pub fn make_sys_ex_start<'a>(
        group:     u8,
        num_bytes: u8,
        data:      *const u8) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeSysEx (group, 0x1, numBytes, data);
        */
    }
    
    pub fn make_sys_ex_continue<'a>(
        group:     u8,
        num_bytes: u8,
        data:      *const u8) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeSysEx (group, 0x2, numBytes, data);
        */
    }
    
    pub fn make_sys_ex_end<'a>(
        group:     u8,
        num_bytes: u8,
        data:      *const u8) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeSysEx (group, 0x3, numBytes, data);
        */
    }
    
    pub fn make_registered_per_note_controllerv2<'a>(
        group:      u8,
        channel:    u8,
        note:       u8,
        controller: u8,
        data:       u32) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0x0)
                                   .withChannel (channel)
                                   .withU8<2> (note & 0x7f)
                                   .withU8<3> (controller & 0x7f)
                                   .withU32<1> (data);
        */
    }
    
    pub fn make_assignable_per_note_controllerv2<'a>(
        group:      u8,
        channel:    u8,
        note:       u8,
        controller: u8,
        data:       u32) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0x1)
                                   .withChannel (channel)
                                   .withU8<2> (note & 0x7f)
                                   .withU8<3> (controller & 0x7f)
                                   .withU32<1> (data);
        */
    }
    
    pub fn make_registered_controllerv2<'a>(
        group:   u8,
        channel: u8,
        bank:    u8,
        index:   u8,
        data:    u32) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0x2)
                                   .withChannel (channel)
                                   .withU8<2> (bank & 0x7f)
                                   .withU8<3> (index & 0x7f)
                                   .withU32<1> (data);
        */
    }
    
    pub fn make_assignable_controllerv2<'a>(
        group:   u8,
        channel: u8,
        bank:    u8,
        index:   u8,
        data:    u32) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0x3)
                                   .withChannel (channel)
                                   .withU8<2> (bank & 0x7f)
                                   .withU8<3> (index & 0x7f)
                                   .withU32<1> (data);
        */
    }
    
    pub fn make_relative_registered_controllerv2<'a>(
        group:   u8,
        channel: u8,
        bank:    u8,
        index:   u8,
        data:    u32) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0x4)
                                   .withChannel (channel)
                                   .withU8<2> (bank & 0x7f)
                                   .withU8<3> (index & 0x7f)
                                   .withU32<1> (data);
        */
    }
    
    pub fn make_relative_assignable_controllerv2<'a>(
        group:   u8,
        channel: u8,
        bank:    u8,
        index:   u8,
        data:    u32) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0x5)
                                   .withChannel (channel)
                                   .withU8<2> (bank & 0x7f)
                                   .withU8<3> (index & 0x7f)
                                   .withU32<1> (data);
        */
    }
    
    pub fn make_per_note_pitch_bendv2<'a>(
        group:   u8,
        channel: u8,
        note:    u8,
        data:    u32) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0x6)
                                   .withChannel (channel)
                                   .withU8<2> (note & 0x7f)
                                   .withU32<1> (data);
        */
    }
    
    pub fn make_note_offv2<'a>(
        group:           u8,
        channel:         u8,
        note:            u8,
        attribute:       UniversalMidiPacketsFactoryNoteAttributeKind,
        velocity:        u16,
        attribute_value: u16) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0x8)
                                   .withChannel (channel)
                                   .withU8<2> (note & 0x7f)
                                   .withU8<3> ((uint8_t) attribute)
                                   .withU16<2> (velocity)
                                   .withU16<3> (attributeValue);
        */
    }
    
    pub fn make_note_onv2<'a>(
        group:           u8,
        channel:         u8,
        note:            u8,
        attribute:       UniversalMidiPacketsFactoryNoteAttributeKind,
        velocity:        u16,
        attribute_value: u16) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0x9)
                                   .withChannel (channel)
                                   .withU8<2> (note & 0x7f)
                                   .withU8<3> ((uint8_t) attribute)
                                   .withU16<2> (velocity)
                                   .withU16<3> (attributeValue);
        */
    }
    
    pub fn make_poly_pressurev2<'a>(
        group:   u8,
        channel: u8,
        note:    u8,
        data:    u32) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0xa)
                                   .withChannel (channel)
                                   .withU8<2> (note & 0x7f)
                                   .withU32<1> (data);
        */
    }
    
    pub fn make_control_changev2<'a>(
        group:      u8,
        channel:    u8,
        controller: u8,
        data:       u32) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0xb)
                                   .withChannel (channel)
                                   .withU8<2> (controller & 0x7f)
                                   .withU32<1> (data);
        */
    }
    
    pub fn make_program_changev2<'a>(
        group:        u8,
        channel:      u8,
        option_flags: u8,
        program:      u8,
        bank_msb:     u8,
        bank_lsb:     u8) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0xc)
                                   .withChannel (channel)
                                   .withU8<3> (optionFlags)
                                   .withU8<4> (program)
                                   .withU8<6> (bankMsb)
                                   .withU8<7> (bankLsb);
        */
    }
    
    pub fn make_channel_pressurev2<'a>(
        group:   u8,
        channel: u8,
        data:    u32) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0xd)
                                   .withChannel (channel)
                                   .withU32<1> (data);
        */
    }
    
    pub fn make_pitch_bendv2<'a>(
        group:   u8,
        channel: u8,
        data:    u32) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0xe)
                                   .withChannel (channel)
                                   .withU32<1> (data);
        */
    }
    
    pub fn make_per_note_managementv2<'a>(
        group:        u8,
        channel:      u8,
        note:         u8,
        option_flags: u8) -> UniversalMidiPacketX2<'a> {
        
        todo!();
        /*
            return Detail::makeV2().withGroup (group)
                                   .withStatus (0xf)
                                   .withChannel (channel)
                                   .withU8<2> (note)
                                   .withU8<3> (optionFlags);
        */
    }
    
    pub fn make_sys_ex8in_1packet<'a>(
        group:     u8,
        num_bytes: u8,
        stream_id: u8,
        data:      *const u8) -> UniversalMidiPacketX4<'a> {
        
        todo!();
        /*
            return Detail::makeSysEx8 (group, 0x0, numBytes, 3, data).withU8<2> (streamId);
        */
    }
    
    pub fn make_sys_ex_8start<'a>(
        group:     u8,
        num_bytes: u8,
        stream_id: u8,
        data:      *const u8) -> UniversalMidiPacketX4<'a> {
        
        todo!();
        /*
            return Detail::makeSysEx8 (group, 0x1, numBytes, 3, data).withU8<2> (streamId);
        */
    }
    
    pub fn make_sys_ex_8continue<'a>(
        group:     u8,
        num_bytes: u8,
        stream_id: u8,
        data:      *const u8) -> UniversalMidiPacketX4<'a> {
        
        todo!();
        /*
            return Detail::makeSysEx8 (group, 0x2, numBytes, 3, data).withU8<2> (streamId);
        */
    }
    
    pub fn make_sys_ex_8end<'a>(
        group:     u8,
        num_bytes: u8,
        stream_id: u8,
        data:      *const u8) -> UniversalMidiPacketX4<'a> {
        
        todo!();
        /*
            return Detail::makeSysEx8 (group, 0x3, numBytes, 3, data).withU8<2> (streamId);
        */
    }
    
    pub fn make_mixed_data_set_header<'a>(
        group:       u8,
        data_set_id: u8,
        data:        *const u8) -> UniversalMidiPacketX4<'a> {
        
        todo!();
        /*
            return Detail::makeSysEx8 (group, 0x8, 14, 2, data).withChannel (dataSetId);
        */
    }
    
    pub fn make_data_set_payload<'a>(
        group:       u8,
        data_set_id: u8,
        data:        *const u8) -> UniversalMidiPacketX4<'a> {
        
        todo!();
        /*
            return Detail::makeSysEx8 (group, 0x9, 14, 2, data).withChannel (dataSetId);
        */
    }
}
