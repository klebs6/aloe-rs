crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstmessage.h]

/**
  | Private plug-in message: Vst::IMessage
  | \ingroup vstIHost vst300
  | 
  | - [host imp]
  | 
  | - [create via IHostApplication::createInstance]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | Messages are sent from a Vst controller
  | component to a Vst editor component
  | and vice versa. \see IAttributeList,
  | IConnectionPoint, \ref vst3Communication
  |
  */
pub trait IMessage: FUnknown {

    /**
      | Returns the message ID (for example
      | "TextMessage").
      |
      */
    #[PLUGIN_API]
    fn get_messageid(&mut self) -> FIDString;

    /**
      | Sets a message ID (for example "TextMessage").
      |
      */
    #[PLUGIN_API]
    fn set_messageid(&mut self, id: FIDString);

    /**
      | Returns the attribute list associated
      | to the message.
      |
      */
    #[PLUGIN_API]
    fn get_attributes(&mut self) -> *mut dyn IAttributeList<AttrID = *const u8>;
}

lazy_static!{
    /*
    static const FUID imessage_iid;
    */
}

declare_class_iid!{
    IMessage, 
    0x936F033B, 
    0xC6C047DB, 
    0xBB0882F8, 
    0x13C1E613
}
