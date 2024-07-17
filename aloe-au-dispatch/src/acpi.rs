crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUPlugInDispatch.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUPlugInDispatch.cpp]

macro_rules! acpi {
    () => {
        /*
                ((AudioComponentPlugInInstance *)self)
        */
    }
}

macro_rules! aui {
    () => {
        /*
                ((AUBase *)&ACPI->mInstanceStorage)
        */
    }
}

macro_rules! aui_lock {
    () => {
        /*
                CAMutex::Locker auLock(AUI->GetMutex());
        */
    }
}
