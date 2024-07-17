crate::ix!();
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPProtocols.h]

/**
  | The kinds of MIDI protocol that can be
  | formatted into Universal MIDI Packets.
  |
  */
pub enum UniversalMidiPacketsPacketProtocol
{
    MIDI_1_0,
    MIDI_2_0,
}

/**
  | All kinds of MIDI protocol understood
  | by Aloe.
  |
  */
pub enum UniversalMidiPacketsMidiProtocol
{
    bytestream,
    UMP_MIDI_1_0,
    UMP_MIDI_2_0,
}
