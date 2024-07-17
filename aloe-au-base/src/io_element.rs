crate::ix!();

pub struct AUIOElement {
    base:          AUElement,

    stream_format: CAStreamBasicDescription,

    /**
      | for input: input proc buffer, only
      | allocated when needed 
      |
      | for output: output cache, usually
      | allocated early on
      |
      */
    io_buffer:     AUBufferList,

    will_allocate: bool,
}

pub trait NeedsBufferSpace {
    fn needs_buffer_space(&self) -> bool;
}

impl AUIOElementInterface for AUIOElement {}

pub trait AUIOElementInterface {

    fn set_stream_format(&mut self, desc: &CAStreamBasicDescription) -> OSStatus {
        
        todo!();
        /*
        
        */
    }

    fn allocate_buffer(&mut self, in_frames_to_allocate: Option<u32>)  {

        let in_frames_to_allocate: u32 = in_frames_to_allocate.unwrap_or(0);

        todo!();
        /*
        
        */
    }

    fn get_channel_layout_tags(&mut self, out_layout_tags_ptr: *mut AudioChannelLayoutTag) -> u32 {
        
        todo!();
        /*
        
        */
    }

    fn get_audio_channel_layout(&mut self, 
        out_map_ptr:  *mut AudioChannelLayout,
        out_writable: &mut bool) -> u32 {
        
        todo!();
        /*
        
        */
    }

    fn set_audio_channel_layout(&mut self, in_data: &AudioChannelLayout) -> OSStatus {
        
        todo!();
        /*
        
        */
    }

    fn remove_audio_channel_layout(&mut self) -> OSStatus {
        
        todo!();
        /*
        
        */
    }

    fn as_io_element(&mut self) -> *mut AUIOElement {
        
        todo!();
        /*
            return this;
        */
    }
}

impl AUIOElement {

    pub fn get_stream_format(&self) -> &CAStreamBasicDescription {
        
        todo!();
        /*
            return mStreamFormat;
        */
    }
    
    pub fn set_will_allocate_buffer(&mut self, in_flag: bool)  {
        
        todo!();
        /*
            mWillAllocate = inFlag;
        */
    }
    
    pub fn will_allocate_buffer(&self) -> bool {
        
        todo!();
        /*
            return mWillAllocate;
        */
    }
    
    pub fn use_external_buffer(&mut self, buf: &AudioUnitExternalBuffer)  {
        
        todo!();
        /*
            mIOBuffer.UseExternalBuffer(mStreamFormat, buf);
        */
    }
    
    pub fn prepare_buffer(&mut self, n_frames: u32) -> &mut AudioBufferList {
        
        todo!();
        /*
            if (mWillAllocate)
                                            return mIOBuffer.PrepareBuffer(mStreamFormat, nFrames);
                                        throw OSStatus(kAudioUnitErr_InvalidPropertyValue);
        */
    }
    
    pub fn prepare_null_buffer(&mut self, n_frames: u32) -> &mut AudioBufferList {
        
        todo!();
        /*
            return mIOBuffer.PrepareNullBuffer(mStreamFormat, nFrames);
        */
    }
    
    pub fn set_buffer_list(&mut self, abl: &mut AudioBufferList) -> &mut AudioBufferList {
        
        todo!();
        /*
            return mIOBuffer.SetBufferList(abl);
        */
    }
    
    pub fn set_buffer(&mut self, 
        index: u32,
        ab:    &mut AudioBuffer)  {
        
        todo!();
        /*
            mIOBuffer.SetBuffer(index, ab);
        */
    }
    
    pub fn invalidate_buffer_list(&mut self)  {
        
        todo!();
        /*
            mIOBuffer.InvalidateBufferList();
        */
    }
    
    pub fn get_buffer_list(&self) -> &mut AudioBufferList {
        
        todo!();
        /*
            return mIOBuffer.GetBufferList();
        */
    }
    
    pub fn get_channel_data(&self, ch: i32) -> *mut AudioUnitSampleType {
        
        todo!();
        /*
            if (mStreamFormat.IsInterleaved())
                                            return static_cast<AudioUnitSampleType *>(mIOBuffer.GetBufferList().mBuffers[0].mData) + ch;
                                        else
                                            return static_cast<AudioUnitSampleType *>(mIOBuffer.GetBufferList().mBuffers[ch].mData);
        */
    }
    
    pub fn get_float_32channel_data(&self, ch: i32) -> *mut f32 {
        
        todo!();
        /*
            if (mStreamFormat.IsInterleaved())
                                            return static_cast<Float32 *>(mIOBuffer.GetBufferList().mBuffers[0].mData) + ch;
                                        else
                                            return static_cast<Float32 *>(mIOBuffer.GetBufferList().mBuffers[ch].mData);
        */
    }
    
    pub fn get_sint_32channel_data(&self, ch: i32) -> *mut i32 {
        
        todo!();
        /*
            if (mStreamFormat.IsInterleaved())
                                            return static_cast<SInt32 *>(mIOBuffer.GetBufferList().mBuffers[0].mData) + ch;
                                        else
                                            return static_cast<SInt32 *>(mIOBuffer.GetBufferList().mBuffers[ch].mData);
        */
    }
    
