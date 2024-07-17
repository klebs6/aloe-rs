crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/AudioClock.h]

pub type ClockId = u32;

/**
  | TODO: Move this class into the public
  | headers because it is useful when calculating
  | stream latency
  |
  */
pub struct AudioClock {

}

impl AudioClock {

    pub fn get_nanoseconds(clock_id: Option<ClockId>) -> i64 {

        let clock_id: ClockId = clock_id.unwrap_or(CLOCK_MONOTONIC);

        todo!();
        /*
            struct timespec time;
            int result = clock_gettime(clockId, &time);
            if (result < 0) {
                return result;
            }
            return (time.tv_sec * kNanosPerSecond) + time.tv_nsec;
        */
    }

    /**
      | Sleep until the specified time.
      | 
      | -----------
      | @param nanoTime
      | 
      | time to wake up
      | ----------
      | @param clockId
      | 
      | CLOCK_MONOTONIC is default
      | 
      | -----------
      | @return
      | 
      | 0 or a negative error, eg. -EINTR
      |
      */
    pub fn sleep_until_nano_time(
        nano_time: i64,
        clock_id:  Option<ClockId>

    ) -> i32 {

        let clock_id: ClockId = clock_id.unwrap_or(CLOCK_MONOTONIC);

        todo!();
        /*
            struct timespec time;
            time.tv_sec = nanoTime / kNanosPerSecond;
            time.tv_nsec = nanoTime - (time.tv_sec * kNanosPerSecond);
            return 0 - clock_nanosleep(clockId, TIMER_ABSTIME, &time, NULL);
        */
    }

    /**
      | Sleep for the specified number of nanoseconds
      | in real-time.
      | 
      | Return immediately with 0 if a negative
      | nanoseconds is specified.
      | 
      | -----------
      | @param nanoseconds
      | 
      | time to sleep
      | ----------
      | @param clockId
      | 
      | CLOCK_REALTIME is default
      | 
      | -----------
      | @return
      | 
      | 0 or a negative error, eg. -EINTR
      |
      */
    pub fn sleep_for_nanos(
        nanoseconds: i64,
        clock_id:    Option<ClockId>

    ) -> i32 {

        let clock_id: ClockId = clock_id.unwrap_or(CLOCK_REALTIME);

        todo!();
        /*
            if (nanoseconds > 0) {
                struct timespec time;
                time.tv_sec = nanoseconds / kNanosPerSecond;
                time.tv_nsec = nanoseconds - (time.tv_sec * kNanosPerSecond);
                return 0 - clock_nanosleep(clockId, 0, &time, NULL);
            }
            return 0;
        */
    }
}
