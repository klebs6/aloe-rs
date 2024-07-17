crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_android_OpenSL.cpp]

#[cfg(target_os="android")]
pub fn aloe_create_realtime_audio_thread(
        entry:    fn(_0: *mut c_void) -> *mut c_void,
        user_ptr: *mut c_void) -> pthread_t {
    
    todo!();
    /*
        auto thread = std::make_unique<SLRealtimeThread>();

        if (! thread->isOk())
            return {};

        auto threadID = thread->startThread (entry, userPtr);

        // the thread will de-allocate itself
        thread.release();

        return threadID;
    */
}

#[cfg(target_os="android")]
pub struct SLRealtimeThread {
    sl_library:         DynamicLibrary, // { "libOpenSLES.so" };
    engine:             SlRef<SLEngineItf_>,
    output_mix:         SlRef<SLOutputMixItf_>,
    player:             SlRef<SLPlayItf_>,
    queue:              SlRef<SLAndroidSimpleBufferQueueItf_>,
    buffer_size:        i32, // = AndroidHighPerformanceAudioHelpers::getNativeBufferSizeHint();
    buffer:             HeapBlock<i16>, // { HeapBlock<int16> (static_cast<size_t> (1 * bufferSize * numBuffers)) };
    threadEntryProc:    fn(_0: *mut c_void) -> *mut c_void,
    thread_user_ptr:    *mut c_void,
    thread_ready:       pthread_cond_t,
    thread_ready_mutex: pthread_mutex_t,
    threadid:           pthread_t,
}

#[cfg(target_os="android")]
pub const SL_REALTIME_THREAD_NUM_BUFFERS: i32 = 4;

#[cfg(target_os="android")]
impl Default for SLRealtimeThread {
    
    fn default() -> Self {
        todo!();
        /*


            if (auto createEngine = (CreateEngineFunc) slLibrary.getFunction ("slCreateEngine"))
            {
                SLObjectItf obj = nullptr;
                auto err = createEngine (&obj, 0, nullptr, 0, nullptr, nullptr);

                if (err != SL_RESULT_SUCCESS || obj == nullptr || *obj == nullptr)
                    return;

                if ((*obj)->Realize (obj, 0) != SL_RESULT_SUCCESS)
                {
                    destroyObject (obj);
                    return;
                }

                engine = SlRef<SLEngineItf_>::cast (SlObjectRef (obj));

                if (engine == nullptr)
                {
                    destroyObject (obj);
                    return;
                }

                obj = nullptr;
                err = (*engine)->CreateOutputMix (engine, &obj, 0, nullptr, nullptr);

                if (err != SL_RESULT_SUCCESS || obj == nullptr || (*obj)->Realize (obj, 0) != SL_RESULT_SUCCESS)
                {
                    destroyObject (obj);
                    return;
                }

                outputMix = SlRef<SLOutputMixItf_>::cast (SlObjectRef (obj));

                if (outputMix == nullptr)
                {
                    destroyObject (obj);
                    return;
                }

                SLDataLocator_AndroidSimpleBufferQueue queueLocator = {SL_DATALOCATOR_ANDROIDSIMPLEBUFFERQUEUE, static_cast<SLuint32> (numBuffers)};
                SLDataLocator_OutputMix outputMixLocator = {SL_DATALOCATOR_OUTPUTMIX, outputMix};

                PCMDataFormatEx dataFormat;
                BufferHelpers<int16>::initPCMDataFormat (dataFormat, 1, AndroidHighPerformanceAudioHelpers::getNativeSampleRate());

                SLDataSource source = { &queueLocator, &dataFormat };
                SLDataSink   sink   = { &outputMixLocator, nullptr };

                SLInterfaceID queueInterfaces[] = { &IntfIID<SLAndroidSimpleBufferQueueItf_>::iid };
                SLboolean trueFlag = SL_BOOLEAN_TRUE;

                obj = nullptr;
                err = (*engine)->CreateAudioPlayer (engine, &obj, &source, &sink, 1, queueInterfaces, &trueFlag);

                if (err != SL_RESULT_SUCCESS || obj == nullptr)
                    return;

                if ((*obj)->Realize (obj, 0) != SL_RESULT_SUCCESS)
                {
                    destroyObject (obj);
                    return;
                }

                player = SlRef<SLPlayItf_>::cast (SlObjectRef (obj));

                if (player == nullptr)
                {
                    destroyObject (obj);
                    return;
                }

                queue = SlRef<SLAndroidSimpleBufferQueueItf_>::cast (player);
                if (queue == nullptr)
                    return;

                if ((*queue)->RegisterCallback (queue, staticFinished, this) != SL_RESULT_SUCCESS)
                {
                    queue = nullptr;
                    return;
                }

                pthread_cond_init (&threadReady, nullptr);
                pthread_mutex_init (&threadReadyMutex, nullptr);
            
        */
    }
}

#[cfg(target_os="android")]
impl SLRealtimeThread {

    pub fn is_ok(&self) -> bool {
        
        todo!();
        /*
            return queue != nullptr;
        */
    }
    
    pub fn start_thread(
        &mut self, 
        entry:    fn(_0: *mut c_void) -> *mut c_void,
        user_ptr: *mut c_void

    ) -> pthread_t {
        
        todo!();
        /*
            memset (buffer.get(), 0, static_cast<size_t> (sizeof (int16) * static_cast<size_t> (bufferSize * numBuffers)));

            for (int i = 0; i < numBuffers; ++i)
            {
                int16* dst = buffer.get() + (bufferSize * i);
                (*queue)->Enqueue (queue, dst, static_cast<SLuint32> (static_cast<size_t> (bufferSize) * sizeof (int16)));
            }

            pthread_mutex_lock (&threadReadyMutex);

            threadEntryProc = entry;
            threadUserPtr  = userPtr;

            (*player)->SetPlayState (player, SL_PLAYSTATE_PLAYING);

            pthread_cond_wait (&threadReady, &threadReadyMutex);
            pthread_mutex_unlock (&threadReadyMutex);

            return threadID;
        */
    }
    
    pub fn finished(&mut self)  {
        
        todo!();
        /*
            if (threadEntryProc != nullptr)
            {
                pthread_mutex_lock (&threadReadyMutex);

                threadID = pthread_self();

                pthread_cond_signal (&threadReady);
                pthread_mutex_unlock (&threadReadyMutex);

                threadEntryProc (threadUserPtr);
                threadEntryProc = nullptr;

                (*player)->SetPlayState (player, SL_PLAYSTATE_STOPPED);
                MessageManager::callAsync ([this]() { delete this; });
            }
        */
    }
    
    pub fn static_finished(
        _0:      SLAndroidSimpleBufferQueueItf,
        context: *mut c_void)  {
        
        todo!();
        /*
            static_cast<SLRealtimeThread*> (context)->finished();
        */
    }
}
