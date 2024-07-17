crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_Phase.h]

/**
  | Represents an increasing phase value
  | between 0 and 2*pi.
  | 
  | This represents a value which can be
  | incremented, and which wraps back to
  | 0 when it goes past 2 * pi.
  | 
  | @tags{DSP}
  |
  */
pub struct Phase<Type> {
    phase: Type, // default = 0
}

impl<Type> Phase<Type> {

    /**
      | Resets the phase to 0.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            phase = 0;
        */
    }

    /**
      | Returns the current value, and increments
      | the phase by the given increment.
      | 
      | The increment must be a positive value,
      | it can't go backwards!
      | 
      | The new value of the phase after calling
      | this function will be (phase + increment)
      | % (2 * pi).
      |
      */
    pub fn advance(&mut self, increment: Type) -> Type {
        
        todo!();
        /*
            jassert (increment >= 0); // cannot run this value backwards!

            auto last = phase;
            auto next = last + increment;

            while (next >= MathConstants<Type>::twoPi)
                next -= MathConstants<Type>::twoPi;

            phase = next;
            return last;
        */
    }
}
