crate::ix!();

/** 
  | This class measures the time between its
  | construction and destruction and adds it to
  | an AudioProcessLoadMeasurer.
  |
  | e.g.
  | @code
  | {
  | AudioProcessLoadMeasurer::AudioProcessLoadMeasurerScopedTimer timer (myProcessLoadMeasurer);
  | myCallback->doTheCallback();
  | }
  | @endcode
  |
  | @tags{Audio}
  */
#[no_copy]
pub struct AudioProcessLoadMeasurerScopedTimer<'a>
{
    owner:      &'a mut AudioProcessLoadMeasurer,
    start_time: f64,
}

impl<'a> Drop for AudioProcessLoadMeasurerScopedTimer<'a> {

    fn drop(&mut self) {

        todo!();
        /* 
        owner.registerBlockRenderTime (Time::getMillisecondCounterHiRes() - startTime);
         */
    }
}

impl<'a> AudioProcessLoadMeasurerScopedTimer<'a> {
    
    pub fn new(p: &mut AudioProcessLoadMeasurer) -> Self {
    
        todo!();
        /*
            : owner (p), startTime (Time::getMillisecondCounterHiRes())
        */
    }
}

