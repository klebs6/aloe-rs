/*!
  | \defgroup pluginBase Basic Interfaces
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/funknown.h]

/**
  | The basic interface of all interfaces.
  | \ingroup pluginBase
  | 
  | - The FUnknown::queryInterface method
  | is used to retrieve pointers to other
  | interfaces of the object.
  | 
  | - FUnknown::addRef and FUnknown::release
  | manage the lifetime of the object.
  | 
  | If no more references exist, the object
  | is destroyed in memory.
  | 
  | Interfaces are identified by 16 byte
  | Globally Unique Identifiers.
  | 
  | The SDK provides a class called FUID
  | for this purpose.
  | 
  | \ref howtoClass
  |
  */
pub trait FUnknown {

    /**
      | Query for a pointer to the specified
      | interface.
      | 
      | Returns kResultOk on success or kNoInterface
      | if the object does not implement the
      | interface.
      | 
      | The object has to call addRef when returning
      | an interface.
      | 
      | -----------
      | @param _iid
      | 
      | : (in) 16 Byte interface identifier
      | (-> FUID)
      | ----------
      | @param obj
      | 
      | : (out) On return, *obj points to the
      | requested interface
      |
      */
    #[PLUGIN_API]
    fn query_interface(&mut self, 
            iid: TUID,
            obj: *mut *mut c_void) -> tresult;

    /**
      | Adds a reference and returns the new
      | reference count. \par Remarks:
      | 
      | The initial reference count after creating
      | an object is 1.
      |
      */
    #[PLUGIN_API]
    fn add_ref(&mut self) -> u32;

    /**
      | Releases a reference and returns the
      | new reference count.
      | 
      | If the reference count reaches zero,
      | the object will be destroyed in memory.
      |
      */
    #[PLUGIN_API]
    fn release(&mut self) -> u32;
}

lazy_static!{
    /*
    static const FUID funknown_iid;
    */
}

declare_class_iid!{
    FUnknown, 
    0x00000000, 
    0x00000000, 
    0xC0000000, 
    0x00000046
}
