crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/MonotonicCounter.h]

/**
  | Maintain a 64-bit monotonic counter.
  | 
  | Can be used to track a 32-bit counter
  | that wraps or gets reset.
  | 
  | -----------
  | @note
  | 
  | this is not atomic and has no interior
  | locks. A caller will need to provide
  | their own exterior locking if they need
  | to use it from multiple threads.
  |
  */
#[derive(Default)]
pub struct MonotonicCounter {
    counter64: i64, // default = 0
    counter32: i32, // default = 0
}

impl MonotonicCounter {

    /**
      | @return
      | 
      | current value of the counter
      |
      */
    pub fn get(&self) -> i64 {
        
        todo!();
        /*
            return mCounter64;
        */
    }

    /**
      | set the current value of the counter
      |
      */
    pub fn set(&mut self, counter: i64)  {
        
        todo!();
        /*
            mCounter64 = counter;
        */
    }

    /**
      | Advance the counter if delta is positive.
      | 
      | -----------
      | @return
      | 
      | current value of the counter
      |
      */
    pub fn increment(&mut self, delta: i64) -> i64 {
        
        todo!();
        /*
            if (delta > 0) {
                mCounter64 += delta;
            }
            return mCounter64;
        */
    }

    /**
      | Advance the 64-bit counter if (current32
      | - previousCurrent32) > 0.
      | 
      | This can be used to convert a 32-bit counter
      | that may be wrapping into a monotonic
      | 64-bit counter.
      | 
      | This counter32 should NOT be allowed
      | to advance by more than 0x7FFFFFFF between
      | calls.
      | 
      | Think of the wrapping counter like a
      | sine wave. If the frequency of the signal
      | is more than half the sampling rate (Nyquist
      | rate) then you cannot measure it properly.
      | 
      | If the counter wraps around every 24
      | hours then we should measure it with
      | a period of less than 12 hours.
      | 
      | -----------
      | @return
      | 
      | current value of the 64-bit counter
      |
      */
    pub fn update32(&mut self, counter32: i32) -> i64 {
        
        todo!();
        /*
            int32_t delta = counter32 - mCounter32;
            // protect against the mCounter64 going backwards
            if (delta > 0) {
                mCounter64 += delta;
                mCounter32 = counter32;
            }
            return mCounter64;
        */
    }

    /**
      | Reset the stored value of the 32-bit
      | counter.
      | 
      | This is used if your counter32 has been
      | reset to zero.
      |
      */
    pub fn reset32(&mut self)  {
        
        todo!();
        /*
            mCounter32 = 0;
        */
    }

    /**
      | Round 64-bit counter up to a multiple
      | of the period.
      | 
      | The period must be positive.
      | 
      | -----------
      | @param period
      | 
      | might be, for example, a buffer capacity
      |
      */
    pub fn round_up64(&mut self, period: i32)  {
        
        todo!();
        /*
            if (period > 0) {
                int64_t numPeriods = (mCounter64 + period - 1) / period;
                mCounter64 = numPeriods * period;
            }
        */
    }
}
