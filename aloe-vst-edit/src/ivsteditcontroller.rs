crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivsteditcontroller.h]

/**
  | Class Category Name for Controller
  | Component
  |
  */
#[cfg(not(kVstComponentControllerClass))]
pub const VstComponentControllerClass: &'static str = "Component Controller Class";


/**
  | View Types used for IEditController::createView
  |
  */
pub const VIEW_TYPE_EDITOR: &'static str = "editor";