    pub fn get_int_16channel_data(&self, ch: i32) -> *mut i16 {
        
        todo!();
        /*
            if (mStreamFormat.IsInterleaved())
                                            return static_cast<SInt16 *>(mIOBuffer.GetBufferList().mBuffers[0].mData) + ch;
                                        else
                                            return static_cast<SInt16 *>(mIOBuffer.GetBufferList().mBuffers[ch].mData);
        */
    }
    
    pub fn copy_buffer_list_to(&self, abl: &mut AudioBufferList)  {
        
        todo!();
        /*
            mIOBuffer.CopyBufferListTo(abl);
        */
    }
    
    pub fn copy_buffer_contents_to(&self, abl: &mut AudioBufferList)  {
        
        todo!();
        /*
            mIOBuffer.CopyBufferContentsTo(abl);
        */
    }
    
    pub fn is_interleaved(&self) -> bool {
        
        todo!();
        /*
            return mStreamFormat.IsInterleaved();
        */
    }
    
    pub fn number_channels(&self) -> u32 {
        
        todo!();
        /*
            return mStreamFormat.NumberChannels();
        */
    }
    
    pub fn number_interleaved_channels(&self) -> u32 {
        
        todo!();
        /*
            return mStreamFormat.NumberInterleavedChannels();
        */
    }

    pub fn new(audio_unit: *mut AUBase) -> Self {
    
        todo!();
        /*
        : au_element(audioUnit),
        : will_allocate(true),

            mStreamFormat.SetAUCanonical(2, // stereo
            audioUnit->AudioUnitAPIVersion() == 1);
            // interleaved if API version 1, deinterleaved if version 2
        mStreamFormat.mSampleRate = kAUDefaultSampleRate;
        */
    }
    
    pub fn set_stream_format(&mut self, desc: &CAStreamBasicDescription) -> OSStatus {
        
        todo!();
        /*
            mStreamFormat = desc;
        return AUBase::noErr;
        */
    }

    /**
      inFramesToAllocate == 0 implies the
      AudioUnit's max-frames-per-slice will be
      used
      */
    pub fn allocate_buffer(&mut self, in_frames_to_allocate: u32)  {
        
        todo!();
        /*
            if (GetAudioUnit()->HasBegunInitializing())
        {
            UInt32 framesToAllocate = inFramesToAllocate > 0 ? inFramesToAllocate : GetAudioUnit()->GetMaxFramesPerSlice();

    //      printf ("will allocate: %d\n", (int)((mWillAllocate && NeedsBufferSpace()) ? framesToAllocate : 0));

            mIOBuffer.Allocate(mStreamFormat, (mWillAllocate && NeedsBufferSpace()) ? framesToAllocate : 0);
        }
        */
    }
    
    pub fn deallocate_buffer(&mut self)  {
        
        todo!();
        /*
            mIOBuffer.Deallocate();
        */
    }

    /* ---------- AudioChannelLayout support  ---------- */


    /**
      | outLayoutTagsPtr WILL be NULL if called to find
      | out how many layouts that Audio Unit will
      | report return 0 (ie. NO channel layouts) if the
      | AU doesn't require channel layout knowledge
      */
    pub fn get_channel_layout_tags(&mut self, out_layout_tags_ptr: *mut AudioChannelLayoutTag) -> u32 {
        
        todo!();
        /*
            return 0;
        */
    }

    /**
      | As the AudioChannelLayout can be a variable
      | length structure (though in most cases it won't
      | be!!!)
      |
      | The size of the ACL is always returned by the
      | method if outMapPtr is NOT-NULL, then AU should
      | copy into this pointer (outMapPtr) the current
      | ACL that it has in use.  the AU should also
      | return whether the property is writable (that
      | is the client can provide any arbitrary ACL
      | that the audio unit will then honour) or if the
      | property is read only - which is the generally
      | preferred mode.
      |
      | If the AU doesn't require an
      | AudioChannelLayout, then just return 0.
      */
    pub fn get_audio_channel_layout(&mut self, 
        out_map_ptr:  *mut AudioChannelLayout,
        out_writable: &mut bool) -> u32 {
        
        todo!();
        /*
            return 0;
        */
    }

    /**
      | the incoming channel map will be at least as
      | big as a basic AudioChannelLayout but its
      | contents will determine its actual size
      | Subclass should overide if channel map is
      | writable
      */
    pub fn set_audio_channel_layout(&mut self, in_data: &AudioChannelLayout) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidProperty;
        */
    }

    /**
      | Some units support optional usage of channel
      | maps - typically converter units that can do
      | channel remapping between different maps. In
      | that optional case the user should be able to
      | remove a channel map if that is possible.
      |
      | Typically this is NOT the case (e.g., the
      | 3DMixer even in the stereo case needs to know
      | if it is rendering to speakers or headphones)
      */
    pub fn remove_audio_channel_layout(&mut self) -> OSStatus {
        
        todo!();
        /*
            return kAudioUnitErr_InvalidPropertyValue;
        */
    }
}
