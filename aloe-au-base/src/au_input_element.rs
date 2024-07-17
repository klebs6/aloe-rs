crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUInputElement.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/AUInputElement.cpp]

pub enum AUInputElementEInputType { 
    kNoInput, 
    kFromConnection, 
    kFromCallback 
}

pub trait AUInputElementInterface {

    fn set_stream_format(&mut self, desc: &CAStreamBasicDescription) -> OSStatus;

    fn needs_buffer_space(&self) -> bool;
}

pub struct AUInputElement {
    base:                  AUIOElement,

    input_type:            AUInputElementEInputType,

    /**
      | if from callback:
      |
      */
    input_proc:            AURenderCallback,

    input_proc_ref_con:    *mut c_void,

    /**
      | if from connection:
      |
      */
    connection:            AudioUnitConnection,

    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    conn_render_proc:      AudioUnitRenderProc,

    /**
      | for the input component
      |
      */
    conn_instance_storage: *mut c_void,
}

impl AUInputElement {

    pub fn is_active(&self) -> bool {
        
        todo!();
        /*
            return mInputType != kNoInput;
        */
    }
    
    pub fn is_callback(&self) -> bool {
        
        todo!();
        /*
            return mInputType == kFromCallback;
        */
    }
    
    pub fn has_connection(&self) -> bool {
        
        todo!();
        /*
            return mInputType == kFromConnection;
        */
    }
    
    pub fn new(audio_unit: *mut AUBase) -> Self {
    
        todo!();
        /*
        : auio_element(audioUnit),
        : input_type(kNoInput),

        
        */
    }
    
    pub fn set_connection(&mut self, conn: &AudioUnitConnection)  {
        
        todo!();
        /*
            if (conn.sourceAudioUnit == 0) {
            Disconnect();
            return;
        }

        mInputType = kFromConnection;
        mConnection = conn;
        AllocateBuffer();

        mConnInstanceStorage = NULL;

    #if !CA_USE_AUDIO_PLUGIN_ONLY
        mConnRenderProc = NULL;
        UInt32 size = sizeof(AudioUnitRenderProc);
        OSStatus result = AudioUnitGetProperty( conn.sourceAudioUnit,
                                kAudioUnitProperty_FastDispatch,
                                kAudioUnitScope_Global,
                                kAudioUnitRenderSelect,
                                &mConnRenderProc,
                                &size);
        if (result == noErr)
            mConnInstanceStorage = CMgr_GetComponentInstanceStorage (conn.sourceAudioUnit);
        else
            mConnRenderProc = NULL;
    #endif
        */
    }
    
    pub fn disconnect(&mut self)  {
        
        todo!();
        /*
            mInputType = kNoInput;
        mIOBuffer.Deallocate();
        */
    }
    
    pub fn set_input_callback(&mut self, 
        proc:    AURenderCallback,
        ref_con: *mut c_void)  {
        
        todo!();
        /*
            if (proc == NULL)
            Disconnect();
        else {
            mInputType = kFromCallback;
            mInputProc = proc;
            mInputProcRefCon = refCon;
            AllocateBuffer();
        }
        */
    }
    
    pub fn set_stream_format(&mut self, fmt: &CAStreamBasicDescription) -> OSStatus {
        
        todo!();
        /*
            OSStatus err = AUIOElement::SetStreamFormat(fmt);
        if (err == AUBase::noErr)
            AllocateBuffer();
        return err;
        */
    }
    
    pub fn pull_input(&mut self, 
        io_action_flags: &mut AudioUnitRenderActionFlags,
        in_time_stamp:   &AudioTimeStamp,
        in_element:      AudioUnitElement,
        n_frames:        u32) -> OSStatus {
        
        todo!();
        /*
            if (!IsActive())
            return kAudioUnitErr_NoConnection;

        AudioBufferList *pullBuffer;

        if (HasConnection() || !WillAllocateBuffer())
            pullBuffer = &mIOBuffer.PrepareNullBuffer(mStreamFormat, nFrames);
        else
            pullBuffer = &mIOBuffer.PrepareBuffer(mStreamFormat, nFrames);

        return PullInputWithBufferList (ioActionFlags, inTimeStamp, inElement, nFrames, pullBuffer);
        */
    }
}
