crate::ix!();

/**
  | Provides a FIFO for an AudioFormatWriter,
  | allowing you to push incoming data into
  | a buffer which will be flushed to disk
  | by a background thread.
  |
  */
pub struct ThreadedWriter<'a, W: Write> {
    buffer: Box<ThreadedWriterBuffer<'a, W>>,
}

impl<'a, W: Write> ThreadedWriter<'a, W> {

    /**
      | Creates a ThreadedWriter for a given
      | writer and a thread.
      | 
      | The writer object which is passed in
      | here will be owned and deleted by the
      | ThreadedWriter when it is no longer
      | needed.
      | 
      | To stop the writer and flush the buffer
      | to disk, simply delete this object.
      |
      */
    pub fn new(
        writer:                *mut AudioFormatWriter<'a, W>,
        background_thread:     &mut TimeSliceThread,
        num_samples_to_buffer: i32) -> Self {
    
        todo!();
        /*
            : buffer (new AudioFormatWriter::ThreadedWriter::ThreadedWriterBuffer (backgroundThread, writer, (int) writer->numChannels, numSamplesToBuffer))
        */
    }
    
    /**
      | Pushes some incoming audio data into
      | the FIFO.
      | 
      | If there's enough free space in the buffer,
      | this will add the data to it,
      | 
      | If the FIFO is too full to accept this
      | many samples, the method will return
      | false - then you could either wait until
      | the background thread has had time to
      | consume some of the buffered data and
      | try again, or you can give up and lost
      | this block.
      | 
      | The data must be an array containing
      | the same number of channels as the
      | 
      | AudioFormatWriter object is using.
      | None of these channels can be null.
      |
      */
    pub fn write(
        &mut self, 
        data:        *const *const f32,
        num_samples: i32
    ) -> bool {
        
        todo!();
        /*
            return buffer->write (data, numSamples);
        */
    }
    
    /**
      | Allows you to specify a callback that
      | this writer should update with the incoming
      | data.
      | 
      | The receiver will be cleared and the
      | writer will begin adding data to it as
      | the data arrives. Pass a null pointer
      | to remove the current receiver.
      | 
      | The object passed-in must not be deleted
      | while this writer is still using it.
      |
      */
    pub fn set_data_receiver(&mut self, receiver: *mut dyn IncomingDataReceiver)  {
        
        todo!();
        /*
            buffer->setDataReceiver (receiver);
        */
    }
    
    /**
      | Sets how many samples should be written
      | before calling the AudioFormatWriter::flush
      | method.
      | 
      | Set this to 0 to disable flushing (this
      | is the default).
      |
      */
    pub fn set_flush_interval(&mut self, num_samples_per_flush: i32)  {
        
        todo!();
        /*
            buffer->setFlushInterval (numSamplesPerFlush);
        */
    }
}
