crate::ix!();

pub enum AUBufferListEPtrState {
    kPtrsInvalid,
    kPtrsToMyMemory,
    kPtrsToExternalMemory
}

pub struct AUBufferList {
    ptr_state:         AUBufferListEPtrState,
    external_memory:   bool,
    ptrs:              *mut AudioBufferList,
    memory:            *mut u8,
    allocated_streams: u32,
    allocated_frames:  u32,
    allocated_bytes:   u32,
}

impl Drop for AUBufferList {

    fn drop(&mut self) {
        todo!();
        /*
            Deallocate();
        if (mPtrs)
            free(mPtrs);
        */
    }
}

impl Default for AUBufferList {
    
    fn default() -> Self {
        todo!();
        /*
        : ptr_state(kPtrsInvalid),
        : external_memory(false),
        : ptrs(NULL),
        : memory(NULL),
        : allocated_streams(0),
        : allocated_frames(0),
        : allocated_bytes(0),
        */
    }
}

impl AUBufferList {
    
    pub fn set_buffer_list(&mut self, abl: &AudioBufferList) -> &mut AudioBufferList {
        
        todo!();
        /*
            if (mAllocatedStreams < abl.mNumberBuffers)
                                    COMPONENT_THROW(-1);
                                mPtrState = kPtrsToExternalMemory;
                                memcpy(mPtrs, &abl, (char *)&abl.mBuffers[abl.mNumberBuffers] - (char *)&abl);
                                return *mPtrs;
        */
    }
    
    pub fn set_buffer(&mut self, 
        index: u32,
        ab:    &AudioBuffer)  {
        
        todo!();
        /*
            if (mPtrState == kPtrsInvalid || index >= mPtrs->mNumberBuffers)
                                    COMPONENT_THROW(-1);
                                mPtrState = kPtrsToExternalMemory;
                                mPtrs->mBuffers[index] = ab;
        */
    }
    
    pub fn invalidate_buffer_list(&mut self)  {
        
        todo!();
        /*
            mPtrState = kPtrsInvalid;
        */
    }
    
    pub fn get_buffer_list(&self) -> &mut AudioBufferList {
        
        todo!();
        /*
            if (mPtrState == kPtrsInvalid)
                                    COMPONENT_THROW(-1);
                                return *mPtrs;
        */
    }
    
    pub fn copy_buffer_list_to(&self, abl: &mut AudioBufferList)  {
        
        todo!();
        /*
            if (mPtrState == kPtrsInvalid)
                                    COMPONENT_THROW(-1);
                                memcpy(&abl, mPtrs, (char *)&abl.mBuffers[abl.mNumberBuffers] - (char *)&abl);
        */
    }
    
    pub fn copy_buffer_contents_to(&self, abl: &mut AudioBufferList)  {
        
        todo!();
        /*
            if (mPtrState == kPtrsInvalid)
                                    COMPONENT_THROW(-1);
                                const AudioBuffer *srcbuf = mPtrs->mBuffers;
                                AudioBuffer *destbuf = abl.mBuffers;

                                for (UInt32 i = 0; i < abl.mNumberBuffers; ++i, ++srcbuf, ++destbuf) {
                                    if (i >= mPtrs->mNumberBuffers) // duplicate last source to additional outputs [4341137]
                                        --srcbuf;
                                    if (destbuf->mData != srcbuf->mData)
                                        memmove(destbuf->mData, srcbuf->mData, srcbuf->mDataByteSize);
                                    destbuf->mDataByteSize = srcbuf->mDataByteSize;
                                }
        */
    }
    
    /* ----------- AudioBufferList utilities  ----------- */

    pub fn zero_buffer(abl: &mut AudioBufferList)  {
        
        todo!();
        /*
            AudioBuffer *buf = abl.mBuffers;
                                for (UInt32 i = abl.mNumberBuffers ; i--; ++buf)
                                    memset(buf->mData, 0, buf->mDataByteSize);
        */
    }

