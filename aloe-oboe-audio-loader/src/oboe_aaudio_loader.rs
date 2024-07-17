crate::ix!();

pub fn oboe_aaudio_loader_check(
    proc:          *mut c_void,
    function_name: *const u8

) {

    todo!();
    /*
        if (proc == nullptr) {
                LOGW("OboeAAudioLoader could not find %s", functionName);
            }
    */
}

/**
  | The AAudio API was not available in early
  | versions of Android.
  | 
  | To avoid linker errors, we dynamically
  | link with the functions by name using
  | dlsym().
  | 
  | On older versions this linkage will
  | safely fail.
  |
  */
pub struct OboeAAudioLoader {

    /**
       Function pointers into the AAudio shared
       library.
      */
    create_stream_builder:                 oboe_aaudio_loader_signature_i_ps, // default = nullptr
    builder_open_stream:                   oboe_aaudio_loader_signature_i_pbpps, // default = nullptr
    builder_set_buffer_capacity_in_frames: oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_channel_count:             oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_device_id:                 oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_direction:                 oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_format:                    oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_frames_per_data_callback:  oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_performance_mode:          oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_sample_rate:               oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_sharing_mode:              oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_usage:                     oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_content_type:              oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_input_preset:              oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_session_id:                oboe_aaudio_loader_signature_v_pbi, // default = nullptr
    builder_set_package_name:              oboe_aaudio_loader_signature_v_pbcph, // default = nullptr
    builder_set_attribution_tag:           oboe_aaudio_loader_signature_v_pbcph, // default = nullptr
    builder_set_data_callback:             oboe_aaudio_loader_signature_v_pbpdpv, // default = nullptr
    builder_set_error_callback:            oboe_aaudio_loader_signature_v_pbpepv, // default = nullptr
    builder_delete:                        oboe_aaudio_loader_signature_i_pb, // default = nullptr
    stream_get_format:                     oboe_aaudio_loader_signature_f_ps, // default = nullptr
    stream_read:                           oboe_aaudio_loader_signature_i_pspvil, // default = nullptr
    stream_write:                          oboe_aaudio_loader_signature_i_pscpvil, // default = nullptr
    stream_wait_for_state_change:          oboe_aaudio_loader_signature_i_pstptl, // default = nullptr
    stream_get_timestamp:                  oboe_aaudio_loader_signature_i_pskplpl, // default = nullptr
    stream_close:                          oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_channel_count:              oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_device_id:                  oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_buffer_size:                oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_buffer_capacity:            oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_frames_per_burst:           oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_state:                      oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_performance_mode:           oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_sample_rate:                oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_sharing_mode:               oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_xrun_count:                 oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_set_buffer_size:                oboe_aaudio_loader_signature_i_psi, // default = nullptr
    stream_request_start:                  oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_request_pause:                  oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_request_flush:                  oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_request_stop:                   oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_frames_read:                oboe_aaudio_loader_signature_l_ps, // default = nullptr
    stream_get_frames_written:             oboe_aaudio_loader_signature_l_ps, // default = nullptr
    convert_result_to_text:                oboe_aaudio_loader_signature_cph_i, // default = nullptr
    stream_get_usage:                      oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_content_type:               oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_input_preset:               oboe_aaudio_loader_signature_i_ps, // default = nullptr
    stream_get_session_id:                 oboe_aaudio_loader_signature_i_ps, // default = nullptr
    lib_handle:                            *mut c_void, // default = nullptr
}

impl Drop for OboeAAudioLoader {

    fn drop(&mut self) {
        todo!();
        /* 
            // Issue 360: thread_local variables with non-trivial destructors
            // will cause segfaults if the containing library is dlclose()ed on
            // devices running M or newer, or devices before M when using a static STL.
            // The simple workaround is to not call dlclose.
            // https://github.com/android/ndk/wiki/Changelog-r22#known-issues
            //
            // The libaaudio and libaaudioclient do not use thread_local.
            // But, to be safe, we should avoid dlclose() if possible.
            // Because OboeAAudioLoader is a static Singleton, we can safely skip
            // calling dlclose() without causing a resource leak.
            LOGI("%s() dlclose(%s) not called, OK", __func__, LIB_AAUDIO_NAME);
         */
    }
}

