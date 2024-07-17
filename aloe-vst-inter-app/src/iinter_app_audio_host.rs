crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstinterappaudio.h]

/**
  | Inter-App Audio host Interface. \ingroup
  | vstIHost vst360
  | 
  | - [host imp]
  | 
  | - [passed as 'context' to IPluginBase::initialize() ]
  | 
  | - [released: 3.6.0]
  | 
  | - [optional]
  | 
  | Implemented by the InterAppAudio Wrapper.
  |
  */
pub trait IInterAppAudioHost: FUnknown {

    /**
      | get the size of the screen
      | 
      | -----------
      | @param size
      | 
      | size of the screen
      | ----------
      | @param scale
      | 
      | scale of the screen
      | 
      | -----------
      | @return
      | 
      | kResultTrue on success
      |
      */
    #[PLUGIN_API]
    fn get_screen_size(&mut self, 
            size:  *mut ViewRect,
            scale: *mut f32) -> tresult;

    /**
      | get status of connection
      | 
      | 
      | -----------
      | @return
      | 
      | kResultTrue if an Inter-App Audio connection
      | is established
      |
      */
    #[PLUGIN_API]
    fn connected_to_host(&mut self) -> tresult;

    /**
      | switch to the host.
      | 
      | 
      | -----------
      | @return
      | 
      | kResultTrue on success
      |
      */
    #[PLUGIN_API]
    fn switch_to_host(&mut self) -> tresult;

    /**
      | send a remote control event to the host
      | 
      | -----------
      | @param event
      | 
      | event type, see AudioUnitRemoteControlEvent
      | in the iOS SDK documentation for possible
      | types
      | 
      | -----------
      | @return
      | 
      | kResultTrue on success
      |
      */
    #[PLUGIN_API]
    fn send_remote_control_event(&mut self, event: u32) -> tresult;

    /**
      | ask for the host icon.
      | 
      | -----------
      | @param icon
      | 
      | pointer to a CGImageRef
      | 
      | -----------
      | @return
      | 
      | kResultTrue on success
      |
      */
    #[PLUGIN_API]
    fn get_host_icon(&mut self, icon: *mut *mut c_void) -> tresult;

    /**
      | schedule an event from the user interface
      | thread
      | 
      | -----------
      | @param event
      | 
      | the event to schedule
      | 
      | -----------
      | @return
      | 
      | kResultTrue on success
      |
      */
    #[PLUGIN_API]
    fn schedule_event_fromui(&mut self, event: &mut VstEvent) -> tresult;

    /**
      | get the preset manager
      | 
      | -----------
      | @param cid
      | 
      | class ID to use by the preset manager
      | 
      | -----------
      | @return
      | 
      | the preset manager. Needs to be released
      | by called.
      |
      */
    #[PLUGIN_API]
    fn create_preset_manager(&mut self, cid: &TUID) -> *mut dyn IInterAppAudioPresetManager;

    /**
      | show the settings view currently includes
      | MIDI settings and Tempo setting
      | 
      | 
      | -----------
      | @return
      | 
      | kResultTrue on success
      |
      */
    #[PLUGIN_API]
    fn show_settings_view(&mut self) -> tresult;
}

lazy_static!{
    /*
    static const FUID iinter_app_audio_host_iid;
    */
}

declare_class_iid!{
    IInterAppAudioHost, 
    0x0CE5743D, 
    0x68DF415E, 
    0xAE285BD4, 
    0xE2CDC8FD
}
