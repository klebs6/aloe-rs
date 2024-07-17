crate::ix!();

pub trait AudioStreamInterface {

    /*
       | Asynchronous requests. Use waitForStateChange()
       | if you need to wait for completion.
       |
       */

    /**
      | Start the stream asynchronously. Returns
      | immediately (does not block). Equivalent
      | to calling `start(0)`.
      |
      */
    fn request_start(&mut self) -> OboeResult;

    /**
      | Pause the stream asynchronously. Returns
      | immediately (does not block). Equivalent
      | to calling `pause(0)`.
      |
      */
    fn request_pause(&mut self) -> OboeResult;

    /**
      | Flush the stream asynchronously. Returns
      | immediately (does not block). Equivalent
      | to calling `flush(0)`.
      |
      */
    fn request_flush(&mut self) -> OboeResult;

    /**
      | Stop the stream asynchronously. Returns
      | immediately (does not block). Equivalent
      | to calling `stop(0)`.
      |
      */
    fn request_stop(&mut self) -> OboeResult;

    /**
      | Query the current state, eg. StreamState::Pausing
      | 
      | 
      | -----------
      | @return
      | 
      | state or a negative error.
      |
      */
    fn get_state(&mut self) -> OboeStreamState;


    /**
      | Wait until the stream's current state
      | no longer matches the input state. The
      | input state is passed to avoid race conditions
      | caused by the state changing between
      | calls.
      | 
      |  <pre><code>
      |  int64_t timeoutNanos = 500 * kNanosPerMillisecond; // arbitrary 1/2 second
      |  StreamState currentState = stream->getState();
      |  StreamState nextState = StreamState::Unknown;
      |  while (result == OboeResult::OK && currentState != StreamState::Paused) {
      |      result = stream->waitForStateChange(
      |                                    currentState, &nextState, timeoutNanos);
      |      currentState = nextState;
      |  }
      |  </code></pre>
      | 
      | -----------
      | @note
      | 
      | generally applications do not need
      | to call this. It is considered an advanced
      | technique and is mostly used for testing.
      | 
      | If the state does not change within the
      | timeout period then it will return ErrorTimeout.
      | This is true even if timeoutNanoseconds
      | is zero.
      | 
      | -----------
      | @param inputState
      | 
      | The state we want to change away from.
      | @param nextState Pointer to a variable
      | that will be set to the new state. @param
      | timeoutNanoseconds The maximum time
      | to wait in nanoseconds. @return OboeResult::OK
      | or a OboeResult::Error.
      |
      */
    fn wait_for_state_change(&mut self, 
        input_state:         OboeStreamState,
        next_state:          *mut OboeStreamState,
        timeout_nanoseconds: i64) -> OboeResult;

    /**
      | @return
      | 
      | true if XRun counts are supported on
      | the stream
      |
      */
    fn isx_run_count_supported(&self) -> bool;

    /**
      | Get the underlying audio API which the
      | stream uses.
      | 
      | -----------
      | @return
      | 
      | the API that this stream uses.
      |
      */
    fn get_audio_api(&self) -> OboeAudioApi;

    /**
      | Update mFramesWritten. For internal
      | use only.
      |
      */
    fn update_frames_written(&mut self);

    /**
      | Update mFramesRead. For internal use
      | only.
      |
      */
    fn update_frames_read(&mut self);
}