/**
  | Use signatures for common functions.
  | Key to letter abbreviations.
  | S = Stream
  | B = Builder
  | I = int32_t
  | L = int64_t
  | T = sTate
  | K = clocKid_t
  | P = Pointer to following data type
  | C = Const prefix
  | H = cHar
  |
  | NOTE: ^^^ no!!! bad!
  |
  */
pub type oboe_aaudio_loader_signature_i_ppb = fn(builder: *mut *mut AAudioStreamBuilder) -> i32;
pub type oboe_aaudio_loader_signature_cph_i = fn(_0: i32) -> *const u8;

/**
   AAudioStreamBuilder_open()
  */
pub type oboe_aaudio_loader_signature_i_pbpps = fn(
    _0: *mut AAudioStreamBuilder, 
    stream: *mut *mut OboeAAudioStream) -> i32;

/**
  AAudioStreamBuilder_delete()
  */
pub type oboe_aaudio_loader_signature_i_pb = fn(
    _0: *mut AAudioStreamBuilder) -> i32;

/**
   AAudioStreamBuilder_setSampleRate()
  */
pub type oboe_aaudio_loader_signature_v_pbi = fn(
    _0: *mut AAudioStreamBuilder, 
    _1: i32) -> c_void;

pub type oboe_aaudio_loader_signature_v_pbcph = fn(
    _0: *mut AAudioStreamBuilder, _1: *const u8) -> c_void;

/**
   AAudioStream_getSampleRate()
  */
pub type oboe_aaudio_loader_signature_i_ps = fn(
    _0: *mut OboeAAudioStream) -> i32;

/**
   AAudioStream_getFramesRead()
  */
pub type oboe_aaudio_loader_signature_l_ps = fn(
    _0: *mut OboeAAudioStream) -> i64;

/**
   AAudioStream_setBufferSizeInFrames()
  */
pub type oboe_aaudio_loader_signature_i_psi = fn(
    _0: *mut OboeAAudioStream, 
    _1: i32) -> i32;

pub type oboe_aaudio_loader_signature_v_pbpdpv  = fn(
    _0: *mut AAudioStreamBuilder, 
    _1: AAudioStreamDataCallback, 
    _2: *mut c_void) -> c_void;

pub type oboe_aaudio_loader_signature_v_pbpepv  = fn(
    _0: *mut AAudioStreamBuilder, 
    _1: AAudioStreamErrorCallback, 
    _2: *mut c_void) -> c_void;

pub type oboe_aaudio_loader_signature_f_ps      = fn(
    stream: *mut OboeAAudioStream) -> AAudioFormat;

pub type oboe_aaudio_loader_signature_i_pspvil  = fn(
    _0: *mut OboeAAudioStream, 
    _1: *mut c_void, 
    _2: i32, _3: i64) -> i32;

pub type oboe_aaudio_loader_signature_i_pscpvil = fn(
    _0: *mut OboeAAudioStream, 
    _1: *const c_void, 
    _2: i32, _3: i64) -> i32;

pub type oboe_aaudio_loader_signature_i_pstptl  = fn(
    _0: *mut OboeAAudioStream, 
    _1: AAudioStreamState, 
    _2: *mut AAudioStreamState, _3: i64) -> i32;

pub type oboe_aaudio_loader_signature_i_pskplpl = fn(
    _0: *mut OboeAAudioStream, 
    _1: ClockId, 
    _2: *mut i64, _3: *mut i64) -> i32;

pub type oboe_aaudio_loader_signature_b_ps      = fn(
    _0: *mut OboeAAudioStream) -> bool;



impl OboeAAudioLoader {

    pub fn get_lib_handle(&self)  {
        
        todo!();
        /*
            return mLibHandle;
        */
    }
    
    /**
      | singleton
      |
      */
    pub fn get_instance(&mut self) -> *mut OboeAAudioLoader {
        
        todo!();
        /*
            static OboeAAudioLoader instance;
            return &instance;
        */
    }
    
