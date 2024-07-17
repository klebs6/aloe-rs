crate::ix!();

/**
  | Description of an VstEvent Bus. \ingroup
  | vstClasses
  |
  */
pub struct VstEventBus {
    base:          Bus,
    channel_count: i32,
}

obj_methods!{ VstEventBus, VstBus }

impl VstEventBus {

    /* ---from Bus------- */

    /**
      | Constructor of an VstEventBus.
      |
      */
    pub fn new(
        name:          *const TChar,
        bus_type:      BusType,
        flags:         i32,
        channel_count: i32) -> Self {
    
        todo!();
        /*
        : bus(name, busType, flags),
        : channel_count(channelCount),

        
        */
    }
    
    /**
      | Gets the BusInfo associated to this
      | VstEvent bus.
      |
      */
    pub fn get_info(&mut self, info: &mut VstBusInfo) -> bool {
        
        todo!();
        /*
            info.channelCount = channelCount;
        return Bus::getInfo (info);
        */
    }
}
