crate::ix!();

#[cfg(target_os="android")]
pub struct OpenSLQueueRunner<'a, T,ChildType,RunnerObjectType> {
    owner:          &'a mut OpenSLSessionT<'a,T>,
    runner:         SlRef<RunnerObjectType>,
    queue:          SlRef<SLAndroidSimpleBufferQueueItf_>,
    config:         SlRef<SLAndroidConfigurationItf_>,
    java_proxy:     GlobalRef,
    num_channels:   i32,
    native_buffer:  HeapBlock<T>,
    scratch_buffer: AudioBuffer<f32>,
    sample_buffer:  AudioBuffer<f32>,
    next_block:     Atomic<i32>, // default = 0 
    num_blocks_out: Atomic<i32>, // default = 0 
    _0:             PhantomData<ChildType>,
}

#[cfg(target_os="android")]
impl<'a,T,ChildType,RunnerObjectType> Drop for OpenSLQueueRunner<'a,T,ChildType,RunnerObjectType> {

    fn drop(&mut self) {
        todo!();
        /* 
                if (config != nullptr && javaProxy != nullptr)
                {
                    javaProxy.clear();
                    (*config)->ReleaseJavaProxy (config, /*SL_ANDROID_JAVA_PROXY_ROUTING*/1);
                }
             */
    }
}

#[cfg(target_os="android")]
impl<'a,T,ChildType,RunnerObjectType> OpenSLQueueRunner<'a,T,ChildType,RunnerObjectType> {

    pub fn new(
        session_to_use:      &mut OpenSLSessionT<T>,
        num_channels_to_use: i32) -> Self {
    
        todo!();
        /*


            : owner (sessionToUse),
                  numChannels (numChannelsToUse),
                  nativeBuffer (static_cast<size_t> (numChannels * owner.bufferSize * owner.numBuffers)),
                  scratchBuffer (numChannelsToUse, owner.bufferSize),
                  sampleBuffer (scratchBuffer.getArrayOfWritePointers(), numChannelsToUse, owner.bufferSize)
        */
    }
    
    pub fn init(&mut self) -> bool {
        
        todo!();
        /*
            runner = crtp().createPlayerOrRecorder();

                if (runner == nullptr)
                    return false;

                const bool supportsJavaProxy = (getAndroidSDKVersion() >= 24);

                if (supportsJavaProxy)
                {
                    // may return nullptr on some platforms - that's ok
                    config = SlRef<SLAndroidConfigurationItf_>::cast (runner);

                    if (config != nullptr)
                    {
                        jobject audioRoutingJni;
                        auto status = (*config)->AcquireJavaProxy (config, /*SL_ANDROID_JAVA_PROXY_ROUTING*/1,
                                                                   &audioRoutingJni);

                        if (status == SL_RESULT_SUCCESS && audioRoutingJni != nullptr)
                            javaProxy = GlobalRef (LocalRef<jobject>(getEnv()->NewLocalRef (audioRoutingJni)));
                    }
                }

                queue = SlRef<SLAndroidSimpleBufferQueueItf_>::cast (runner);

                if (queue == nullptr)
                    return false;

                return ((*queue)->RegisterCallback (queue, staticFinished, this) == SL_RESULT_SUCCESS);
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            nextBlock.set (0);
                numBlocksOut.set (0);

                zeromem (nativeBuffer.get(), static_cast<size_t> (owner.bufferSize * numChannels * owner.numBuffers) * sizeof (T));
                scratchBuffer.clear();
                (*queue)->Clear (queue);
        */
    }
    
    pub fn enqueue_buffer(&mut self)  {
        
        todo!();
        /*
            (*queue)->Enqueue (queue, getCurrentBuffer(), static_cast<SLuint32> (getBufferSizeInSamples() * sizeof (T)));
                ++numBlocksOut;
        */
    }
    
    pub fn is_buffer_available(&self) -> bool {
        
        todo!();
        /*
            return (numBlocksOut.get() < owner.numBuffers);
        */
    }
    
    pub fn get_next_buffer(&mut self) -> *mut T {
        
        todo!();
        /*
            nextBlock.set((nextBlock.get() + 1) % owner.numBuffers); return getCurrentBuffer();
        */
    }
    
    pub fn get_current_buffer(&mut self) -> *mut T {
        
        todo!();
        /*
            return nativeBuffer.get() + (static_cast<size_t> (nextBlock.get()) * getBufferSizeInSamples());
        */
    }
    
    pub fn get_buffer_size_in_samples(&self) -> usize {
        
        todo!();
        /*
            return static_cast<size_t> (owner.bufferSize * numChannels);
        */
    }
    
    pub fn finished(&mut self, _0: SLAndroidSimpleBufferQueueItf)  {
        
        todo!();
        /*
            --numBlocksOut;
                owner.doSomeWorkOnAudioThread();
        */
    }
    
    pub fn static_finished(
        caller:  SLAndroidSimpleBufferQueueItf,
        context: *mut c_void)  {
        
        todo!();
        /*
            reinterpret_cast<OpenSLQueueRunner*> (pContext)->finished (caller);
        */
    }

    /**
       get the "this" pointer for CRTP
      */
    pub fn crtp_mut(&mut self) -> &mut ChildType {
        
        todo!();
        /*
            return * ((Child*) this);
        */
    }
    
    pub fn crtp(&self) -> &ChildType {
        
        todo!();
        /*
            return * ((Child*) this);
        */
    }
}