    /**
      | Open the AAudio shared library and load
      | the function pointers.
      | 
      | This can be called multiple times.
      | 
      | It should only be called from one thread.
      | 
      | The destructor will clean up after the
      | open.
      | 
      | -----------
      | @return
      | 
      | 0 if successful or negative error.
      |
      */
    pub fn open(&mut self) -> i32 {
        
        todo!();
        /*
            if (mLibHandle != nullptr) {
                return 0;
            }

            // Use RTLD_NOW to avoid the unpredictable behavior that RTLD_LAZY can cause.
            // Also resolving all the links now will prevent a run-time penalty later.
            mLibHandle = dlopen(LIB_AAUDIO_NAME, RTLD_NOW);
            if (mLibHandle == nullptr) {
                LOGI("oboe_aaudio_loader_open() could not find " LIB_AAUDIO_NAME);
                return -1; // TODO review return code
            } else {
                LOGD("OboeAAudioLoader():  dlopen(%s) returned %p", LIB_AAUDIO_NAME, mLibHandle);
            }

            // Load all the function pointers.
            createStreamBuilder        = load_I_PPB("AAudio_createStreamBuilder");
            builder_openStream         = load_I_PBPPS("AAudioStreamBuilder_openStream");

            builder_setChannelCount    = load_V_PBI("AAudioStreamBuilder_setChannelCount");
            if (builder_setChannelCount == nullptr) {
                // Use old deprecated alias if needed.
                builder_setChannelCount = load_V_PBI("AAudioStreamBuilder_setSamplesPerFrame");
            }

            builder_setBufferCapacityInFrames = load_V_PBI("AAudioStreamBuilder_setBufferCapacityInFrames");
            builder_setDeviceId        = load_V_PBI("AAudioStreamBuilder_setDeviceId");
            builder_setDirection       = load_V_PBI("AAudioStreamBuilder_setDirection");
            builder_setFormat          = load_V_PBI("AAudioStreamBuilder_setFormat");
            builder_setFramesPerDataCallback = load_V_PBI("AAudioStreamBuilder_setFramesPerDataCallback");
            builder_setSharingMode     = load_V_PBI("AAudioStreamBuilder_setSharingMode");
            builder_setPerformanceMode = load_V_PBI("AAudioStreamBuilder_setPerformanceMode");
            builder_setSampleRate      = load_V_PBI("AAudioStreamBuilder_setSampleRate");

            if (getSdkVersion() >= __ANDROID_API_P__){
                builder_setUsage       = load_V_PBI("AAudioStreamBuilder_setUsage");
                builder_setContentType = load_V_PBI("AAudioStreamBuilder_setContentType");
                builder_setInputPreset = load_V_PBI("AAudioStreamBuilder_setInputPreset");
                builder_setSessionId   = load_V_PBI("AAudioStreamBuilder_setSessionId");
            }

            if (getSdkVersion() >= __ANDROID_API_S__){
                builder_setPackageName       = load_V_PBCPH("AAudioStreamBuilder_setPackageName");
                builder_setAttributionTag    = load_V_PBCPH("AAudioStreamBuilder_setAttributionTag");
            }

            builder_delete             = load_I_PB("AAudioStreamBuilder_delete");

            builder_setDataCallback    = load_V_PBPDPV("AAudioStreamBuilder_setDataCallback");
            builder_setErrorCallback   = load_V_PBPEPV("AAudioStreamBuilder_setErrorCallback");

            stream_read                = load_I_PSPVIL("AAudioStream_read");

            stream_write               = load_I_PSCPVIL("AAudioStream_write");

            stream_waitForStateChange  = load_I_PSTPTL("AAudioStream_waitForStateChange");

            stream_getTimestamp        = load_I_PSKPLPL("AAudioStream_getTimestamp");

            stream_getChannelCount     = load_I_PS("AAudioStream_getChannelCount");
            if (stream_getChannelCount == nullptr) {
                // Use old alias if needed.
                stream_getChannelCount    = load_I_PS("AAudioStream_getSamplesPerFrame");
            }

            stream_close               = load_I_PS("AAudioStream_close");

            stream_getBufferSize       = load_I_PS("AAudioStream_getBufferSizeInFrames");
            stream_getDeviceId         = load_I_PS("AAudioStream_getDeviceId");
            stream_getBufferCapacity   = load_I_PS("AAudioStream_getBufferCapacityInFrames");
            stream_getFormat           = load_F_PS("AAudioStream_getFormat");
            stream_getFramesPerBurst   = load_I_PS("AAudioStream_getFramesPerBurst");
            stream_getFramesRead       = load_L_PS("AAudioStream_getFramesRead");
            stream_getFramesWritten    = load_L_PS("AAudioStream_getFramesWritten");
            stream_getPerformanceMode  = load_I_PS("AAudioStream_getPerformanceMode");
            stream_getSampleRate       = load_I_PS("AAudioStream_getSampleRate");
            stream_getSharingMode      = load_I_PS("AAudioStream_getSharingMode");
            stream_getState            = load_I_PS("AAudioStream_getState");
            stream_getXRunCount        = load_I_PS("AAudioStream_getXRunCount");

            stream_requestStart        = load_I_PS("AAudioStream_requestStart");
            stream_requestPause        = load_I_PS("AAudioStream_requestPause");
            stream_requestFlush        = load_I_PS("AAudioStream_requestFlush");
            stream_requestStop         = load_I_PS("AAudioStream_requestStop");

            stream_setBufferSize       = load_I_PSI("AAudioStream_setBufferSizeInFrames");

            convertResultToText        = load_CPH_I("AAudio_convertResultToText");

            if (getSdkVersion() >= __ANDROID_API_P__){
                stream_getUsage        = load_I_PS("AAudioStream_getUsage");
                stream_getContentType  = load_I_PS("AAudioStream_getContentType");
                stream_getInputPreset  = load_I_PS("AAudioStream_getInputPreset");
                stream_getSessionId    = load_I_PS("AAudioStream_getSessionId");
            }
            return 0;
        */
    }
    
