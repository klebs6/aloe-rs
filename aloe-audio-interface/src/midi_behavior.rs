crate::ix!();

pub trait CheckAcceptsMidi {

    /**
      | Returns true if the processor wants
      | MIDI messages.
      |
      */
    fn accepts_midi(&self) -> bool;
}

pub trait CheckProducesMidi {

    /**
      | Returns true if the processor produces
      | MIDI messages.
      |
      */
    fn produces_midi(&self) -> bool;
}

pub trait CheckSupportsMpe {

    /**
      | Returns true if the processor supports
      | MPE.
      |
      */
    fn supportsmpe(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

pub trait CheckIsMidiEffect {

    /**
      | Returns true if this is a MIDI effect
      | plug-in and does no audio processing.
      |
      */
    fn is_midi_effect(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
