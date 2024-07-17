/**
  | \defgroup vst3typedef Vst 3 Data Types
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstnoteexpression.h]

/**
  | Extended plug-in interface IEditController
  | for key switches support: Vst::IKeyswitchController
  | \ingroup vstIPlug vst350
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.5.0]
  | 
  | - [optional]
  | 
  | When a (instrument) plug-in supports
  | such interface, the host could get from
  | the plug-in the current set of used key
  | switches (megatrig/articulation)
  | for a given channel of a event bus and
  | then automatically use them (like in
  | Cubase 6) to create Vst Expression Map
  | (allowing to associated symbol to a
  | given articulation / key switch).
  |
  */
pub trait IKeyswitchController: FUnknown {

    /**
      | Returns number of supported key switches
      | for event bus index and channel.
      |
      */
    #[PLUGIN_API]
    fn get_keyswitch_count(&mut self, 
            bus_index: i32,
            channel:   i16) -> i32;

    /**
      | Returns key switch info.
      |
      */
    #[PLUGIN_API]
    fn get_keyswitch_info(&mut self, 
            bus_index:        i32,
            channel:          i16,
            key_switch_index: i32,
            info:             &mut KeyswitchInfo) -> tresult;
}

lazy_static!{
    /*
    static const FUID ikeyswitch_controller_iid;
    */
}

declare_class_iid!{
    IKeyswitchController, 
    0x1F2F76D3, 
    0xBFFB4B96, 
    0xB99527A5, 
    0x5EBCCEF4
}
