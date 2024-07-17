crate::ix!();

#[inline] pub fn has_good_buffer_pointers(
        abl:     &AudioBufferList,
        n_bytes: u32) -> bool {
    
    todo!();
        /*
            const AudioBuffer *buf = abl.mBuffers;
        for (UInt32 i = abl.mNumberBuffers; i--;++buf) {
            if (buf->mData == NULL || buf->mDataByteSize < nBytes)
                return false;
        }
        return true;
        */
}


pub struct CoreAudioBufferList {
    scratch:        AudioBuffer/*<f32>*/,
    mutable_buffer: AudioBuffer/*<f32>*/,
    channels:       HeapBlock<*mut f32>,
    push_idx:       i32,
    pop_idx:        i32,
}

impl Default for CoreAudioBufferList {
    
    fn default() -> Self {
        todo!();
        /*
            reset();
        */
    }
}

impl CoreAudioBufferList {

    
    pub fn prepare(
        &mut self, 
        in_channels:  i32,
        out_channels: i32,
        max_frames:   i32
    ) {
        
        todo!();
        /*
            const int numChannels = jmax (inChannels, outChannels);

                scratch.setSize (numChannels, maxFrames);
                channels.calloc (static_cast<size_t> (numChannels));

                reset();
        */
    }
    
    pub fn release(&mut self)  {
        
        todo!();
        /*
            scratch.setSize (0, 0);
                channels.free();
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            pushIdx = 0;
                popIdx = 0;
                zeromem (channels.get(), sizeof(float*) * static_cast<size_t> (scratch.getNumChannels()));
        */
    }
    
    pub fn set_buffer(
        &mut self, 
        idx: i32,
        ptr: *mut f32
    ) -> *mut f32 {

        todo!();
        /*
            jassert (idx < scratch.getNumChannels());
                return (channels [idx] = uniqueBuffer (idx, ptr));
        */
    }
    
    pub fn push(&mut self) -> *mut f32 {
        
        todo!();
        /*
            jassert (pushIdx < scratch.getNumChannels());
                return channels [pushIdx++];
        */
    }
    
    pub fn push_buffer_list(
        &mut self, 
        buffer_list: &mut AudioBufferList,
        channel_map: *const i32
    ) {
        
        todo!();
        /*
            jassert (pushIdx < scratch.getNumChannels());

                if (bufferList.mNumberBuffers > 0)
                {
                    const UInt32 n = bufferList.mBuffers [0].mDataByteSize /
                    (bufferList.mBuffers [0].mNumberChannels * sizeof (float));
                    const bool isInterleaved = isAudioBufferInterleaved (bufferList);
                    const int numChannels = static_cast<int> (isInterleaved ? bufferList.mBuffers [0].mNumberChannels
                                                              : bufferList.mNumberBuffers);

                    for (int ch = 0; ch < numChannels; ++ch)
                    {
                        float* data = push();

                        int mappedChannel = channelMap [ch];
                        if (isInterleaved || static_cast<float*> (bufferList.mBuffers [mappedChannel].mData) != data)
                            copyAudioBuffer (bufferList, mappedChannel, n, data);
                    }
                }
        */
    }
    
    pub fn pop(&mut self) -> *mut f32 {
        
        todo!();
        /*
            jassert (popIdx < scratch.getNumChannels());
                return channels[popIdx++];
        */
    }
    
    pub fn pop_buffer_list(
        &mut self, 
        buffer:      &mut AudioBufferList,
        channel_map: *const i32

    )  {
        
        todo!();
        /*
            if (buffer.mNumberBuffers > 0)
                {
                    const UInt32 n = buffer.mBuffers [0].mDataByteSize / (buffer.mBuffers [0].mNumberChannels * sizeof (float));
                    const bool isInterleaved = isAudioBufferInterleaved (buffer);
                    const int numChannels = static_cast<int> (isInterleaved ? buffer.mBuffers [0].mNumberChannels : buffer.mNumberBuffers);

                    for (int ch = 0; ch < numChannels; ++ch)
                    {
                        int mappedChannel = channelMap [ch];
                        float* nextBuffer = pop();

                        if (nextBuffer == buffer.mBuffers [mappedChannel].mData && ! isInterleaved)
                            continue; // no copying necessary

                        if (buffer.mBuffers [mappedChannel].mData == nullptr && ! isInterleaved)
                            buffer.mBuffers [mappedChannel].mData = nextBuffer;
                        else
                            copyAudioBuffer (nextBuffer, mappedChannel, n, buffer);
                    }
                }
        */
    }
    
    pub fn get_buffer(&mut self, frames: u32) -> &mut AudioBuffer/*<f32>*/ {
        
        todo!();
        /*
            jassert (pushIdx == scratch.getNumChannels());

              #if ALOE_DEBUG
                for (int i = 0; i < pushIdx; ++i)
                    jassert (channels [i] != nullptr);
              #endif

                mutableBuffer.setDataToReferTo (channels, pushIdx, static_cast<int> (frames));
                return mutableBuffer;
        */
    }
    
    pub fn unique_buffer(&mut self, 
        idx:    i32,
        buffer: *mut f32) -> *mut f32 {
        
        todo!();
        /*
            if (buffer == nullptr)
                    return scratch.getWritePointer (idx);

                for (int ch = 0; ch < idx; ++ch)
                    if (buffer == channels[ch])
                        return scratch.getWritePointer (idx);

                return buffer;
        */
    }
}
