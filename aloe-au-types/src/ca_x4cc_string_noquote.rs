crate::ix!();

pub struct CAX4CCStringNoQuote {
    str_: [u8; 16],
}

impl Into<*mut u8> for CAX4CCStringNoQuote {
    
    #[inline] fn into(self) -> *mut u8 {
        todo!();
        /*
            return mStr;
        */
    }
}

impl CAX4CCStringNoQuote {
    
    pub fn new(error: OSStatus) -> Self {
    
        todo!();
        /*


            // see if it appears to be a 4-char-code
            UInt32 beErr = CFSwapInt32HostToBig(error);
            char *str = mStr;
            memcpy(str, &beErr, 4);
            if (isprint(str[0]) && isprint(str[1]) && isprint(str[2]) && isprint(str[3])) {
                str[4] = '\0';
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
