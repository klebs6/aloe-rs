crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/include/oboe/OboeStabilizedCallback.h]

pub struct OboeStabilizedCallback {
    callback:         *mut dyn AudioStreamCallback, // default = nullptr
    frame_count:      i64, // default = 0
    epoch_time_nanos: i64, // default = 0
    ops_per_nano:     f64, // default = 1
}

impl AudioStreamErrorCallback for OboeStabilizedCallback {

}

impl AudioStreamDataCallback for OboeStabilizedCallback {

    fn on_audio_ready(
        &mut self, 
        _: *mut AudioStream, 
        _: *mut c_void, 
        _: i32
    ) -> OboeDataCallbackResult { todo!() }
}

impl AudioStreamCallback for OboeStabilizedCallback {

}

impl OboeStabilizedCallback {

    pub fn on_error_before_close(&mut self, 
        oboe_stream: *mut AudioStream,
        error:       OboeResult)  {
        
        todo!();
        /*
            return mCallback->onErrorBeforeClose(oboeStream, error);
        */
    }
    
    pub fn on_error_after_close(&mut self, 
        oboe_stream: *mut AudioStream,
        error:       OboeResult)  {
        
        todo!();
        /*
            // Reset all fields now that the stream has been closed
            mFrameCount = 0;
            mEpochTimeNanos = 0;
            mOpsPerNano = 1;
            return mCallback->onErrorAfterClose(oboeStream, error);
        */
    }
}

/**
  | cpu_relax is an architecture specific
  | method of telling the CPU that you don't
  | want it to do much work. asm volatile
  | keeps the compiler from optimising
  | these instructions out.
  |
  */
#[cfg(any(__i386__,__x86_64__))]
macro_rules! cpu_relax {
    () => {
        /*
                asm volatile("rep; nop" ::: "memory");
        */
    }
}

#[cfg(any(__arm__,__mips__))]
macro_rules! cpu_relax {
    () => {
        /*
                asm volatile("":::"memory")
        */
    }
}

#[cfg(__aarch64__)]
macro_rules! cpu_relax {
    () => {
        /*
                asm volatile("yield" ::: "memory")
        */
    }
}
