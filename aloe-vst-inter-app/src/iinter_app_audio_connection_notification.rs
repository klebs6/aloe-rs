crate::ix!();

/**
  | Extended plug-in interface IEditController
  | for Inter-App Audio connection state
  | change notifications \ingroup vstIPlug
  | vst360
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.6.0]
  |
  */
pub trait IInterAppAudioConnectionNotification: FUnknown {

    /**
      | called when the Inter-App Audio connection
      | state changes
      | 
      | -----------
      | @param newState
      | 
      | true if an Inter-App Audio connection
      | is established, otherwise false
      |
      */
    #[PLUGIN_API]
    fn on_inter_app_audio_connection_state_change(&mut self, new_state: TBool);
}

lazy_static!{
    /*
    static const FUID iinter_app_audio_connection_notification_iid;
    */
}

declare_class_iid!{
    IInterAppAudioConnectionNotification, 
    0x6020C72D, 
    0x5FC24AA1, 
    0xB0950DB5, 
    0xD7D6D5CF
}
