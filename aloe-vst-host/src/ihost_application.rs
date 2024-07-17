crate::ix!();

/**
  | Basic host callback interface: Vst::IHostApplication
  | \ingroup vstIHost vst300
  | 
  | - [host imp]
  | 
  | - [passed as 'context' in to IPluginBase::initialize
  | () ]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | Basic VST host application interface.
  |
  */
pub trait IHostApplication: FUnknown {

    /**
      | Gets host application name.
      |
      */
    #[PLUGIN_API]
    fn get_name(&mut self, name: String128) -> tresult;

    /**
      | Creates host object (e.g. Vst::IMessage).
      |
      */
    #[PLUGIN_API]
    fn create_instance(&mut self, 
            cid: TUID,
            iid: TUID,
            obj: *mut *mut c_void) -> tresult;
}

lazy_static!{
    /*
    static const FUID ihost_application_iid;
    */
}

declare_class_iid!{
    IHostApplication, 
    0x58E595CC, 
    0xDB2D4969, 
    0x8B6AAF8C, 
    0x36A664E5
}

/**
  | Helper to allocate a message
  |
  */
#[inline] pub fn allocate_message(host: *mut dyn IHostApplication) -> *mut dyn IMessage {
    
    todo!();
        /*
            TUID iid;
        IMessage::iid.toTUID (iid);
        IMessage* m = nullptr;
        if (host->createInstance (iid, iid, (void**)&m) == kResultOk)
            return m;
        return nullptr;
        */
}
