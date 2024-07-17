crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/gui/iplugviewcontentscalesupport.h]

/**
  | Plug-in view content scale support
  | \ingroup pluginGUI vstIPlug vst366
  | 
  | - [plug impl]
  | 
  | - [extends IPlugView]
  | 
  | - [released: 3.6.6]
  | 
  | - [optional]
  | 
  | This interface communicates the content
  | scale factor from the host to the plug-in
  | view on systems where plug-ins cannot
  | get this information directly like
  | Microsoft Windows.
  | 
  | The host calls setContentScaleFactor
  | directly after the plug-in view is attached
  | and when the scale factor changes (system
  | change or window moved to another screen
  | with different scaling settings).
  | 
  | The host could call setContentScaleFactor
  | in a different context, for example:
  | scaling the plug-in editor for better
  | readability.
  | 
  | When a plug-in handles this (by returning
  | kResultTrue), it needs to scale the
  | width and height of its view by the scale
  | factor and inform the host via a IPlugFrame::resizeView(),
  | the host will then call IPlugView::onSize().
  | 
  | -----------
  | @note
  | 
  | the host is allowed to call setContentScaleFactor()
  | at any time the IPlugView is valid.
  |
  */
pub trait IPlugViewContentScaleSupport: FUnknown {

    type ScaleFactor = f32;

    #[PLUGIN_API]
    fn set_content_scale_factor(&mut self, factor: Self::ScaleFactor) -> tresult;
}

lazy_static!{
    /*
    static const FUID iplug_vieW_content_scale_support_iid;
    */
}

declare_class_iid!{
    IPlugViewContentScaleSupport, 
    0x65ED9690, 
    0x8AC44525, 
    0x8AADEF7A, 
    0x72EA703F
}
