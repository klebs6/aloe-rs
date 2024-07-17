crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_android_Oboe.cpp]

#[cfg(not(ALOE_OBOE_LOG_ENABLED))]
pub const ALOE_OBOE_LOG_ENABLED: usize = 1;

#[cfg(ALOE_OBOE_LOG_ENABLED)]
macro_rules! aloe_oboe_log {
    ($x:ident) => {
        /*
                DBG(x)
        */
    }
}

#[cfg(ALOE_OBOE_LOG_ENABLED)]
macro_rules! aloe_oboe_log {
    ($x:ident) => {
        /*
                {}
        */
    }
}

pub fn get_oboe_string<Type>(value: &Type) -> String {

    todo!();
    /*
        return String (OboeconvertToText (value));
    */
}
