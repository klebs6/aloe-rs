crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/buffers/aloe_AudioProcessLoadMeasurer.h]

/**
  | Maintains an ongoing measurement of the
  | proportion of time which is being spent inside
  | an audio callback.
  |
  | @tags{Audio}
  */
pub struct AudioProcessLoadMeasurer {
    cpu_usage_ms:      f64, // default = 0
    time_to_cpu_scale: f64, // default = 0
    ms_per_block:      f64, // default = 0
    xruns:             i32, // default = 0
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/buffers/aloe_AudioProcessLoadMeasurer.cpp]
impl AudioProcessLoadMeasurer {

    /**
      | Resets the state.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            reset (0, 0);
        */
    }
    
    /**
      | Resets the counter, in preparation
      | for use with the given sample rate and
      | block size.
      |
      */
    pub fn reset_with_sample_rate_and_block_size(
        &mut self, 
        sample_rate: f64,
        block_size:  i32)  {
        
        todo!();
        /*
            cpuUsageMs = 0;
        xruns = 0;

        if (sampleRate > 0.0 && blockSize > 0)
        {
            msPerBlock = 1000.0 * blockSize / sampleRate;
            timeToCpuScale = (msPerBlock > 0.0) ? (1.0 / msPerBlock) : 0.0;
        }
        else
        {
            msPerBlock = 0;
            timeToCpuScale = 0;
        }
        */
    }
    
    /** 
      | Can be called manually to add the time of
      | a callback to the stats.
      |
      | Normally you probably would never call
      | this - it's simpler and more robust to use
      | a AudioProcessLoadMeasurerScopedTimer to measure the time using an
      | RAII pattern.
      */
    pub fn register_block_render_time(&mut self, milliseconds: f64)  {
        
        todo!();
        /*
            const double filterAmount = 0.2;
        cpuUsageMs += filterAmount * (milliseconds - cpuUsageMs);

        if (milliseconds > msPerBlock)
            ++xruns;
        */
    }
    
    /**
      | Returns the current load as a proportion
      | 0 to 1.0
      |
      */
    pub fn get_load_as_proportion(&self) -> f64 {
        
        todo!();
        /*
            return jlimit (0.0, 1.0, timeToCpuScale * cpuUsageMs);
        */
    }
    
    /**
      | Returns the current load as a percentage
      | 0 to 100.0
      |
      */
    pub fn get_load_as_percentage(&self) -> f64 {
        
        todo!();
        /*
            return 100.0 * getLoadAsProportion();
        */
    }
    
    /**
      | Returns the number of over- (or under-)
      | runs recorded since the state was reset.
      |
      */
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            return xruns;
        */
    }
}
