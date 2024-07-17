crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUSilentTimeout.h]

pub struct AUSilentTimeout {
    timeout_counter: i32,
    reset_timer:     bool,
}

impl Default for AUSilentTimeout {
    
    fn default() -> Self {
        todo!();
        /*
        : timeout_counter(0),
        : reset_timer(true),

            }{
        */
    }
}

impl AUSilentTimeout {

    pub fn process(&mut self, 
        in_frames_to_process: u32,
        in_timeout_limit:     u32,
        io_silence:           &mut bool)  {
        
        todo!();
        /*
            if(ioSilence )
            {
                if(mResetTimer )
                {
                    mTimeoutCounter = inTimeoutLimit;
                    mResetTimer = false;
                }

                if(mTimeoutCounter > 0 )
                {
                    mTimeoutCounter -= inFramesToProcess;
                    ioSilence = false;
                }
            }
            else
            {
                // signal to reset the next time we receive silence
                mResetTimer = true;
            }
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            mResetTimer = true;
        }{
        */
    }
}
