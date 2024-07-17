crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAXException.h]

pub struct CAX4CCString {
    str_: [u8; 16],
}

impl Into<*mut u8> for CAX4CCString {
    
    #[inline] fn into(self) -> *mut u8 {
        todo!();
        /*
            return mStr;
        */
    }
}

impl CAX4CCString {

    pub fn new(error: OSStatus) -> Self {
    
        todo!();
        /*


            // see if it appears to be a 4-char-code
            UInt32 beErr = CFSwapInt32HostToBig(error);
            char *str = mStr;
            memcpy(str + 1, &beErr, 4);
            if (isprint(str[1]) && isprint(str[2]) && isprint(str[3]) && isprint(str[4])) {
                str[0] = str[5] = '\'';
                str[6] = '\0';
            } else if (error > -200000 && error < 200000)
                // no, format it as an integer
                snprintf(str, sizeof(mStr), "%d", (int)error);
            else
                snprintf(str, sizeof(mStr), "0x%x", (int)error);
        */
    }
    
    pub fn get(&self) -> *const u8 {
        
        todo!();
        /*
            return mStr;
        */
    }
}
