crate::ix!();

pub struct OboeRealtimeThread {
    base:               OboeAudioStreamCallback,
    threadEntryProc:    fn(_0: *mut c_void) -> *mut c_void,
    thread_user_ptr:    *mut c_void, // default = nullptr
    thread_ready:       pthread_cond_t,
    thread_ready_mutex: pthread_mutex_t,
    parent_threadid:    pthread_t,
    realtime_threadid:  pthread_t,
    test_stream:        Box<OboeStream>,
    format_used:        OboeAudioFormat,
}

pub type OboeRealtimeThreadOboeStream = OboeAudioIODeviceOboeStream;

impl Default for OboeRealtimeThread {
    
    fn default() -> Self {
        todo!();
        /*


            : testStream (new OboeStream (OboekUnspecified,
                                          OboeDirection::Output,
                                          OboeSharingMode::Exclusive,
                                          1,
                                          OboeAudioFormat::Float,
                                          (int) AndroidHighPerformanceAudioHelpers::getNativeSampleRate(),
                                          OboeAudioIODevice::getNativeBufferSize(),
                                          this)),
              formatUsed (OboeAudioFormat::Float)

            // Fallback to I16 stream format if Float has not worked
            if (! testStream->openedOk())
            {
                testStream.reset (new OboeStream (OboekUnspecified,
                                                  OboeDirection::Output,
                                                  OboeSharingMode::Exclusive,
                                                  1,
                                                  OboeAudioFormat::I16,
                                                  (int) AndroidHighPerformanceAudioHelpers::getNativeSampleRate(),
                                                  OboeAudioIODevice::getNativeBufferSize(),
                                                  this));

                formatUsed = OboeAudioFormat::I16;
            }

            parentThreadID = pthread_self();

            pthread_cond_init (&threadReady, nullptr);
            pthread_mutex_init (&threadReadyMutex, nullptr)
        */
    }
}

impl OboeRealtimeThread {

    pub fn is_ok(&self) -> bool {
        
        todo!();
        /*
            return testStream != nullptr && testStream->openedOk();
        */
    }
    
    pub fn start_thread(
        &mut self, 
        entry:    fn(_0: *mut c_void) -> *mut c_void,
        user_ptr: *mut c_void

    ) -> pthread_t {
        
        todo!();
        /*
            pthread_mutex_lock (&threadReadyMutex);

            threadEntryProc = entry;
            threadUserPtr  = userPtr;

            testStream->start();

            pthread_cond_wait (&threadReady, &threadReadyMutex);
            pthread_mutex_unlock (&threadReadyMutex);

            return realtimeThreadID;
        */
    }
    
    pub fn on_audio_ready(&mut self, 
        _0: *mut OboeAudioStream,
        _1: *mut c_void,
        _2: i32) -> OboeDataCallbackResult {
        
        todo!();
        /*
            // When running with OpenSL, the first callback will come on the parent thread.
            if (threadEntryProc != nullptr && ! pthread_equal (parentThreadID, pthread_self()))
            {
                pthread_mutex_lock (&threadReadyMutex);

                realtimeThreadID = pthread_self();

                pthread_cond_signal (&threadReady);
                pthread_mutex_unlock (&threadReadyMutex);

                threadEntryProc (threadUserPtr);
                threadEntryProc = nullptr;

                MessageManager::callAsync ([this]() { delete this; });

                return OboeDataCallbackResult::Stop;
            }

            return OboeDataCallbackResult::Continue;
        */
    }
    
    pub fn on_error_before_close(&mut self, 
        _0:    *mut OboeAudioStream,
        error: OboeResult)  {
        
        todo!();
        /*
            ALOE_OBOE_LOG ("OboeRealtimeThread: Oboe stream onErrorBeforeClose(): " + getOboeString (error));
            ignoreUnused (error);
            jassertfalse;  // Should never get here!
        */
    }
    
    pub fn on_error_after_close(&mut self, 
        _0:    *mut OboeAudioStream,
        error: OboeResult)  {
        
        todo!();
        /*
            ALOE_OBOE_LOG ("OboeRealtimeThread: Oboe stream onErrorAfterClose(): " + getOboeString (error));
            ignoreUnused (error);
            jassertfalse;  // Should never get here!
        */
    }
}

pub fn aloe_create_realtime_audio_thread(
    entry:    fn(_0: *mut c_void) -> *mut c_void,
    user_ptr: *mut c_void
) -> pthread_t {
    
    todo!();
    /*
        auto thread = std::make_unique<OboeRealtimeThread>();

        if (! thread->isOk())
            return {};

        auto threadID = thread->startThread (entry, userPtr);

        // the thread will de-allocate itself
        thread.release();

        return threadID;
    */
}
