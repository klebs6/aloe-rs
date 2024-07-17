crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/utilities/aloe_Vst3ClientExtensions.h]

pub type TUID = [u8; 16];

/**
  | An interface to allow an AudioProcessor
  | to implement extended Vst3-specific
  | functionality.
  | 
  | To use this class, ensure that your AudioProcessor
  | publicly inherits from Vst3ClientExtensions.
  | 
  | @see VstCallbackHandler
  | 
  | @tags{Audio}
  |
  */
pub struct Vst3ClientExtensions {

}

impl Vst3ClientExtensions {

    /**
      | This function may be used by implementations
      | of queryInterface() in the Vst3's implementation
      | of IEditController to return additional
      | supported interfaces.
      |
      */
    fn query_iedit_controller(&mut self, 
        _0:  TUID,
        obj: *mut *mut c_void) -> i32 {
        
        todo!();
        /*
            *obj = nullptr;
            return -1;
        */
    }

    /**
      | This function may be used by implementations
      | of queryInterface() in the Vst3's implementation
      | of IAudioProcessor to return additional
      | supported interfaces.
      |
      */
    fn query_iaudio_processor(&mut self, 
        _0:  TUID,
        obj: *mut *mut c_void) -> i32 {
        
        todo!();
        /*
            *obj = nullptr;
            return -1;
        */
    }
}

pub trait SetIComponentHandler {

    fn set_icomponent_handler(&mut self, _0: *mut dyn FUnknown);
}

impl SetIComponentHandler for Vst3ClientExtensions {

    /**
      | This may be called by the Vst3 wrapper
      | when the host sets an
      | 
      | IComponentHandler for the plugin to
      | use.
      | 
      | You should not make any assumptions
      | about how and when this will be called
      | - this function may not be called at all!
      |
      */
    fn set_icomponent_handler(&mut self, _0: *mut dyn FUnknown)  {
        
        todo!();
        /*
        
        */
    }
}

pub trait SetIHostApplication {

    fn set_ihost_application(&mut self, _0: *mut dyn FUnknown);
}

impl SetIHostApplication for Vst3ClientExtensions {

    /**
      | This may be called shortly after the
      | AudioProcessor is constructed with
      | the current IHostApplication.
      | 
      | You should not make any assumptions
      | about how and when this will be called
      | - this function may not be called at all!
      |
      */
    fn set_ihost_application(&mut self, _0: *mut dyn FUnknown)  {
        
        todo!();
        /*
        
        */
    }
}

pub trait GetPluginHasMainInput {

    fn get_plugin_has_main_input(&self) -> bool;
}

impl GetPluginHasMainInput for Vst3ClientExtensions {

    /**
      | This function will be called to check
      | whether the first input bus should be
      | designated as "kMain" or "kAux". Return
      | true if the first bus should be kMain,
      | or false if the bus should be kAux.
      | 
      | All other input buses will always be
      | designated kAux.
      |
      */
    fn get_plugin_has_main_input(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
