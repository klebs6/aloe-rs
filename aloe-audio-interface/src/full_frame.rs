crate::ix!();

pub trait IsFullFrame {

    /**
      | Returns true if this is a full-frame
      | midi timecode message.
      |
      */
    fn is_full_frame(&self) -> bool;
}

pub trait GetFullFrameParameters {

    /** 
      | Extracts the timecode information from
      | a full-frame midi timecode message.
      |
      | You should only call this on messages
      | where you've used isFullFrame() to check
      | that they're the right kind.
      */
    fn get_full_frame_parameters(
        &self, 
        hours:         &mut i32,
        minutes:       &mut i32,
        seconds:       &mut i32,
        frames:        &mut i32,
        timecode_type: &mut MidiMessageSmpteTimecodeType
    );
}

pub trait FullFrame {

    /**
      | Creates a full-frame MTC message.
      |
      */
    fn full_frame(
        &mut self, 
        hours:         i32,
        minutes:       i32,
        seconds:       i32,
        frames:        i32,
        timecode_type: MidiMessageSmpteTimecodeType

    ) -> dyn MidiMessageInterface;
}
