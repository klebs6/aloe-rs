crate::ix!();

pub trait IsSysEx {

    /**
      | Returns true if this is a system-exclusive
      | message.
      |
      */
    fn is_sys_ex(&self) -> bool;
}

pub trait CreateSysExMessage {

    /**
      | Creates a system-exclusive message.
      | The data passed in is wrapped with header
      | and tail bytes of 0xf0 and 0xf7.
      |
      */
    fn create_sys_ex_message(
        &mut self, 
        sysex_data: *const c_void,
        data_size:  i32
    ) -> dyn MidiMessageInterface;
}

pub trait GetSysExData {

    /**
      | Returns a pointer to the sysex data inside
      | the message. If this event isn't a sysex
      | event, it'll return 0. @see getSysExDataSize
      |
      */
    fn get_sys_ex_data(&self) -> *const u8;
}

pub trait GetSysExDataSize {

    /**
      | Returns the size of the sysex data. This
      | value excludes the 0xf0 header byte
      | and the 0xf7 at the end. @see getSysExData
      |
      */
    fn get_sys_ex_data_size(&self) -> i32;
}
