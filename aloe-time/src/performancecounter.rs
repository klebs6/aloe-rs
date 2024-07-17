crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/time/aloe_PerformanceCounter.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_core/time/aloe_PerformanceCounter.cpp]

/**
  | Simple RAII class for measuring the time spent
  | in a scope.
  |
  | Example:
  |
  | {
  |     double timeSec;
  |
  |     {
  |         ScopedTimeMeasurement m (timeSec);
  |         doSomething();
  |     }
  |
  |     Logger::writeToLog ("doSomething() took " + String (timeSec) + "seconds");
  | }
  |
  | -------------------------
  | @param resultInSeconds 
  |
  | The result of the measurement will be stored
  | in this variable.
  |
  | @tags{Core}
  */
#[no_copy]
#[leak_detector]
pub struct ScopedTimeMeasurement<'a> {
    start_time_ticks: i64, // = Time::getHighResolutionTicks();
    result:           &'a mut f64,
}

impl<'a> Drop for ScopedTimeMeasurement<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            static auto scaler = 1.0 / static_cast<double> (Time::getHighResolutionTicksPerSecond());
            result = static_cast<double> (Time::getHighResolutionTicks() - startTimeTicks) * scaler;
         */
    }
}

impl<'a> ScopedTimeMeasurement<'a> {

    pub fn new(result_in_seconds: &mut f64) -> Self {
    
        todo!();
        /*
        : result(resultInSeconds),

            result = 0.0;
        */
    }
}

///-----------------------------

pub fn time_to_string(secs: f64) -> String {
    
    todo!();
    /*
        return String ((int64) (secs * (secs < 0.01 ? 1000000.0 : 1000.0) + 0.5))
                        + (secs < 0.01 ? " microsecs" : " millisecs");
    */
}
