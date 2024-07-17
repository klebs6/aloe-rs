crate::ix!();

/**
  | Description of an Audio Bus. \ingroup
  | vstClasses
  |
  */
pub struct AudioBus {
    base:        Bus,
    speaker_arr: SpeakerArrangement,
}

impl AudioBus {
    
    /**
      | Gets the speaker arrangement defining
      | this Audio bus.
      |
      */
    pub fn get_arrangement(&self) -> SpeakerArrangement {
        
        todo!();
        /*
            return speakerArr;
        */
    }

    /**
      | Sets the speaker arrangement defining
      | this Audio bus.
      |
      */
    pub fn set_arrangement(&mut self, arr: &SpeakerArrangement)  {
        
        todo!();
        /*
            speakerArr = arr;
        */
    }

    /* ---from Bus--------------------- */

    obj_methods!{ Vst::AudioBus, Vst::Bus }

    pub fn new(
        name:     *const TChar,
        bus_type: BusType,
        flags:    i32,
        arr:      SpeakerArrangement) -> Self {
    
        todo!();
        /*
        : bus(name, busType, flags),
        : speaker_arr(arr),

        
        */
    }
    
    /**
      | Gets the BusInfo associated to this
      | Audio bus.
      |
      */
    pub fn get_info(&mut self, info: &mut VstBusInfo) -> bool {
        
        todo!();
        /*
            info.channelCount = SpeakerArr::getChannelCount (speakerArr);
        return Bus::getInfo (info);
        */
    }
}
