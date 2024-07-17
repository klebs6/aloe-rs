crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/extern/reaper_vst3_interfaces.h]

/**
  | available from IHostApplication in
  | REAPER v5.02+
  |
  */
pub trait IReaperHostApplication: FUnknown {

    /**
      | Gets a REAPER Extension API function
      | by name, returns NULL is failed
      |
      */
    #[PLUGIN_API]
    fn get_reaper_api(&mut self, funcname: CStringA);

    /**
      | get parent track(=1), take(=2), project(=3),
      | fxdsp(=4), trackchan(=5)
      |
      */
    #[PLUGIN_API]
    fn get_reaper_parent(&mut self, w: u32);

    /**
      | Multi-purpose function, returns NULL
      | if unsupported
      |
      */
    #[PLUGIN_API]
    fn reaper_extended(&mut self, 
            call:  u32,
            parm1: *mut c_void,
            parm2: *mut c_void,
            parm3: *mut c_void);
}

lazy_static!{
    /*
    static const FUID ireaper_host_application_iid;
    */
}

declare_class_iid!{
    IReaperHostApplication, 
    0x79655E36, 0x77EE4267, 0xA573FEF7, 0x4912C27C
}
