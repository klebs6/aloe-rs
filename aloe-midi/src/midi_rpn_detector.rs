crate::ix!();

/**
  | Parses a stream of MIDI data to assemble RPN
  | and NRPN messages from their constituent MIDI
  | CC messages.
  |
  | The detector uses the following parsing rules:
  | the parameter number LSB/MSB can be
  | sent/received in either order and must both
  | come before the parameter value; for the
  | parameter value, LSB always has to be
  | sent/received before the value MSB, otherwise
  | it will be treated as 7-bit (MSB only).
  |
  | @tags{Audio}
  */
#[leak_detector]
#[derive(Default)]
pub struct MidiRPNDetector {
    states: [MidiRPNDetectorChannelState; 16],
}

impl MidiRPNDetector {

    /**
      | Takes the next in a stream of incoming
      | MIDI CC messages and returns true if
      | it forms the last of a sequence that makes
      | an RPN or NPRN.
      | 
      | If this returns true, then the RPNMessage
      | object supplied will be filled-out
      | with the message's details. (If it returns
      | false then the RPNMessage object will
      | be unchanged).
      |
      */
    pub fn parse_controller_message(&mut self, 
        midi_channel:      i32,
        controller_number: i32,
        controller_value:  i32,
        result:            &mut MidiRPNMessage) -> bool {
        
        todo!();
        /*
            jassert (midiChannel > 0 && midiChannel <= 16);
        jassert (controllerNumber >= 0 && controllerNumber < 128);
        jassert (controllerValue >= 0 && controllerValue < 128);

        return states[midiChannel - 1].handleController (midiChannel, controllerNumber, controllerValue, result);
        */
    }
    
    /**
      | Resets the RPN detector's internal
      | state, so that it forgets about previously
      | received MIDI CC messages.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < 16; ++i)
        {
            states[i].parameterMSB = 0xff;
            states[i].parameterLSB = 0xff;
            states[i].resetValue();
            states[i].isNRPN = false;
        }
        */
    }
}
