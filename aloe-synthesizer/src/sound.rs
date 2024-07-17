crate::ix!();

/**
  | Describes one of the sounds that a Synthesizer
  | can play.
  | 
  | A synthesiser can contain one or more
  | sounds, and a sound can choose which
  | midi notes and channels can trigger
  | it.
  | 
  | The SynthesizerSound is a passive class
  | that just describes what the sound is
  | - the actual audio rendering for a sound
  | is done by a SynthesizerVoice. This
  | allows more than one SynthesizerVoice
  | to play the same sound at the same time.
  | 
  | @see Synthesizer, SynthesizerVoice
  | 
  | @tags{Audio}
  |
  */
#[leak_detector]
pub trait SynthesizerSound {
    
    /**
      | Returns true if this sound should be
      | played when a given midi note is pressed.
      | 
      | The Synthesizer will use this information
      | when deciding which sounds to trigger
      | for a given note.
      |
      */
    fn applies_to_note(&mut self, midi_note_number: i32) -> bool;

    /**
      | Returns true if the sound should be triggered
      | by midi events on a given channel.
      | 
      | The Synthesizer will use this information
      | when deciding which sounds to trigger
      | for a given note.
      |
      */
    fn applies_to_channel(&mut self, midi_channel: i32) -> bool;
}

/**
  | The class is reference-counted, so
  | this is a handy pointer class for it.
  |
  */
pub type SynthesizerSoundPtr = Rc<RefCell<dyn SynthesizerSound>>;
