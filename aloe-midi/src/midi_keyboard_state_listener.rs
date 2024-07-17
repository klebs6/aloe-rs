crate::ix!();

/**
  | Receives events from a MidiKeyboardState
  | object.
  |
  */
pub trait MidiKeyboardStateListener {

    /** 
      | Called when one of the
      | MidiKeyboardState's keys is pressed.
      |
      | This will be called synchronously when
      | the state is either processing
      | a buffer in its
      | MidiKeyboardState::processNextMidiBuffer()
      | method, or when a note is being played
      | with its MidiKeyboardState::noteOn()
      | method.
      |
      | Note that this callback could happen
      | from an audio callback thread, so be
      | careful not to block, and avoid any UI
      | activity in the callback.
      */
    fn handle_note_on(&mut self, 
        source:           *mut MidiKeyboardState,
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32);

    /** 
      | Called when one of the
      | MidiKeyboardState's keys is released.
      |
      | This will be called synchronously when
      | the state is either processing
      | a buffer in its
      | MidiKeyboardState::processNextMidiBuffer()
      | method, or when a note is being played
      | with its MidiKeyboardState::noteOff()
      | method.
      |
      | Note that this callback could happen
      | from an audio callback thread, so be
      | careful not to block, and avoid any UI
      | activity in the callback.
      */
    fn handle_note_off(&mut self, 
        source:           *mut MidiKeyboardState,
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32);
}
