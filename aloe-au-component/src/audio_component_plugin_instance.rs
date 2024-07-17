crate::ix!();

pub struct AudioComponentPlugInInstance {

    plug_in_interface: AudioComponentPlugInInterface,

    construct:         fn(memory: *mut c_void, ci: AudioComponentInstance) -> *mut c_void,

    destruct:          fn(memory: *mut c_void) -> c_void,

    /**
      | pad to a 16-byte boundary (in either
      | 32 or 64 bit mode)
      |
      */
    pad:               [*mut c_void; 2],

    /**
      | the ACI implementation object is constructed
      | into this memory this member is just
      | a placeholder. it is aligned to a 16byte
      | boundary
      */
    instance_storage:  u32,
}
