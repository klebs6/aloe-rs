crate::ix!();

/**
  | Generates an appropriate sequence
  | of MIDI CC messages to represent an RPN
  | or NRPN message.
  | 
  | This sequence (as a MidiBuffer) can
  | then be directly sent to a MidiOutput.
  | 
  | @tags{Audio}
  |
  */
pub struct MidiRPNGenerator {

}

impl MidiRPNGenerator {

    /**
      | Generates a MIDI sequence representing
      | the given RPN or NRPN message.
      |
      */
    pub fn generate_midi_rpn_message(&mut self, message: MidiRPNMessage) -> MidiBuffer {
        
        todo!();
        /*
            return generate (message.channel,
                         message.parameterNumber,
                         message.value,
                         message.isNRPN,
                         message.is14BitValue);
        */
    }
    
    /**
      | Generates a MIDI sequence representing
      | an RPN or NRPN message with the given
      | parameters.
      | 
      | -----------
      | @param channel
      | 
      | The MIDI channel of the RPN/NRPN message.
      | ----------
      | @param parameterNumber
      | 
      | The parameter number, in the range 0
      | to 16383.
      | ----------
      | @param value
      | 
      | The parameter value, in the range 0 to
      | 16383, orin the range 0 to 127 if sendAs14BitValue
      | is false.
      | ----------
      | @param isNRPN
      | 
      | Whether you need a MIDI RPN or NRPN sequence
      | (RPN is default).
      | ----------
      | @param use14BitValue
      | 
      | If true (default), the value will have
      | 14-bit precision(two MIDI bytes).
      | If false, instead the value will have7-bit
      | precision (a single MIDI byte).
      |
      */
    pub fn generate(
        &mut self, 
        midi_channel:     i32,
        parameter_number: i32,
        value:            i32,
        isnrpn:           Option<bool>,
        use_14bit_value:  Option<bool>

    ) -> MidiBuffer {
        
        let isnrpn          = isnrpn.unwrap_or(false);
        let use_14bit_value = use_14bit_value.unwrap_or(true);

        todo!();
        /*
            jassert (midiChannel > 0 && midiChannel <= 16);
        jassert (parameterNumber >= 0 && parameterNumber < 16384);
        jassert (value >= 0 && value < (use14BitValue ? 16384 : 128));

        uint8 parameterLSB = uint8 (parameterNumber & 0x0000007f);
        uint8 parameterMSB = uint8 (parameterNumber >> 7);

        uint8 valueLSB = use14BitValue ? uint8 (value & 0x0000007f) : 0x00;
        uint8 valueMSB = use14BitValue ? uint8 (value >> 7) : uint8 (value);

        uint8 channelByte = uint8 (0xb0 + midiChannel - 1);

        MidiBuffer buffer;

        buffer.addEvent (MidiMessage (channelByte, isNRPN ? 0x62 : 0x64, parameterLSB),  0);
        buffer.addEvent (MidiMessage (channelByte, isNRPN ? 0x63 : 0x65, parameterMSB),  0);

        // sending the value LSB is optional, but must come before sending the value MSB:
        if (use14BitValue)
            buffer.addEvent (MidiMessage (channelByte, 0x26, valueLSB), 0);

        buffer.addEvent (MidiMessage (channelByte, 0x06, valueMSB), 0);

        return buffer;
        */
    }
}
