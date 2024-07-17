crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstevents.h]

/**
  | List of events to process: Vst::IEventList
  | \ingroup vstIHost vst300
  | 
  | - [host imp]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | \see ProcessData, VstEvent
  |
  */
pub trait IEventList: FUnknown {

    /**
      | Returns the count of events.
      |
      */
    #[PLUGIN_API]
    fn get_event_count(&mut self) -> i32;

    /**
      | Gets parameter by index.
      |
      */
    #[PLUGIN_API]
    fn get_event(&mut self, 
            index: i32,
            e:     &mut VstEvent) -> tresult;

    /**
      | Adds a new event.
      |
      */
    #[PLUGIN_API]
    fn add_event(&mut self, e: &mut VstEvent) -> tresult;
}

lazy_static!{
    /*
    static const FUID ievent_list_iid;
    */
}

declare_class_iid!{
    IEventList, 
    0x3A2C4214, 
    0x346349FE, 
    0xB2C4F397, 
    0xB9695A44
}
