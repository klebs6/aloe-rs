crate::ix!();

pub const NUM_MIDI_CHANNELS: usize = 16;

pub struct AloeVst3EditControllerMidiController
{
    channel:     i32, // default = -1
    ctrl_number: i32, // default = -1
}

