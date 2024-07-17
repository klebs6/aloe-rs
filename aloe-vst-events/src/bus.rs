crate::ix!();

/**
  | Basic Bus object. \ingroup vstClasses
  |
  */
pub struct Bus {

    base: FObject,

    /**
      | name
      |
      */
    name:     String,

    /**
      | kMain or kAux, see \ref BusTypes
      |
      */
    bus_type: BusType,

    /**
      | flags, see \ref BusInfo::BusFlags
      |
      */
    flags:    i32,

    /**
      | activation state
      |
      */
    active:   TBool,
}

obj_methods!{ Vst::Bus, FObject }

impl Bus {

    /**
      | Returns true if the bus is active.
      |
      */
    pub fn is_active(&self) -> TBool {
        
        todo!();
        /*
            return active;
        */
    }

    /**
      | Activates the bus.
      |
      */
    pub fn set_active(&mut self, state: TBool)  {
        
        todo!();
        /*
            active = state;
        */
    }

    /**
      | Sets a new name for this bus.
      |
      */
    pub fn set_name(&mut self, new_name: String)  {
        
        todo!();
        /*
            name = newName;
        */
    }

    /**
      | Sets a new busType for this bus.
      |
      */
    pub fn set_bus_type(&mut self, new_bus_type: BusType)  {
        
        todo!();
        /*
            busType = newBusType;
        */
    }

    /**
      | Sets a new flags for this bus.
      |
      */
    pub fn set_flags(&mut self, new_flags: u32)  {
        
        todo!();
        /*
            flags = newFlags;
        */
    }

    pub fn new(
        name:     *const TChar,
        bus_type: BusType,
        flags:    i32) -> Self {
    
        todo!();
        /*
        : name(_name),
        : bus_type(_busType),
        : flags(_flags),
        : active(false),

        
        */
    }
    
    pub fn get_info(&mut self, info: &mut VstBusInfo) -> bool {
        
        todo!();
        /*
            name.copyTo16 (info.name, 0, str16BufferSize (info.name) - 1);
        info.busType = busType;
        info.flags = flags;
        return true;
        */
    }
}
