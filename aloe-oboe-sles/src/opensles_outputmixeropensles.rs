crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OutputMixerOpenSLES.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/opensles/OutputMixerOpenSLES.cpp]

/**
  | INTERNAL USE ONLY
  |
  */
pub struct OboeOutputMixerOpenSL {
    lock:              parking_lot::RawMutex,
    open_count:        i32, // default = 0
    output_mix_object: SLObjectItf, // default = nullptr
}

impl OboeOutputMixerOpenSL {

    pub fn get_instance(&mut self) -> &mut OboeOutputMixerOpenSL {
        
        todo!();
        /*
            static OboeOutputMixerOpenSL sInstance;
        return sInstance;
        */
    }
    
    pub fn open(&mut self) -> SLresult {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mLock);

        SLresult result = SL_RESULT_SUCCESS;
        if (mOpenCount++ == 0) {
            // get the output mixer
            result = EngineOpenSLES::getInstance().createOutputMix(&mOutputMixObject);
            if (SL_RESULT_SUCCESS != result) {
                LOGE("OboeOutputMixerOpenSL() - createOutputMix() result:%s", getSLErrStr(result));
                goto error;
            }

            // realize the output mix
            result = (*mOutputMixObject)->Realize(mOutputMixObject, SL_BOOLEAN_FALSE);
            if (SL_RESULT_SUCCESS != result) {
                LOGE("OboeOutputMixerOpenSL() - Realize() mOutputMixObject result:%s", getSLErrStr(result));
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
            // destroy output mix object, and invalidate all associated interfaces
            if (mOutputMixObject != nullptr) {
                (*mOutputMixObject)->Destroy(mOutputMixObject);
                mOutputMixObject = nullptr;
            }
        }
        */
    }
    
    pub fn create_audio_player(&mut self, 
        object_itf:   *mut SLObjectItf,
        audio_source: *mut SLDataSource) -> SLresult {
        
        todo!();
        /*
            SLDataLocator_OutputMix loc_outmix = {SL_DATALOCATOR_OUTPUTMIX, mOutputMixObject};
        SLDataSink audioSink = {&loc_outmix, NULL};
        return EngineOpenSLES::getInstance().createAudioPlayer(objectItf, audioSource, &audioSink);
        */
    }
}
