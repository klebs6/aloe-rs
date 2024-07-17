crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/AudioPluginDemo.h]

/**
  | A demo synth sound that's just a basic
  | sine wave..
  |
  */
#[derive(Default)]
pub struct SineWaveSound {

}

impl SynthesizerSound for SineWaveSound {

    fn applies_to_note(&mut self, midi_note_number: i32) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    fn applies_to_channel(&mut self, midi_channel: i32) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
