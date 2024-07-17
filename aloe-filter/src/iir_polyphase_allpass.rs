crate::ix!();

/**
  | The structure returned by the function
  | designIIRLowpassHalfBandPolyphaseAllpassMethod.
  | 
  | The two first members of this structure
  | directPath and delayedPath are arrays
  | of IIR::Coefficients, made of polyphase
  | second order allpass filters and an
  | additional delay in the second array,
  | that can be used in cascaded filters
  | processed in two parallel paths, which
  | must be summed at the end to get the high
  | order efficient low-pass filtering.
  | The last member is an array with the useful
  | parameters for simulating the structure
  | using any custom processing function.
  |
  */
pub struct IIRPolyphaseAllpassStructure<NumericType>
{
    direct_path:  Vec<IIRCoefficientsPtr<Self>>,
    delayed_path: Vec<IIRCoefficientsPtr<Self>>,
    alpha:        Vec<f64>,
}

impl<NumericType> HasIIRCoefficients for IIRPolyphaseAllpassStructure<NumericType> {

    type Coefficients = IIRCoefficients<NumericType>;
}

impl<NumericType> HasIIRCoefficientsPtr for IIRPolyphaseAllpassStructure<NumericType> { }
