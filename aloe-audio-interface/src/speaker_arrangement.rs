crate::ix!();

pub trait GetSpeakerArrangementAsString {

    /**
      | Returns a string containing a whitespace-separated
      | list of speaker types corresponding to each channel.
      | For example in a 5.1 arrangement, the string may
      | be "L R C Lfe Ls Rs". If the speaker arrangement
      | is unknown, the returned string will be empty.
      */
    fn get_speaker_arrangement_as_string(&self) -> String;
}