    /**
      | Load function pointers for specific
      | signatures.
      |
      */
    pub fn load_i_ppb(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_i_ppb {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_I_PPB>(proc);
        */
    }
    
    pub fn load_cph_i(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_cph_i {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_CPH_I>(proc);
        */
    }
    
    pub fn load_v_pbi(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_v_pbi {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_V_PBI>(proc);
        */
    }
    
    pub fn load_v_pbcph(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_v_pbcph {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_V_PBCPH>(proc);
        */
    }
    
    pub fn load_v_pbpdpv(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_v_pbpdpv {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_V_PBPDPV>(proc);
        */
    }
    
    pub fn load_v_pbpepv(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_v_pbpepv {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_V_PBPEPV>(proc);
        */
    }
    
    pub fn load_i_psi(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_i_psi {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_I_PSI>(proc);
        */
    }
    
    pub fn load_i_ps(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_i_ps {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_I_PS>(proc);
        */
    }
    
    pub fn load_l_ps(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_l_ps {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_L_PS>(proc);
        */
    }
    
    pub fn load_f_ps(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_f_ps {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_F_PS>(proc);
        */
    }
    
    pub fn load_b_ps(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_b_ps {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_B_PS>(proc);
        */
    }
    
    pub fn load_i_pb(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_i_pb {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_I_PB>(proc);
        */
    }
    
    pub fn load_i_pbpps(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_i_pbpps {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_I_PBPPS>(proc);
        */
    }
    
    pub fn load_i_pscpvil(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_i_pscpvil {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_I_PSCPVIL>(proc);
        */
    }
    
    pub fn load_i_pspvil(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_i_pspvil {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_I_PSPVIL>(proc);
        */
    }
    
    pub fn load_i_pstptl(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_i_pstptl {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_I_PSTPTL>(proc);
        */
    }
    
    pub fn load_i_pskplpl(&mut self, function_name: *const u8) -> oboe_aaudio_loader_signature_i_pskplpl {
        
        todo!();
        /*
            void *proc = dlsym(mLibHandle, functionName);
            AAudioLoader_check(proc, functionName);
            return reinterpret_cast<signature_I_PSKPLPL>(proc);
        */
    }
}
