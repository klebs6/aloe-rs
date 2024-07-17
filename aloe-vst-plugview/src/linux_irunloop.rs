crate::ix!();

/**
  | Linux host run loop interface \ingroup
  | pluginGUI vst368
  | 
  | - [host imp]
  | 
  | - [extends IPlugFrame]
  | 
  | - [released: 3.6.8]
  | 
  | On Linux the host has to provide this
  | interface to the plug-in as there's
  | no global event run loop defined as on
  | other platforms.
  | 
  | A plug-in can register an event handler
  | for a file descriptor. The host has to
  | call the event handler when the file
  | descriptor is marked readable.
  | 
  | A plug-in also can register a timer which
  | will be called repeatedly until it is
  | unregistered.
  |
  */
pub trait IRunLoop: FUnknown {

    #[PLUGIN_API]
    fn register_event_handler(&mut self, 
            handler: *mut IEventHandler,
            fd:      FileDescriptor) -> tresult;

    #[PLUGIN_API]
    fn unregister_event_handler(&mut self, handler: *mut IEventHandler) -> tresult;

    #[PLUGIN_API]
    fn register_timer(&mut self, 
            handler:      *mut ITimerHandler,
            milliseconds: TimerInterval) -> tresult;

    #[PLUGIN_API]
    fn unregister_timer(&mut self, handler: *mut ITimerHandler) -> tresult;
}

lazy_static!{
    /*
    static const FUID irun_loop_iid;
    */
}

declare_class_iid!{
    IRunLoop, 
    0x18C35366, 
    0x97764F1A, 
    0x9C5B8385, 
    0x7A871389
}
