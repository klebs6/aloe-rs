crate::ix!();

/**
  | Structure used for the state variable
  | filter parameters.
  | 
  | @tags{DSP}
  |
  */
#[derive(Default)]
pub struct StateVariableFilterParameters<NumericType> {

    /**
      | The type of the IIR filter
      |
      */
    ty:   StateVariableFilterType, // default = StateVariableFilterType::lowPass

    g:    NumericType, //= static_cast<NumericType> (std::tan (MathConstants<double>::pi * 200.0 / 44100.0));
    r2:   NumericType, //= static_cast<NumericType> (MathConstants<double>::sqrt2);
    h:    NumericType, //= static_cast<NumericType> (1.0 / (1.0 + R2 * g + g * g));
}

impl<NumericType> ProcessorState for StateVariableFilterParameters<NumericType> {}

/**
  | The Coefficients structure is ref-counted,
  | so this is a handy type that can be used
  | as a pointer to one.
  |
  */
pub type StateVariableFilterParametersPtr<NumericType> = Rc<RefCell<StateVariableFilterParameters<NumericType>>>;

impl<NumericType> StateVariableFilterParameters<NumericType> {

    /**
      | Sets the cutoff frequency and resonance
      | of the IIR filter.
      | 
      | -----------
      | @note
      | 
      | The bandwidth of the resonance increases
      | with the value of the parameter. To have
      | a standard 12 dB/octave filter, the
      | value must be set at 1 / sqrt(2).
      |
      */
    pub fn set_cut_off_frequency<FloatType: num::Float>(
        &mut self, 
        sample_rate: f64,
        frequency:   FloatType,
        resonance:   Option<FloatType>

    ) {

        let resonance = resonance.unwrap_or(FloatType::from(1.0 / std::f64::consts::SQRT_2).unwrap());
        
        todo!();
        /*
            jassert (sampleRate > 0);
                jassert (resonance > NumericType (0));
                jassert (frequency > NumericType (0) && frequency <= NumericType (sampleRate * 0.5));

                g  = static_cast<NumericType> (std::tan (MathConstants<double>::pi * frequency / sampleRate));
                R2 = static_cast<NumericType> (1.0 / resonance);
                h  = static_cast<NumericType> (1.0 / (1.0 + R2 * g + g * g));
        */
    }
    
    pub fn new(o: &StateVariableFilterParameters<NumericType>) -> Self {
    
        todo!();
        /*
        : g(o.g),
        : r2(o.R2),
        : h(o.h),
        */
    }
    
    pub fn assign_from(&mut self, o: &StateVariableFilterParameters<NumericType>) 
        -> &mut StateVariableFilterParameters<NumericType> 
    {
        todo!();

        /*
            g = o.g; R2 = o.R2; h = o.h; return *this;
        */
    }
}
