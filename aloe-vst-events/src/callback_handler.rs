crate::ix!();

pub trait HandleVstManufacturerSpecific {

    /**
      | This is called by the Vst plug-in wrapper
      | when it receives unhandled vendor specific
      | calls from the host.
      |
      */
    fn handle_vst_manufacturer_specific(&mut self, 
            index: i32,
            value: PointerSizedInt,
            ptr:   *mut c_void,
            opt:   f32) -> PointerSizedInt;
}

/**
  | The host callback function type.
  |
  */
pub type VstHostCallbackType = fn(
        opcode: i32,
        index:  i32,
        value:  PointerSizedInt,
        ptr:    *mut c_void,
        opt:    f32
) -> PointerSizedInt;

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_VstCallbackHandler.h]

/**
  | An interface to allow an AudioProcessor
  | to send and receive Vst specific calls
  | from the host.
  | 
  | To use this class, ensure that your AudioProcessor
  | publicly inherits from VstCallbackHandler.
  | 
  | @see Vst3ClientExtensions
  | 
  | @tags{Audio}
  |
  */
pub struct VstCallbackHandler {

}

pub trait VstCallbackHandlerInterface: HandleVstManufacturerSpecific {}

pub trait HandleVstPluginCanDo {

    fn handle_vst_plugin_can_do(&mut self, 
        index: i32,
        value: PointerSizedInt,
        ptr:   *mut c_void,
        opt:   f32) -> PointerSizedInt;
}

impl HandleVstPluginCanDo for VstCallbackHandler {

    /**
      | This is called by the Vst plug-in wrapper
      | when it receives unhandled plug-in
      | "can do" calls from the host.
      |
      */
    fn handle_vst_plugin_can_do(&mut self, 
        index: i32,
        value: PointerSizedInt,
        ptr:   *mut c_void,
        opt:   f32) -> PointerSizedInt {
        
        todo!();
        /*
            ignoreUnused (index, value, ptr, opt);
            return 0;
        */
    }
}

pub trait HandleVstHostCallbackAvailable {

    /**
      | This is called once by the Vst plug-in
      | wrapper after its constructor.
      | 
      | You can use the supplied function to
      | query the Vst host.
      |
      */
    fn handle_vst_host_callback_available(&mut self, callback: VstHostCallbackType)  {
        
        todo!();
        /*
            ignoreUnused (callback);
        */
    }
}
