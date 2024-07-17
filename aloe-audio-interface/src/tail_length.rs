crate::ix!();

pub trait GetTailLengthSeconds {

    /**
      | Returns the length of the processor's
      | tail, in seconds.
      |
      */
    fn get_tail_length_seconds(&self) -> f64;
}
