crate::ix!();

/** 
  | SMPTE timecode types.
  |
  | Used by the getFullFrameParameters() and
  | fullFrame() methods.
  */
pub enum MidiMessageSmpteTimecodeType
{
    fps24       = 0,
    fps25       = 1,
    fps30drop   = 2,
    fps30       = 3
}
