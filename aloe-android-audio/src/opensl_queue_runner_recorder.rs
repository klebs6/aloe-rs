crate::ix!();

#[cfg(target_os="android")]
pub struct OpenSLQueueRunnerRecorder<'a, T> {
    base: OpenSLQueueRunner<'a,T,OpenSLQueueRunnerRecorder<'a, T>,SLRecordItf_>,
}

#[cfg(target_os="android")]
impl<'a,T> HasBase for OpenSLQueueRunnerRecorder<'a, T> {
    type Base = OpenSLQueueRunner<'a, T,OpenSLQueueRunnerRecorder<'a, T>,SLRecordItf_>;
}

#[cfg(target_os="android")]
impl<'a,T> OpenSLQueueRunnerRecorder<'a, T> {

    pub fn new(
        session_to_use:      &mut OpenSLSessionT<T>,
        num_channels_to_use: i32) -> Self {
    
        todo!();
        /*
        : base(sessionToUse, numChannelsToUse),
        */
    }
    
    pub fn create_player_or_recorder(&mut self) -> SlRef<SLRecordItf_> {
        
        todo!();
        /*
            SLDataLocator_IODevice ioDeviceLocator = { SL_DATALOCATOR_IODEVICE, SL_IODEVICE_AUDIOINPUT, SL_DEFAULTDEVICEID_AUDIOINPUT, nullptr };
                SLDataLocator_AndroidSimpleBufferQueue queueLocator = { SL_DATALOCATOR_ANDROIDSIMPLEBUFFERQUEUE, static_cast<SLuint32> (Base::owner.numBuffers) };

                PCMDataFormatEx dataFormat;
                BufferHelpers<T>::initPCMDataFormat (dataFormat, Base::numChannels, Base::owner.sampleRate);

                SLDataSource source = { &ioDeviceLocator, nullptr };
                SLDataSink   sink   = { &queueLocator,    &dataFormat };

                SLInterfaceID queueInterfaces[] = { &IntfIID<SLAndroidSimpleBufferQueueItf_>::iid, &IntfIID<SLAndroidConfigurationItf_>::iid };
                SLboolean interfaceRequired[] = { SL_BOOLEAN_TRUE, SL_BOOLEAN_FALSE };

                SLObjectItf obj = nullptr;

                auto& holder = getEngineHolder();

                if (auto e = *holder.engine)
                {
                    auto status = e->CreateAudioRecorder (holder.engine, &obj, &source, &sink, 2, queueInterfaces, interfaceRequired);

                    if (status != SL_RESULT_SUCCESS || obj == nullptr || (*obj)->Realize (obj, 0) != SL_RESULT_SUCCESS)
                    {
                        destroyObject (obj);
                        return {};
                    }
                }

                return SlRef<SLRecordItf_>::cast (SlObjectRef (obj));
        */
    }
    
    pub fn set_audio_preprocessing_enabled(&mut self, should_enable: bool) -> bool {
        
        todo!();
        /*
            if (Base::config != nullptr)
                {
                    const bool supportsUnprocessed = (getAndroidSDKVersion() >= 25);
                    const SLuint32 recordingPresetValue
                        = (shouldEnable ? SL_ANDROID_RECORDING_PRESET_GENERIC
                                        : (supportsUnprocessed ? SL_ANDROID_RECORDING_PRESET_UNPROCESSED
                                                               : SL_ANDROID_RECORDING_PRESET_VOICE_RECOGNITION));

                    auto status = (*Base::config)->SetConfiguration (Base::config, SL_ANDROID_KEY_RECORDING_PRESET,
                                                                     &recordingPresetValue, sizeof (recordingPresetValue));

                    return (status == SL_RESULT_SUCCESS);
                }

                return false;
        */
    }
    
    pub fn set_state(&mut self, running: bool)  {
        
        todo!();
        /*
            (*Base::runner)->SetRecordState (Base::runner, running ? SL_RECORDSTATE_RECORDING
                                                                                                     : SL_RECORDSTATE_STOPPED);
        */
    }
}

