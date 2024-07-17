/*!
  |  AUOutputBase
  |
  |  this is now a mix-in rather than an AUBase
  |  subclass
  */

crate::ix!();

/**
  | additional component entry points
  |
  */
pub trait AUOutputBaseInterface {
    fn start(&mut self) -> OSStatus;
    fn stop(&mut self) -> OSStatus;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUOutputBase.h]

pub struct AUOutputBase<'a> {
    au_base_instance: &'a mut AUBase,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUOutputBase.cpp]
impl<'a> AUOutputBase<'a> {

    pub fn new(in_base: *mut AUBase) -> Self {
    
        todo!();
        /*


            : mAUBaseInstance(*inBase)
        */
    }

    /**
       component dispatcher
      */
    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    pub fn component_entry_dispatch(&mut self, 
        params: *mut ComponentParameters,
        this:   *mut AUOutputBase) -> OSStatus {
        
        todo!();
        /*
            if (This == NULL) return paramErr;

        OSStatus result;

        switch (params->what) {
        case kAudioOutputUnitStartSelect:
            {
                result = This->Start();
            }
            break;

        case kAudioOutputUnitStopSelect:
            {
                result = This->Stop();
            }
            break;

        default:
            result = badComponentSelector;
            break;
        }

        return result;
        */
    }
}
