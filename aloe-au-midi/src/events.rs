crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUMIDIBase.cpp]

pub const kMidiMessage_NoteOff:                usize = 0x80;
pub const kMidiMessage_NoteOn:                 usize = 0x90;
pub const kMidiMessage_PolyPressure:           usize = 0xA0;
pub const kMidiMessage_ControlChange:          usize = 0xB0;
pub const kMidiMessage_ProgramChange:          usize = 0xC0;
pub const kMidiMessage_ChannelPressure:        usize = 0xD0;
pub const kMidiMessage_PitchWheel:             usize = 0xE0;
pub const kMidiController_AllSoundOff:         usize = 120;
pub const kMidiController_ResetAllControllers: usize = 121;
pub const kMidiController_AllNotesOff:         usize = 123;

#[inline] pub fn next_midi_event(
        event: *const u8,
        end:   *const u8) -> *const u8 {
    
    todo!();
        /*
            Byte c = *event;
        switch (c >> 4) {
        default:    // data byte -- assume in sysex
            while ((*++event & 0x80) == 0 && event < end)
                ;
            break;
        case 0x8:
        case 0x9:
        case 0xA:
        case 0xB:
        case 0xE:
            event += 3;
            break;
        case 0xC:
        case 0xD:
            event += 2;
            break;
        case 0xF:
            switch (c) {
            case 0xF0:
                while ((*++event & 0x80) == 0 && event < end)
                    ;
                break;
            case 0xF1:
            case 0xF3:
                event += 2;
                break;
            case 0xF2:
                event += 3;
                break;
            default:
                ++event;
                break;
            }
        }
        return (event >= end) ? end : event;
        */
}

