crate::ix!();

/**
  | Receives incoming messages from a physical
  | MIDI input device.
  | 
  | This class is overridden to handle incoming
  | midi messages. See the MidiInput class
  | for more details.
  | 
  | @see MidiInput
  | 
  | @tags{Audio}
  |
  */
pub trait MidiInputCallback {

    /**
      | Receives an incoming message.
      | 
      | A MidiInput object will call this method
      | when a midi event arrives. It'll be called
      | on a high-priority system thread, so
      | avoid doing anything time-consuming
      | in here, and avoid making any UI calls.
      | You might find the MidiBuffer class
      | helpful for queueing incoming messages
      | for use later.
      | 
      | -----------
      | @param source
      | 
      | the MidiInput object that generated
      | the message
      | ----------
      | @param message
      | 
      | the incoming message. The message's
      | timestamp is set to a value equivalent
      | to (Time::getMillisecondCounter()
      | / 1000.0) to specify the time when the
      | message arrived
      |
      */
    fn handle_incoming_midi_message(&mut self, 
            source:  *mut MidiInput,
            message: &MidiMessage);

   /**
      | Notification sent each time a packet
      | of a multi-packet sysex message arrives.
      | 
      | If a long sysex message is broken up into
      | multiple packets, this callback is
      | made for each packet that arrives until
      | the message is finished, at which point
      | the normal handleIncomingMidiMessage()
      | callback will be made with the entire
      | message.
      | 
      | The message passed in will contain the
      | start of a sysex, but won't be finished
      | with the terminating 0xf7 byte.
      |
      */
    fn handle_partial_sysex_message(&mut self, 
        source:           *mut MidiInput,
        message_data:     *const u8,
        num_bytes_so_far: i32,
        timestamp:        f64)  {
        
        todo!();
        /*
            ignoreUnused (source, messageData, numBytesSoFar, timestamp);
        */
    }
}
