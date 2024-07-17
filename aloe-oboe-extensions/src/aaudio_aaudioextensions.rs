crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/aaudio/OboeAAudioExtensions.h]

pub const OBOE_LIB_AAUDIO_NAME:          &'static str = "libaaudio.so";
pub const OBOE_FUNCTION_IS_MMAP:         &'static str = "AAudioStream_isMMapUsed";
pub const OBOE_FUNCTION_SET_MMAP_POLICY: &'static str = "AAudio_setMMapPolicy";
pub const OBOE_FUNCTION_GET_MMAP_POLICY: &'static str = "AAudio_getMMapPolicy";

pub const OBOE_AAUDIO_EXTENSIONS_AAUDIO_POLICY_NEVER:  usize = 1;
pub const OBOE_AAUDIO_EXTENSIONS_AAUDIO_POLICY_AUTO:   usize = 2;
pub const OBOE_AAUDIO_EXTENSIONS_AAUDIO_POLICY_ALWAYS: usize = 3;

macro_rules! oboe_aaudio_error_unavailable {
    () => {
        /*
                static_cast<aaudio_result_t>(OboeResult::ErrorUnavailable)
        */
    }
}

pub type OboeAAudioStream = OboeAAudioStreamStruct;

pub type OboeAAudioPolicy = i32;

/**
  | Call some AAudio test routines that
  | are not part of the normal API.
  |
  */
pub struct OboeAAudioExtensions {
    mmap_supported:           bool, // default = false
    mmap_exclusive_supported: bool, // default = false
    aaudio_stream_is_mmap:    fn(stream: *mut OboeAAudioStream) -> bool,
    aaudio_set_mmap_policy:   fn(policy: OboeAAudioPolicy) -> i32,
    aaudio_get_mmap_policy:   fn() -> OboeAAudioPolicy,
}

impl Default for OboeAAudioExtensions {
    
    fn default() -> Self {
        todo!();
        /*


            int32_t policy = getIntegerProperty("aaudio.mmap_policy", 0);
            mMMapSupported = isPolicyEnabled(policy);

            policy = getIntegerProperty("aaudio.mmap_exclusive_policy", 0);
            mMMapExclusiveSupported = isPolicyEnabled(policy)
        */
    }
}

impl OboeAAudioExtensions {

    pub fn is_policy_enabled(policy: i32) -> bool {
        
        todo!();
        /*
            return (policy == AAUDIO_POLICY_AUTO || policy == AAUDIO_POLICY_ALWAYS);
        */
    }
    
    pub fn get_instance<'a>() -> &'a mut OboeAAudioExtensions {
        
        todo!();
        /*
            static OboeAAudioExtensions instance;
            return instance;
        */
    }
    
    pub fn is_mmap_used_for_oboe_stream(&mut self, oboe_stream: *mut OboeAAudioStream) -> bool {
        
        todo!();
        /*
            AAudioStream *aaudioStream = (AAudioStream *) oboeStream->getUnderlyingStream();
            return isMMapUsed(aaudioStream);
        */
    }
    
    pub fn is_mmap_used(&mut self, aaudio_stream: *mut AAudioStream) -> bool {
        
        todo!();
        /*
            if (loadSymbols()) return false;
            if (mAAudioStream_isMMap == nullptr) return false;
            return mAAudioStream_isMMap(aaudioStream);
        */
    }

    /**
      | Controls whether the MMAP data path
      | can be selected when opening a stream.
      | 
      | It has no effect after the stream has
      | been opened.
      | 
      | It only affects the application that
      | calls it. Other apps are not affected.
      | 
      | -----------
      | @param enabled
      | 
      | @return 0 or a negative error code
      |
      */
    pub fn set_mmap_enabled(&mut self, enabled: bool) -> i32 {
        
        todo!();
        /*
            if (loadSymbols()) return AAUDIO_ERROR_UNAVAILABLE;
            if (mAAudio_setMMapPolicy == nullptr) return false;
            return mAAudio_setMMapPolicy(enabled ? AAUDIO_POLICY_AUTO : AAUDIO_POLICY_NEVER);
        */
    }
    
    pub fn is_mmap_enabled(&mut self) -> bool {
        
        todo!();
        /*
            if (loadSymbols()) return false;
            if (mAAudio_getMMapPolicy == nullptr) return false;
            int32_t policy = mAAudio_getMMapPolicy();
            return isPolicyEnabled(policy);
        */
    }
    
    pub fn is_mmap_supported(&mut self) -> bool {
        
        todo!();
        /*
            return mMMapSupported;
        */
    }
    
    pub fn is_mmap_exclusive_supported(&mut self) -> bool {
        
        todo!();
        /*
            return mMMapExclusiveSupported;
        */
    }
    
    pub fn get_integer_property(&mut self, 
        name:          *const u8,
        default_value: i32) -> i32 {
        
        todo!();
        /*
            int result = defaultValue;
            char valueText[PROP_VALUE_MAX] = {0};
            if (__system_property_get(name, valueText) != 0) {
                result = atoi(valueText);
            }
            return result;
        */
    }

    /**
      | Load the function pointers.
      | 
      | This can be called multiple times.
      | 
      | It should only be called from one thread.
      | 
      | -----------
      | @return
      | 
      | 0 if successful or negative error.
      |
      */
    pub fn load_symbols(&mut self) -> AAudioResult {
        
        todo!();
        /*
            if (mAAudio_getMMapPolicy != nullptr) {
                return 0;
            }

            void *libHandle = AAudioLoader::getInstance()->getLibHandle();
            if (libHandle == nullptr) {
                LOGI("%s() could not find " LIB_AAUDIO_NAME, __func__);
                return AAUDIO_ERROR_UNAVAILABLE;
            }

            mAAudioStream_isMMap = (bool (*)(AAudioStream *stream))
                    dlsym(libHandle, FUNCTION_IS_MMAP);
            if (mAAudioStream_isMMap == nullptr) {
                LOGI("%s() could not find " FUNCTION_IS_MMAP, __func__);
                return AAUDIO_ERROR_UNAVAILABLE;
            }

            mAAudio_setMMapPolicy = (int32_t (*)(aaudio_policy_t policy))
                    dlsym(libHandle, FUNCTION_SET_MMAP_POLICY);
            if (mAAudio_setMMapPolicy == nullptr) {
                LOGI("%s() could not find " FUNCTION_SET_MMAP_POLICY, __func__);
                return AAUDIO_ERROR_UNAVAILABLE;
            }

            mAAudio_getMMapPolicy = (aaudio_policy_t (*)())
                    dlsym(libHandle, FUNCTION_GET_MMAP_POLICY);
            if (mAAudio_getMMapPolicy == nullptr) {
                LOGI("%s() could not find " FUNCTION_GET_MMAP_POLICY, __func__);
                return AAUDIO_ERROR_UNAVAILABLE;
            }

            return 0;
        */
    }
}
