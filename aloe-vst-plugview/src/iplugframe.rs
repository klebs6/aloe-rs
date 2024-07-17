crate::ix!();

/**
  | Callback interface passed to IPlugView.
  | \ingroup pluginGUI vstIHost vst300
  | 
  | - [host imp]
  | 
  | - [released: 3.0.0]
  | 
  | - [mandatory]
  | 
  | Enables a plug-in to resize the view
  | and cause the host to resize the window.
  |
  */
pub trait IPlugFrame: FUnknown {

    /**
      | Called to inform the host about the resize
      | of a given view.
      | 
      | Afterwards the host has to call IPlugView::onSize
      | ().
      |
      */
    #[PLUGIN_API]
    fn resize_view(&mut self, 
            view:     *mut dyn IPlugView,
            new_size: *mut ViewRect) -> tresult;
}

lazy_static!{
    /*
    static const FUID iplug_frame_iid;
    */
}

declare_class_iid!{
    IPlugFrame, 
    0x367FAF01, 
    0xAFA94693, 
    0x8D4DA2A0, 
    0xED0882A3
}
