crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/iupdatehandler.h]

/**
  | Host implements dependency handling
  | for plugins.
  | 
  | - [host imp]
  | 
  | - [get this interface from IHostClasses]
  | 
  | - [released N3.1]
  | 
  | - Install/Remove change notifications
  | 
  | - Trigger updates when an object has
  | changed
  | 
  | Can be used between host-objects and
  | the Plug-In or inside the Plug-In to
  | handle internal updates!
  | 
  | \see IDependent \ingroup frameworkHostClasses
  |
  */
pub trait IUpdateHandler: FUnknown {

    /**
      | Install update notification for given
      | object. It is essential to remove all
      | dependencies again using 'removeDependent'!
      | Dependencies are not removed automatically
      | when the 'object' is released!
      | 
      | -----------
      | @param object
      | 
      | : interface to object that sends change
      | notifications
      | ----------
      | @param dependent
      | 
      | : interface through which the update
      | is passed
      |
      */
    fn add_dependent(&mut self, 
            object:    *mut dyn FUnknown,
            dependent: *mut dyn IDependent) -> tresult;

    /**
      | Remove a previously installed dependency.
      |
      */
    fn remove_dependent(&mut self, 
            object:    *mut dyn FUnknown,
            dependent: *mut dyn IDependent) -> tresult;

    /**
      | Inform all dependents, that object
      | has changed.
      | 
      | -----------
      | @param object
      | 
      | is the object that has changed
      | ----------
      | @param message
      | 
      | is a value of enum IDependent::ChangeMessage,
      | usually IDependent::kChanged - can
      | be a private message as well (only known
      | to sender and dependent)
      |
      */
    fn trigger_updates(&mut self, 
            object:  *mut dyn FUnknown,
            message: i32) -> tresult;

    /**
      | Same as triggerUpdates, but delivered
      | in idle (usefull to collect updates).
      |
      */
    fn defer_updates(&mut self, 
            object:  *mut dyn FUnknown,
            message: i32) -> tresult;
}

lazy_static!{
    /*
    static const FUID iupdate_handler_iid;
    */
}

declare_class_iid!{
    IUpdateHandler, 
    0xF5246D56, 
    0x86544d60, 
    0xB026AFB5, 
    0x7B697B37
}


