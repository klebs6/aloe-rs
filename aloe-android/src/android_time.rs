crate::ix!();

pub fn aloe_milliseconds_since_startup() -> u32 {
    
    todo!();
    /*
        timespec t;
        clock_gettime (CLOCK_MONOTONIC, &t);

        return static_cast<uint32> (t.tv_sec) * 1000U + static_cast<uint32> (t.tv_nsec) / 1000000U;
    */
}

#[cfg(target_os="android")]
impl Time {
    
    pub fn get_high_resolution_ticks(&mut self) -> i64 {
        
        todo!();
        /*
            timespec t;
        clock_gettime (CLOCK_MONOTONIC, &t);

        return (t.tv_sec * (int64) 1000000) + (t.tv_nsec / 1000);
        */
    }
    
    pub fn get_high_resolution_ticks_per_second(&mut self) -> i64 {
        
        todo!();
        /*
            return 1000000;  // (microseconds)
        */
    }
    
    pub fn get_millisecond_counter_hi_res(&mut self) -> f64 {
        
        todo!();
        /*
            return (double) getHighResolutionTicks() * 0.001;
        */
    }
    
    pub fn set_system_time_to_this_time(&self) -> bool {
        
        todo!();
        /*
            jassertfalse;
        return false;
        */
    }
}
