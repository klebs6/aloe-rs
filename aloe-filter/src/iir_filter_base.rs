crate::ix!();

/**
  | An IIR filter that can perform low, high,
  | or band-pass filtering on an audio signal.
  | 
  | @see IIRCoefficient, IIRFilterAudioSource
  | 
  | @tags{Audio}
  |
  */
#[leak_detector]
pub struct IIRFilterBase<NumericType> {
    process_lock: RawMutex,
    coefficients: IIRCoefficients<NumericType>,
    v1:           f32,  // default = 0
    v2:           f32,  // default = 0
    active:       bool, // default = false
}

impl<NumericType> Default for IIRFilterBase<NumericType> {
    
    /**
      | Creates a filter.
      | 
      | Initially the filter is inactive, so
      | will have no effect on samples that you
      | process with it. Use the setCoefficients()
      | method to turn it into the type of filter
      | needed.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<NumericType> IIRFilterBase<NumericType> {

    /**
      | Returns the coefficients that this
      | filter is using.
      |
      */
    pub fn get_coefficients(&self) -> IIRCoefficients<NumericType> {
        
        todo!();
        /*
            return coefficients;
        */
    }
    
    /**
      | Creates a copy of another filter.
      |
      */
    pub fn new(other: &IIRFilterBase<NumericType>) -> Self {
    
        todo!();
        /*
        : active(other.active),

            const typename Mutex::ScopedLockType sl (other.processLock);
        coefficients = other.coefficients;
        */
    }
    
    /**
      | Clears the filter so that any incoming
      | data passes through unchanged.
      |
      */
    pub fn make_inactive(&mut self)  {
        
        todo!();
        /*
            const typename Mutex::ScopedLockType sl (processLock);
        active = false;
        */
    }
    
    /**
      | Applies a set of coefficients to this
      | filter.
      |
      */
    pub fn set_coefficients(&mut self, new_coefficients: &IIRCoefficients<NumericType>)  {
        
        todo!();
        /*
            const typename Mutex::ScopedLockType sl (processLock);
        coefficients = newCoefficients;
        active = true;
        */
    }
    
    /**
      | Resets the filter's processing pipeline,
      | ready to start a new stream of data.
      | 
      | Note that this clears the processing
      | state, but the type of filter and its
      | coefficients aren't changed. To put
      | a filter into an inactive state, use
      | the makeInactive() method.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            const typename Mutex::ScopedLockType sl (processLock);
        v1 = v2 = 0.0;
        */
    }
    
    /**
      | Processes a single sample, without
      | any locking or checking.
      | 
      | Use this if you need fast processing
      | of a single value, but be aware that this
      | isn't thread-safe in the way that processSamples()
      | is.
      |
      */
    pub fn process_single_sample_raw(&mut self, in_: f32) -> f32 {
        
        todo!();
        /*
            auto out = coefficients.coefficients[0] * in + v1;

        ALOE_SNAP_TO_ZERO (out);

        v1 = coefficients.coefficients[1] * in - coefficients.coefficients[3] * out + v2;
        v2 = coefficients.coefficients[2] * in - coefficients.coefficients[4] * out;

        return out;
        */
    }
    
    /**
      | Performs the filter operation on the
      | given set of samples.
      |
      */
    pub fn process_samples(
        &mut self, 
        samples:     *mut f32,
        num_samples: i32

    ) {
        
        todo!();
        /*
            const typename Mutex::ScopedLockType sl (processLock);

        if (active)
        {
            auto c0 = coefficients.coefficients[0];
            auto c1 = coefficients.coefficients[1];
            auto c2 = coefficients.coefficients[2];
            auto c3 = coefficients.coefficients[3];
            auto c4 = coefficients.coefficients[4];
            auto lv1 = v1, lv2 = v2;

            for (int i = 0; i < numSamples; ++i)
            {
                auto in = samples[i];
                auto out = c0 * in + lv1;
                samples[i] = out;

                lv1 = c1 * in - c3 * out + lv2;
                lv2 = c2 * in - c4 * out;
            }

            ALOE_SNAP_TO_ZERO (lv1);  v1 = lv1;
            ALOE_SNAP_TO_ZERO (lv2);  v2 = lv2;
        }
        */
    }
}
