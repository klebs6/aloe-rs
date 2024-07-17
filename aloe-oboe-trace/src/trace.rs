crate::ix!();

lazy_static!{
    /*
    static char buffer[256];
    */
}

/**
   Tracing functions
  */
pub type a_trace_begin_section = fn(section_name: *const u8) -> *mut c_void;
pub type a_trace_end_section   = fn() -> *mut c_void;

pub type fp_a_trace_begin_section = fn(section_name: *const u8) -> *mut c_void;
pub type fp_a_trace_end_section   = fn() -> *mut c_void;

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/Trace.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/common/Trace.cpp]
lazy_static!{
    /*
    static bool mIsTracingSupported;
    bool Trace::mIsTracingSupported = false;
    */
}

pub struct Trace {

}

impl Trace {

    pub fn begin_section(&mut self, 
        format: *const u8,
        args:   &[&str])  {
        
        todo!();
        /*
            if (mIsTracingSupported) {
            va_list va;
            va_start(va, format);
            vsprintf(buffer, format, va);
            ATrace_beginSection(buffer);
            va_end(va);
        } else {
            LOGE("Tracing is either not initialized (call Trace::initialize()) "
                 "or not supported on this device");
        }
        */
    }
    
    pub fn end_section(&mut self)  {
        
        todo!();
        /*
            if (mIsTracingSupported) {
            ATrace_endSection();
        }
        */
    }
    
    pub fn initialize(&mut self)  {
        
        todo!();
        /*
            // Using dlsym allows us to use tracing on API 21+ without needing android/trace.h which wasn't
        // published until API 23
        void *lib = dlopen("libandroid.so", RTLD_NOW | RTLD_LOCAL);
        if (lib == nullptr) {
            LOGE("Could not open libandroid.so to dynamically load tracing symbols");
        } else {
            ATrace_beginSection =
                    reinterpret_cast<fp_ATrace_beginSection >(
                            dlsym(lib, "ATrace_beginSection"));
            ATrace_endSection =
                    reinterpret_cast<fp_ATrace_endSection >(
                            dlsym(lib, "ATrace_endSection"));

            if (ATrace_beginSection != nullptr && ATrace_endSection != nullptr){
                mIsTracingSupported = true;
            }
        }
        */
    }
}
