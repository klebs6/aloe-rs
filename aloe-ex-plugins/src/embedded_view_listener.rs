/*!
  | This demo shows how to use the VstCallbackHandler
  | and Vst3ClientExtensions classes
  | to provide extended functionality
  | in compatible Vst/Vst3 hosts.
  | 
  | If this project is built as a Vst or Vst3
  | plugin and loaded in REAPER 6.29 or higher,
  | it will provide an embedded level meter
  | in the track control panel. To enable
  | the embedded view, right-click on the
  | plugin and select "Show embedded UI
  | in TCP".
  | 
  | The plugin's editor also include a button
  | which can be used to toggle all inserts
  | on and off.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/ReaperEmbeddedViewPluginDemo.h]

/**
  | These should live in a file which is guaranteed
  | to be compiled only once (i.e. a .cpp
  | file, normally). This demo is a bit special,
  | because we know that this header will
  | only be included in a single translation
  | unit.
  |
  */
def_class_iid!{
    IReaperHostApplication
}
def_class_iid!{
    IReaperUIEmbedInterface
}

pub trait EmbeddedViewListener {

    fn handled_embedded_ui_message(
        &mut self, 
        msg:   i32,
        parm2: TPtrInt,
        parm3: TPtrInt
    ) -> TPtrInt;
}
