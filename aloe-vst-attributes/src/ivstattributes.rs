crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstattributes.h]

/**
  | Attribute list used in IMessage and
  | IStreamAttributes: Vst::IAttributeList
  | \ingroup vstIHost vst300
  | 
  | - [host imp]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | An attribute list associates values
  | with a key (id: some predefined keys
  | can be found in \ref presetAttributes).
  |
  */
pub trait IAttributeList: FUnknown {

    type AttrID = *const u8;

    /**
      | Sets integer value.
      |
      */
    #[PLUGIN_API]
    fn set_int(&mut self, 
            id:    Self::AttrID,
            value: i64) -> tresult;

    /**
      | Gets integer value.
      |
      */
    #[PLUGIN_API]
    fn get_int(&mut self, 
            id:    Self::AttrID,
            value: &mut i64) -> tresult;

    /**
      | Sets float value.
      |
      */
    #[PLUGIN_API]
    fn set_float(&mut self, 
            id:    Self::AttrID,
            value: f64) -> tresult;

    /**
      | Gets float value.
      |
      */
    #[PLUGIN_API]
    fn get_float(&mut self, 
            id:    Self::AttrID,
            value: &mut f64) -> tresult;

    /**
      | Sets string value (UTF16) (should be
      | null-terminated!).
      |
      */
    #[PLUGIN_API]
    fn set_string(&mut self, 
            id:     Self::AttrID,
            string: *const TChar) -> tresult;

    /**
      | Gets string value (UTF16). Note that
      | Size is in Byte, not the string Length!
      | 
      | Do not forget to multiply the length
      | by sizeof (TChar)!
      |
      */
    #[PLUGIN_API]
    fn get_string(&mut self, 
            id:            Self::AttrID,
            string:        *mut TChar,
            size_in_bytes: u32) -> tresult;

    /**
      | Sets binary data.
      |
      */
    #[PLUGIN_API]
    fn set_binary(&mut self, 
            id:            Self::AttrID,
            data:          *const c_void,
            size_in_bytes: u32) -> tresult;

    /**
      | Gets binary data.
      |
      */
    #[PLUGIN_API]
    fn get_binary(&mut self, 
            id:            Self::AttrID,
            data:          &mut *const c_void,
            size_in_bytes: &mut u32) -> tresult;
}

lazy_static!{
    /*
    static const FUID iattribute_list_iid;
    */
}

declare_class_iid!{
    IAttributeList, 
    0x1E5F0AEB, 
    0xCC7F4533, 
    0xA2544011, 
    0x38AD5EE4
}

/**
  | Meta attributes of a stream: Vst::IStreamAttributes
  | \ingroup vstIHost vst360
  | 
  | - [host imp]
  | 
  | - [extends IBStream]
  | 
  | - [released: 3.6.0]
  | 
  | - [optional]
  | 
  | Interface to access preset meta information
  | from stream, used, for example, in setState
  | in order to inform the plug-in about
  | the current context in which the preset
  | loading occurs (Project context or
  | Preset load (see \ref StateType)) or
  | used to get the full file path of the loaded
  | preset (if available).
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | #include "pluginterfaces/base/ustring.h"
  | #include "pluginterfaces/vst/vstpresetkeys.h"
  | ...
  | 
  | tresult PLUGIN_API MyPlugin::setState (IBStream* state)
  | {
  |     FUnknownPtr<IStreamAttributes> stream (state);
  |     if (stream)
  |     {
  |         IAttributeList* list = stream->getAttributes ();
  |         if (list)
  |         {
  |             // get the current type (project/Default..) of this state
  |             String128 string;
  |             if (list->getString (PresetAttributes::kStateType, string, 128 * sizeof (TChar)) == kResultTrue)
  |             {
  |                 UString128 tmp (string);
  |                 char ascii[128];
  |                 tmp.toAscii (ascii, 128);
  |                 if (!strncmp (ascii, StateType::kProject, strlen (StateType::kProject)))
  |                 {
  |                     // we are in project loading context...
  |                 }
  |             }
  | 
  |             // get the full file path of this state
  |             TChar fullPath[1024];
  |             if (list->getString (PresetAttributes::kFilePathStringType, fullPath, 1024 * sizeof (TChar)) == kResultTrue)
  |             {
  |                 // here we have the full path ...
  |             }
  |         }
  |     }
  | 
  |     //...read the state here.....
  |     return kResultTrue;
  | }
  |
  */
pub trait IStreamAttributes: FUnknown {

    /**
      | Gets filename (without file extension)
      | of the stream.
      |
      */
    #[PLUGIN_API]
    fn get_file_name(&mut self, name: String128) -> tresult;

    /**
      | Gets meta information list.
      |
      */
    #[PLUGIN_API]
    fn get_attributes(&mut self) -> *mut dyn IAttributeList<AttrID = *const u8>;
}

lazy_static!{
    /*
    static const FUID istream_attribute_iid;
    */
}

declare_class_iid!{
    IStreamAttributes, 
    0xD6CE2FFC, 
    0xEFAF4B8C, 
    0x9E74F1BB, 
    0x12DA44B4
}
