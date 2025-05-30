crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/include/oboe/AudioStreamCallback.h]

/**
  | AudioStreamDataCallback defines
  | a callback interface for moving data
  | to/from an audio stream using `onAudioReady`
  | 
  | 2) being alerted when a stream has an
  | error using `onError*` methods
  | 
  | It is used with AudioStreamBuilder::setDataCallback().
  |
  */
pub trait AudioStreamDataCallback {

    /**
      | A buffer is ready for processing.
      | 
      | For an output stream, this function
      | should render and write numFrames of
      | data in the stream's current data format
      | to the audioData buffer.
      | 
      | For an input stream, this function should
      | read and process numFrames of data from
      | the audioData buffer.
      | 
      | The audio data is passed through the
      | buffer. So do NOT call read() or write()
      | on the stream that is making the callback.
      | 
      | Note that numFrames can vary unless
      | AudioStreamBuilder::setFramesPerCallback()
      | is called.
      | 
      | Also note that this callback function
      | should be considered a "real-time"
      | function. It must not do anything that
      | could cause an unbounded delay because
      | that can cause the audio to glitch or
      | pop.
      | 
      | These are things the function should NOT do:
      | <ul>
      | <li>allocate memory using, for example, malloc() or new</li>
      | <li>any file operations such as opening, closing, reading or writing</li>
      | <li>any network operations such as streaming</li>
      | <li>use any mutexes or other synchronization primitives</li>
      | <li>sleep</li>
      | <li>oboeStream->stop(), pause(), flush() or close()</li>
      | <li>oboeStream->read()</li>
      | <li>oboeStream->write()</li>
      | </ul>
      |
      | The following are OK to call from the data callback:
      | <ul>
      | <li>oboeStream->get*()</li>
      | <li>OboeconvertToText()</li>
      | <li>oboeStream->setBufferSizeInFrames()</li>
      | </ul>
      | If you need to move data, eg. MIDI commands,
      | in or out of the callback function then
      | we recommend the use of non-blocking
      | techniques such as an atomic FIFO.
      | 
      | -----------
      | @param audioStream
      | 
      | pointer to the associated stream @param
      | audioData buffer containing input
      | data or a place to put output data @param
      | numFrames number of frames to be processed
      | @return DataCallbackResult::Continue
      | or DataCallbackResult::Stop
      |
      */
    fn on_audio_ready(&mut self, 
            audio_stream: *mut AudioStream,
            audio_data:   *mut c_void,
            num_frames:   i32) -> OboeDataCallbackResult;

}

/**
  | AudioStreamErrorCallback defines
  | a callback interface for being alerted
  | when a stream has an error or is disconnected
  | using `onError*` methods.
  | 
  | It is used with AudioStreamBuilder::setErrorCallback().
  |
  */
pub trait AudioStreamErrorCallback {

    /**
      | This will be called before other `onError`
      | methods when an error occurs on a stream,
      | such as when the stream is disconnected.
      | 
      | It can be used to override and customize
      | the normal error processing. Use of
      | this method is considered an advanced
      | technique. It might, for example, be
      | used if an app want to use a high level
      | lock when closing and reopening a stream.
      | Or it might be used when an app want to
      | signal a management thread that handles
      | all of the stream state.
      | 
      | If this method returns false it indicates
      | that the stream has *not been stopped
      | and closed by the application. In this
      | case it will be stopped by Oboe in the
      | following way: onErrorBeforeClose()
      | will be called, then the stream will
      | be closed and onErrorAfterClose()
      | will be closed.
      | 
      | If this method returns true it indicates
      | that the stream *has* been stopped and
      | closed by the application and Oboe will
      | not do this. In that case, the app MUST
      | stop() and close() the stream.
      | 
      | This method will be called on a thread
      | created by Oboe.
      | 
      | -----------
      | @param audioStream
      | 
      | pointer to the associated stream @param
      | error @return true if the stream has
      | been stopped and closed, false if not
      |
      */
    fn on_error(&mut self, 
        audio_stream: *mut AudioStream,
        error:        OboeResult) -> bool {
        
        todo!();
        /*
            return false;
        */
    }

    /**
      | This will be called when an error occurs
      | on a stream, such as when the stream is
      | disconnected, and if onError() returns
      | false (indicating that the error has
      | not already been handled).
      | 
      | Note that this will be called on a thread
      | created by Oboe.
      | 
      | The underlying stream will already
      | be stopped by Oboe but not yet closed.
      | So the stream can be queried.
      | 
      | Do not close or delete the stream in this
      | method because it will be closed after
      | this method returns.
      | 
      | -----------
      | @param audioStream
      | 
      | pointer to the associated stream @param
      | error
      |
      */
    fn on_error_before_close(&mut self, 
        audio_stream: *mut AudioStream,
        error:        OboeResult)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | This will be called when an error occurs
      | on a stream, such as when the stream is
      | disconnected, and if onError() returns
      | false (indicating that the error has
      | not already been handled).
      | 
      | The underlying AAudio or OpenSL ES stream
      | will already be stopped AND closed by
      | Oboe. So the underlying stream cannot
      | be referenced. But you can still query
      | most parameters.
      | 
      | This callback could be used to reopen
      | a new stream on another device.
      | 
      | -----------
      | @param audioStream
      | 
      | pointer to the associated stream @param
      | error
      |
      */
    fn on_error_after_close(&mut self, 
        audio_stream: *mut AudioStream,
        error:        OboeResult)  {
        
        todo!();
        /*
        
        */
    }
}

/**
  | AudioStreamCallback defines a callback
  | interface for:
  | 
  | 1) moving data to/from an audio stream
  | using `onAudioReady`
  | 
  | 2) being alerted when a stream has an
  | error using `onError*` methods
  | 
  | It is used with AudioStreamBuilder::setCallback().
  | 
  | It combines the interfaces defined
  | by AudioStreamDataCallback and AudioStreamErrorCallback.
  | 
  | This was the original callback object.
  | We now recommend using the individual
  | interfaces and using setDataCallback()
  | and setErrorCallback().
  | 
  | @deprecated Use `AudioStreamDataCallback`
  | and `AudioStreamErrorCallback` instead
  |
  */
pub trait AudioStreamCallback: AudioStreamDataCallback + AudioStreamErrorCallback { }
