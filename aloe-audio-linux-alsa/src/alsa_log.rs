crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_linux_ALSA.cpp]

#[cfg(not(ALOE_ALSA_LOGGING))]
pub const ALOE_ALSA_LOGGING: usize = 0;

#[cfg(ALOE_ALSA_LOGGING)]
macro_rules! aloe_alsa_log {
    ($dbgtext:ident) => {
        /*
                { String tempDbgBuf ("ALSA: "); tempDbgBuf << dbgtext; Logger::writeToLog (tempDbgBuf); DBG (tempDbgBuf); }
        */
    }
}

#[cfg(ALOE_ALSA_LOGGING)]
macro_rules! aloe_checked_result {
    ($x:ident) => {
        /*
                (logErrorMessage (x, __LINE__))
        */
    }
}

#[cfg(ALOE_ALSA_LOGGING)]
pub fn log_error_message(
        err:      i32,
        line_num: i32) -> i32 {
    
    todo!();
    /*
        if (err < 0)
            ALOE_ALSA_LOG ("Error: line " << lineNum << ": code " << err << " (" << snd_strerror (err) << ")");

        return err;
    */
}

#[cfg(ALOE_ALSA_LOGGING)]
macro_rules! aloe_alsa_log {
    ($x:ident) => {
        /*
                {}
        */
    }
}

#[cfg(ALOE_ALSA_LOGGING)]
macro_rules! aloe_checked_result {
    ($x:ident) => {
        /*
                (x)
        */
    }
}

macro_rules! aloe_alsa_failed {
    ($x:ident) => {
        /*
                failed (x)
        */
    }
}

pub fn ensure_minimum_num_bits_set(
        chans:         &mut BigInteger,
        min_num_chans: i32)  {
    
    todo!();
    /*
        int i = 0;

        while (chans.countNumberOfSetBits() < minNumChans)
            chans.setBit (i++);
    */
}

pub fn silent_error_handler(
        _0:   *const u8,
        _1:   i32,
        _2:   *const u8,
        _3:   i32,
        _4:   *const u8,
        args: &[&str])  {
    
}

pub fn create_audio_io_device_type_alsa_soundcards() -> *mut AudioIODeviceType {
    
    todo!();
    /*
        return new ALSAAudioIODeviceType (true, "ALSA HW");
    */
}

pub fn create_audio_io_device_type_alsa_pcm_devices() -> *mut AudioIODeviceType {
    
    todo!();
    /*
        return new ALSAAudioIODeviceType (false, "ALSA");
    */
}

