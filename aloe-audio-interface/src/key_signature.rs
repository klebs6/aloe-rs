crate::ix!();

pub trait IsKeySignatureMetaEvent {

    /**
      | Returns true if this is a 'key-signature'
      | meta-event. @see getKeySignatureNumberOfSharpsOrFlats,
      | isKeySignatureMajorKey
      |
      */
    fn is_key_signature_meta_event(&self) -> bool;
}

pub trait GetKeySignatureNumberOfSharpsOrFlats {

    /** 
      | Returns the key from a key-signature
      | meta-event.
      |
      | This method must only be called if
      | isKeySignatureMetaEvent() is true.
      |
      | A positive number here indicates the
      | number of sharps in the key signature, and
      | a negative number indicates a number of
      | flats. So e.g. 3 = F# + C# + G#, -2 = Bb
      | + Eb
      |
      | @see isKeySignatureMetaEvent,
      | isKeySignatureMajorKey
      */
    fn get_key_signature_number_of_sharps_or_flats(&self) -> i32;
}

pub trait IsKeySignatureMajorKey {

    /**
      | Returns true if this key-signature
      | event is major, or false if it's minor.
      | This method must only be called if isKeySignatureMetaEvent()
      | is true.
      |
      */
    fn is_key_signature_major_key(&self) -> bool;
}

pub trait KeySignatureMetaEvent {

    /**
      | Creates a key-signature meta-event.
      | 
      | -----------
      | @param numberOfSharpsOrFlats
      | 
      | if positive, this indicates the number
      | of sharps in the key; if negative, the
      | number of flats
      | ----------
      | @param isMinorKey
      | 
      | if true, the key is minor; if false, it
      | is major
      | 
      | @see isKeySignatureMetaEvent
      |
      */
    fn key_signature_meta_event(
        &mut self, 
        number_of_sharps_or_flats: i32,
        is_minor_key:              bool
    ) -> dyn MidiMessageInterface;
}
