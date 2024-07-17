/*!
  | Description : Vst Bus Implementation
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vstbus.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vstbus.cpp]

pub trait GetInfo {

    /**
      | Gets the BusInfo of this bus.
      |
      */
    fn get_info(&mut self, _0: &mut VstBusInfo) -> bool;
}