    #[cfg(DEBUG)]
    pub fn print_buffer(
        label:     *const u8,
        subscript: i32,
        abl:       &AudioBufferList,
        n_frames:  u32,
        as_floats: bool)  {

        let n_frames: u32 = n_frames.unwrap_or(8);
        let as_floats: bool = as_floats.unwrap_or(true);

        todo!();
        /*
        
        */
    }
    
    pub fn get_allocated_frames(&self) -> u32 {
        
        todo!();
        /*
            return mAllocatedFrames;
        */
    }

    /**
      | prohibit copy constructor
      |
      */
    pub fn new(_0: &mut AUBufferList) -> Self {
    
        todo!();
        /*


        
        */
    }

    pub fn allocate(&mut self, 
        format:   &CAStreamBasicDescription,
        n_frames: u32)  {
        
        todo!();
        /*
            UInt32 nStreams;
        if (format.IsInterleaved()) {
            nStreams = 1;
        } else {
            nStreams = format.mChannelsPerFrame;
        }

        // careful -- the I/O thread could be running!
        if (nStreams > mAllocatedStreams) {
            size_t theHeaderSize = sizeof(AudioBufferList) - sizeof(AudioBuffer);
            mPtrs = (AudioBufferList *)CA_realloc(mPtrs,
                                        SafeMultiplyAddUInt32(nStreams, sizeof(AudioBuffer), theHeaderSize));
            mAllocatedStreams = nStreams;
        }
        UInt32 bytesPerStream = SafeMultiplyAddUInt32(nFrames, format.mBytesPerFrame, 0xF) & ~0xF;
        UInt32 nBytes = SafeMultiplyAddUInt32(nStreams, bytesPerStream, 0);
        if (nBytes > mAllocatedBytes) {
            if (mExternalMemory) {
                mExternalMemory = false;
                mMemory = NULL;
            }
            mMemory = (Byte *)CA_realloc(mMemory, nBytes);
            mAllocatedBytes = nBytes;
        }
        mAllocatedFrames = nFrames;
        mPtrState = kPtrsInvalid;
        */
    }
    
    pub fn deallocate(&mut self)  {
        
        todo!();
        /*
            mAllocatedStreams = 0;
        mAllocatedFrames = 0;
        mAllocatedBytes = 0;
    // this causes a world of hurt if someone upstream disconnects during I/O (SysSoundGraph)
    /*  if (mPtrs) {
            printf("deallocating bufferlist %08X\n", int(mPtrs));
            free(mPtrs);
            mPtrs = NULL;
        } */
        if (mMemory) {
            if (mExternalMemory)
                mExternalMemory = false;
            else
                free(mMemory);
            mMemory = NULL;
        }
        mPtrState = kPtrsInvalid;
        */
    }
    
    pub fn prepare_buffer(&mut self, 
        format:   &CAStreamBasicDescription,
        n_frames: u32) -> &mut AudioBufferList {
        
        todo!();
        /*
            if (nFrames > mAllocatedFrames)
            COMPONENT_THROW(kAudioUnitErr_TooManyFramesToProcess);

        UInt32 nStreams;
        UInt32 channelsPerStream;
        if (format.IsInterleaved()) {
            nStreams = 1;
            channelsPerStream = format.mChannelsPerFrame;
        } else {
            nStreams = format.mChannelsPerFrame;
            channelsPerStream = 1;
            if (nStreams > mAllocatedStreams)
                COMPONENT_THROW(kAudioUnitErr_FormatNotSupported);
        }

        AudioBufferList *abl = mPtrs;
        abl->mNumberBuffers = nStreams;
        AudioBuffer *buf = abl->mBuffers;
        Byte *mem = mMemory;
        UInt32 streamInterval = (mAllocatedFrames * format.mBytesPerFrame + 0xF) & ~0xF;
        UInt32 bytesPerBuffer = nFrames * format.mBytesPerFrame;
        for ( ; nStreams--; ++buf) {
            buf->mNumberChannels = channelsPerStream;
            buf->mData = mem;
            buf->mDataByteSize = bytesPerBuffer;
            mem += streamInterval;
        }
        if (UInt32(mem - mMemory) > mAllocatedBytes)
            COMPONENT_THROW(kAudioUnitErr_TooManyFramesToProcess);
        mPtrState = kPtrsToMyMemory;
        return *mPtrs;
        */
    }
    
