crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OboeEngineOpenSLES.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OboeEngineOpenSLES.cpp]

/**
  | INTERNAL USE ONLY
  |
  */
pub struct OboeEngineOpenSLES {
    lock:             parking_lot::RawMutex,
    open_count:       i32,         // default = 0
    engine_object:    SLObjectItf, // default = nullptr
    engine_interface: SLEngineItf, // default = nullptr
}

impl OboeEngineOpenSLES {

    pub fn get_instance(&mut self) -> &mut OboeEngineOpenSLES {
        
        todo!();
        /*
            static OboeEngineOpenSLES sInstance;
        return sInstance;
        */
    }
    
    pub fn open(&mut self) -> SLresult {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mLock);

        SLresult result = SL_RESULT_SUCCESS;
        if (mOpenCount++ == 0) {

            // create engine
            result = slCreateEngine(&mEngineObject, 0, NULL, 0, NULL, NULL);
            if (SL_RESULT_SUCCESS != result) {
                LOGE("OboeEngineOpenSLES - slCreateEngine() result:%s", getSLErrStr(result));
                goto error;
            }

            // realize the engine
            result = (*mEngineObject)->Realize(mEngineObject, SL_BOOLEAN_FALSE);
            if (SL_RESULT_SUCCESS != result) {
                LOGE("OboeEngineOpenSLES - Realize() engine result:%s", getSLErrStr(result));
                goto error;
            }

            // get the engine interface, which is needed in order to create other objects
            result = (*mEngineObject)->GetInterface(mEngineObject, SL_IID_ENGINE, &mEngineInterface);
            if (SL_RESULT_SUCCESS != result) {
                LOGE("OboeEngineOpenSLES - GetInterface() engine result:%s", getSLErrStr(result));
                goto error;
            }
        }

        return result;

    error:
        close();
        return result;
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mLock);
        if (--mOpenCount == 0) {
            if (mEngineObject != nullptr) {
                (*mEngineObject)->Destroy(mEngineObject);
                mEngineObject = nullptr;
                mEngineInterface = nullptr;
            }
        }
        */
    }
    
    pub fn create_output_mix(&mut self, object_itf: *mut SLObjectItf) -> SLresult {
        
        todo!();
        /*
            return (*mEngineInterface)->CreateOutputMix(mEngineInterface, objectItf, 0, 0, 0);
        */
    }
    
    pub fn create_audio_player(&mut self, 
        object_itf:   *mut SLObjectItf,
        audio_source: *mut SLDataSource,
        audio_sink:   *mut SLDataSink) -> SLresult {
        
        todo!();
        /*
            const SLInterfaceID ids[] = {SL_IID_BUFFERQUEUE, SL_IID_ANDROIDCONFIGURATION};
        const SLboolean reqs[] = {SL_BOOLEAN_TRUE, SL_BOOLEAN_TRUE};

        return (*mEngineInterface)->CreateAudioPlayer(mEngineInterface, objectItf, audioSource,
                                                      audioSink,
                                                      sizeof(ids) / sizeof(ids[0]), ids, reqs);
        */
    }
    
    pub fn create_audio_recorder(&mut self, 
        object_itf:   *mut SLObjectItf,
        audio_source: *mut SLDataSource,
        audio_sink:   *mut SLDataSink) -> SLresult {
        
        todo!();
        /*
            const SLInterfaceID ids[] = {SL_IID_ANDROIDSIMPLEBUFFERQUEUE, SL_IID_ANDROIDCONFIGURATION };
        const SLboolean reqs[] = {SL_BOOLEAN_TRUE, SL_BOOLEAN_TRUE};

        return (*mEngineInterface)->CreateAudioRecorder(mEngineInterface, objectItf, audioSource,
                                                        audioSink,
                                                        sizeof(ids) / sizeof(ids[0]), ids, reqs);
        */
    }
}
