crate::ix!();

pub enum AudioProcessorProcessingPrecision
{
    singlePrecision,
    doublePrecision
}

pub trait GetProcessingPrecision {

    /**
      | Returns the precision-mode of the processor.
      | 
      | Depending on the result of this method
      | you MUST call the corresponding version
      | of processBlock. The default processing
      | precision is single precision. @see
      | setProcessingPrecision, supportsDoublePrecisionProcessing
      |
      */
    fn get_processing_precision(&self) -> AudioProcessorProcessingPrecision;
}

pub trait IsUsingDoublePrecision {

    /**
      | Returns true if the current precision
      | is set to doublePrecision.
      |
      */
    fn is_using_double_precision(&self) -> bool;
}

pub trait SetProcessingPrecision {

    /**
      | Changes the processing precision of
      | the receiver. A client of the AudioProcessor
      | calls this function to indicate which
      | version of processBlock (single or
      | double precision) it intends to call.
      | The client MUST call this function before
      | calling the prepareToPlay method so
      | that the receiver can do any necessary
      | allocations in the prepareToPlay()
      | method. An implementation of prepareToPlay()
      | should call getProcessingPrecision()
      | to determine with which precision it
      | should allocate it's internal buffers.
      | 
      | -----------
      | @note
      | 
      | setting the processing precision to
      | double floating point precision on
      | a receiver which does not support double
      | precision processing (i.e. supportsDoublePrecisionProcessing()
      | returns false) will result in an assertion.
      | 
      | @see getProcessingPrecision, supportsDoublePrecisionProcessing
      |
      */
    fn set_processing_precision(&mut self, new_precision: AudioProcessorProcessingPrecision);
}

pub trait CheckSupportsDoublePrecisionProcessing {

    /**
      | Returns true if the Audio processor
      | supports double precision floating
      | point processing.
      | 
      | The default implementation will always
      | return false.
      | 
      | If you return true here then you must
      | override the double precision versions
      | of processBlock. Additionally, you
      | must call getProcessingPrecision()
      | in your prepareToPlay method to determine
      | the precision with which you need to
      | allocate your internal buffers. @see
      | getProcessingPrecision, setProcessingPrecision
      |
      */
    fn supports_double_precision_processing(&self) -> bool;
}

pub trait IsFloatingPoint {

    /**
      | Returns true if it's a floating-point
      | format, false if it's fixed-point.
      |
      */
    fn is_floating_point(&self) -> bool;
}

pub trait SupportsDoublePrecisionProcessing {

    fn supports_double_precision_processing(&self) -> bool;
}