    pub fn prepare_null_buffer(&mut self, 
        format:   &CAStreamBasicDescription,
        n_frames: u32) -> &mut AudioBufferList {
        
        todo!();
        /*
            UInt32 nStreams;
        UInt32 channelsPerStream;
        if (format.IsInterleaved()) {
            nStreams = 1;
            channelsPerStream = format.mChannelsPerFrame;
        } else {
            nStreams = format.mChannelsPerFrame;
            channelsPerStream = 1;
            if (nStreams > mAllocatedStreams)
                COMPONENT_THROW(kAudioUnitErr_FormatNotSupported);
        }
        AudioBufferList *abl = mPtrs;
        abl->mNumberBuffers = nStreams;
        AudioBuffer *buf = abl->mBuffers;
        UInt32 bytesPerBuffer = nFrames * format.mBytesPerFrame;
        for ( ; nStreams--; ++buf) {
            buf->mNumberChannels = channelsPerStream;
            buf->mData = NULL;
            buf->mDataByteSize = bytesPerBuffer;
        }
        mPtrState = kPtrsToExternalMemory;
        return *mPtrs;
        */
    }

    /**
       this should NOT be called while I/O is in
       process
      */
    pub fn use_external_buffer(&mut self, 
        format: &CAStreamBasicDescription,
        buf:    &AudioUnitExternalBuffer)  {
        
        todo!();
        /*
            UInt32 alignedSize = buf.size & ~0xF;
        if (mMemory != NULL && alignedSize >= mAllocatedBytes) {
            // don't accept the buffer if we already have one and it's big enough
            // if we don't already have one, we don't need one
            Byte *oldMemory = mMemory;
            mMemory = buf.buffer;
            mAllocatedBytes = alignedSize;
            // from Allocate(): nBytes = nStreams * nFrames * format.mBytesPerFrame;
            // thus: nFrames = nBytes / (nStreams * format.mBytesPerFrame)
            mAllocatedFrames = mAllocatedBytes / (format.NumberChannelStreams() * format.mBytesPerFrame);
            mExternalMemory = true;
            free(oldMemory);
        }
        */
    }
    
    #[cfg(DEBUG)]
    pub fn print_buffer(&mut self, 
        label:     *const u8,
        subscript: i32,
        abl:       &AudioBufferList,
        n_frames:  u32,
        as_floats: bool)  {
        
        todo!();
        /*
            printf("  %s [%d] 0x%08lX:\n", label, subscript, long(&abl));
        const AudioBuffer *buf = abl.mBuffers;
        for (UInt32 i = 0; i < abl.mNumberBuffers; ++buf, ++i) {
            printf("      [%2d] %5dbytes %dch @ %p: ", (int)i, (int)buf->mDataByteSize, (int)buf->mNumberChannels, buf->mData);
            if (buf->mData != NULL) {
                UInt32 nSamples = nFrames * buf->mNumberChannels;
                for (UInt32 j = 0; j < nSamples; ++j) {
                    if (nSamples > 16 && (j % 16) == 0)
                        printf("\n\t");
                    if (asFloats)
                        printf(" %6.3f", ((float *)buf->mData)[j]);
                    else
                        printf(" %08X", (unsigned)((UInt32 *)buf->mData)[j]);
                }
            }
            printf("\n");
        }
        */
    }
}
