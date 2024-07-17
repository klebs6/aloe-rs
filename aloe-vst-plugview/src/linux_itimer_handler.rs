crate::ix!();

/**
  | Linux timer handler interface \ingroup
  | pluginGUI vst368
  | 
  | - [plug imp]
  | 
  | - [released: 3.6.8] \see IRunLoop
  |
  */
pub trait ITimerHandler: FUnknown {

    #[PLUGIN_API]
    fn on_timer(&mut self);

}

lazy_static!{
    /*
    static const FUID itimer_handler_iid;
    */
}

declare_class_iid!{
    ITimerHandler, 
    0x10BDD94F, 
    0x41424774, 
    0x821FAD8F, 
    0xECA72CA9
}

