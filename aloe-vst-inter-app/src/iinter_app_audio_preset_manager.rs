crate::ix!();

/**
  | Extended plug-in interface IEditController
  | for Inter-App Audio Preset Management
  | \ingroup vstIPlug vst360
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.6.0]
  |
  */
pub trait IInterAppAudioPresetManager: FUnknown {

    /**
      | Open the Preset Browser in order to load
      | a preset
      |
      */
    #[PLUGIN_API]
    fn run_load_preset_browser(&mut self) -> tresult;

    /**
      | Open the Preset Browser in order to save
      | a preset
      |
      */
    #[PLUGIN_API]
    fn run_save_preset_browser(&mut self) -> tresult;

    /**
      | Load the next available preset
      |
      */
    #[PLUGIN_API]
    fn load_next_preset(&mut self) -> tresult;

    /**
      | Load the previous available preset
      |
      */
    #[PLUGIN_API]
    fn load_previous_preset(&mut self) -> tresult;
}

lazy_static!{
    /*
    static const FUID iinter_app_audio_preset_manager_iid;
    */
}

declare_class_iid!{
    IInterAppAudioPresetManager, 
    0xADE6FCC4, 
    0x46C94E1D, 
    0xB3B49A80, 
    0xC93FEFDD
}
