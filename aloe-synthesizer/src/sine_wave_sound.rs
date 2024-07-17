crate::ix!();

/**
  | Our demo synth sound is just a basic sine
  | wave..
  |
  */
#[derive(Default)]
pub struct SineWaveSound { }

impl SynthesizerSound for SineWaveSound {

    fn applies_to_note(&mut self, midi_note_number: i32) -> bool { true }

    fn applies_to_channel(&mut self, midi_channel: i32)  -> bool { true }
}
