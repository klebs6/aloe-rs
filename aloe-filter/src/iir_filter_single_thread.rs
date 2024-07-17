crate::ix!();

/**
  | An IIR filter that can perform low, high,
  | or band-pass filtering on an audio signal,
  | with no thread-safety guarantees.
  | 
  | You should use this class if you need
  | an IIR filter, and don't plan to call
  | its member functions from multiple
  | threads at once.
  | 
  | @see IIRFilter, IIRCoefficient, IIRFilterAudioSource
  | 
  | @tags{Audio}
  |
  */
pub struct SingleThreadedIIRFilter {
    base: IIRFilterBase<DummyCriticalSection>,
}
