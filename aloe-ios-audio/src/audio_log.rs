crate::ix!();

#[cfg(not(ALOE_IOS_AUDIO_LOGGING))]
pub const ALOE_IOS_AUDIO_LOGGING: usize = 0;

#[cfg(ALOE_IOS_AUDIO_LOGGING)]
macro_rules! aloe_ios_audio_log {
    ($x:ident) => {
        /*
                DBG(x)
        */
    }
}

#[cfg(not(ALOE_IOS_AUDIO_LOGGING))]
macro_rules! aloe_ios_audio_log {
    ($x:ident) => { }
}

pub fn log_ns_error(e: *mut NSError)  {
    
    todo!();
    /*
        if (e != nil)
        {
            ALOE_IOS_AUDIO_LOG ("iOS Audio error: " << [e.localizedDescription UTF8String]);
            jassertfalse;
        }
    */
}

macro_rules! aloe_nserror_check {
    ($X:ident) => {
        /*
                { NSError* error = nil; X; logNSError (error); }
        */
    }
}

