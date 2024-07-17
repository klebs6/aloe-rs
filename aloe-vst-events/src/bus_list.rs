crate::ix!();

/**
  | VstBusInfo:
  | 
  | This is the structure used with getBusInfo,
  | informing the host about what is a specific
  | given bus. \n See also: typename Steinberg::Vst::VstIComponent::getBusInfo
  |
  */
pub struct VstBusInfo {

    /**
      | Media type - has to be a value of \ref
      | VstMediaTypes
      |
      */
    media_type:    MediaType,

    /**
      | input or output \ref VstBusDirections
      |
      */
    direction:     BusDirection,

    /**
      | number of channels (if used then need to be
      | recheck after \ref
      | IAudioProcessor::setBusArrangements is
      | called).  For a bus of type VstMediaTypes::kEvent
      | the channelCount corresponds to the number of
      | supported MIDI channels by this bus
      */
    channel_count: i32,

    /**
      | name of the bus
      |
      */
    name:          String128,

    /**
      | main or aux - has to be a value of \ref
      | VstBusTypes
      |
      */
    bus_type:      BusType,

    /**
      | flags - a combination of \ref VstBusInfoBusFlags
      |
      */
    flags:         u32,
}

/**
  | List of Busses. \ingroup vstClasses
  |
  */
pub struct BusList {
    base:      FObject,
    base2:     Vec<IPtr<Bus>>,
    ty:        MediaType,
    direction: BusDirection,
}

obj_methods!{ Vst::BusList, FObject }

impl BusList {

    /**
      | Returns the BusList Type. See \ref MediaType
      |
      */
    pub fn get_type(&self) -> MediaType {
        
        todo!();
        /*
            return type;
        */
    }

    /**
      | Returns the BusList direction. See
      | \ref BusDirection
      |
      */
    pub fn get_direction(&self) -> BusDirection {
        
        todo!();
        /*
            return direction;
        */
    }

    pub fn new(
        ty:  MediaType,
        dir: BusDirection) -> Self {
    
        todo!();
        /*
        : ty(type),
        : direction(dir),

        
        */
    }
}
