crate::ix!();

///-------------------
#[cfg(target_os="android")]
pub struct OpenSLQueueRunnerPlayer<'a, T> {

    base: OpenSLQueueRunner<
        'a, 
        T,
        OpenSLQueueRunnerPlayer<'a,T>,
        SLPlayItf_
    >,
}

#[cfg(target_os="android")]
pub trait HasBase {
    type Base;
}

#[cfg(target_os="android")]
impl<'a,T> HasBase for OpenSLQueueRunnerPlayer<'a,T> {
    type Base = OpenSLQueueRunner<'a, T,OpenSLQueueRunnerPlayer<'a, T>,SLPlayItf_>;
}

#[cfg(target_os="android")]
impl<'a,T> OpenSLQueueRunnerPlayer<'a, T> {

    pub fn new(
        session_to_use:      &mut OpenSLSessionT<T>,
        num_channels_to_use: i32) -> Self {
    
        todo!();
        /*
        : base(sessionToUse, numChannelsToUse),
        
        */
    }
    
    pub fn create_player_or_recorder(&mut self) -> SlRef<SLPlayItf_> {
        
        todo!();
        /*
            SLDataLocator_AndroidSimpleBufferQueue queueLocator = { SL_DATALOCATOR_ANDROIDSIMPLEBUFFERQUEUE, static_cast<SLuint32> (Base::owner.numBuffers) };
                SLDataLocator_OutputMix outputMix = { SL_DATALOCATOR_OUTPUTMIX, Base::owner.outputMix };

                PCMDataFormatEx dataFormat;
                BufferHelpers<T>::initPCMDataFormat (dataFormat, Base::numChannels, Base::owner.sampleRate);

                SLDataSource source = { &queueLocator, &dataFormat };
                SLDataSink   sink   = { &outputMix,    nullptr };

                SLInterfaceID queueInterfaces[] = { &IntfIID<SLAndroidSimpleBufferQueueItf_>::iid, &IntfIID<SLAndroidConfigurationItf_>::iid };
                SLboolean interfaceRequired[] = {SL_BOOLEAN_TRUE, SL_BOOLEAN_FALSE};

                SLObjectItf obj = nullptr;

                auto& holder = getEngineHolder();

                if (auto e = *holder.engine)
                {
                    auto status = e->CreateAudioPlayer (holder.engine, &obj, &source, &sink, 2,
                                                        queueInterfaces, interfaceRequired);

                    if (status != SL_RESULT_SUCCESS || obj == nullptr || (*obj)->Realize(obj, 0) != SL_RESULT_SUCCESS)
                    {
                        destroyObject (obj);
                        return {};
                    }
                }

                return SlRef<SLPlayItf_>::cast (SlObjectRef (obj));
        */
    }
    
    pub fn set_state(&mut self, running: bool)  {
        
        todo!();
        /*
            (*Base::runner)->SetPlayState (Base::runner, running ? SL_PLAYSTATE_PLAYING : SL_PLAYSTATE_STOPPED);
        */
    }
}
