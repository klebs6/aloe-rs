crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/base/ipersistent.h]

/**
  | Persistent Object Interface. [plug
  | imp] \n
  | 
  | This interface is used to store/restore
  | attributes of an object.
  | 
  | An IPlugController can implement this
  | interface to handle presets.
  | 
  | The gui-xml for a preset control looks
  | like this:
  | 
  | -----------
  | @code
  | 
  | ....
  | <view name="PresetView" data="Preset"/>
  | ....
  | <template name="PresetView">
  |     <view name="preset control" size="0, 0, 100, 20"/>
  |     <switch name="store preset" size="125,0,80,20" style="push|immediate" title="Store"  />
  |     <switch name="remove preset" size="220,0,80,20" style="push|immediate" title="Delete"  />
  | </template>
  | 
  | The tag data="Preset" tells the host
  | to create a preset controller that handles
  | the 3 values named "preset control",
  | "store preset", and "remove preset".
  |
  */
pub trait IPersistent: FUnknown {

    /**
      | The class ID must be a 16 bytes unique
      | id that is used to create the object.
      | 
      | This ID is also used to identify the preset
      | list when used with presets.
      |
      */
    #[PLUGIN_API]
    fn get_classid(&mut self, uid: *mut u8) -> tresult;

    /**
      | Store all members/data in the passed
      | IAttributes.
      |
      */
    #[PLUGIN_API]
    fn save_attributes(&mut self, _0: *mut dyn IAttributes) -> tresult;

    /**
      | Restore all members/data from the passed
      | IAttributes.
      |
      */
    #[PLUGIN_API]
    fn load_attributes(&mut self, _0: *mut dyn IAttributes) -> tresult;
}

lazy_static!{
    /*
    static const FUID ipersistent_iid;
    */
}

declare_class_iid!{
    IPersistent, 
    0xBA1A4637, 
    0x3C9F46D0, 
    0xA65DBA0E, 
    0xB85DA829
}
