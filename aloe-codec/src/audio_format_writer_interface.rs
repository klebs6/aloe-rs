crate::ix!();

pub trait AudioFormatWriterInterface {

    /**
      | Writes a set of samples to the audio stream.
      | 
      | -----------
      | @note
      | 
      | if you're trying to write the contents
      | of an AudioBuffer, you can use writeFromAudioSampleBuffer().
      | 
      | -----------
      | @param samplesToWrite
      | 
      | an array of arrays containing the sample
      | data for each channel to write. This
      | is a zero-terminated array of arrays,
      | and can contain a different number of
      | channels than the actual stream uses,
      | and the writer should do its best to cope
      | with this.
      | 
      | If the format is fixed-point, each channel
      | will be formatted as an array of signed
      | integers using the full 32-bit range
      | -0x80000000 to 0x7fffffff, regardless
      | of the source's bit-depth. If it is a
      | floating-point format, you should
      | treat the arrays as arrays of floats,
      | and just cast it to an (int**) to pass
      | it into the method.
      | ----------
      | @param numSamples
      | 
      | the number of samples to write
      |
      */
    fn write(&mut self, 
        samples_to_write: *const *const i32,
        num_samples:      i32) -> bool;

    /**
      | Some formats may support a flush operation
      | that makes sure the file is in a valid
      | state before carrying on.
      | 
      | If supported, this means that by calling
      | flush periodically when writing data
      | to a large file, then it should still
      | be left in a readable state if your program
      | crashes.
      | 
      | It goes without saying that this method
      | must be called from the same thread that's
      | calling write()!
      | 
      | If the format supports flushing and
      | the operation succeeds, this returns
      | true.
      |
      */
    fn flush(&mut self) -> bool;
}
