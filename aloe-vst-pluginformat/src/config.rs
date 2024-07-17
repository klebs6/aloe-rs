crate::ix!();

pub const VST_FORCE_DEPRECATED:         usize = 0;
pub const ALOE_VSTINTERFACE_H_INCLUDED: usize = 1;

#[cfg(ALOE_MINGW)]
#[cfg(not(WM_APPCOMMAND))]
pub const WM_APPCOMMAND: usize = 0x0319;

#[cfg(ALOE_MINGW)]
lazy_static!{
    /*
    extern "C" void _fpreset();
     extern "C" void _clearfp();
    */
}

#[cfg(not(target_os="windows"))]
pub fn fpreset()  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(target_os="windows"))]
pub fn clearfp()  {
    
    todo!();
        /*
        
        */
}

#[cfg(not(ALOE_VST_WRAPPER_LOAD_CUSTOM_MAIN))]
pub const ALOE_VST_WRAPPER_LOAD_CUSTOM_MAIN: bool = true;

#[cfg(not(ALOE_VST_WRAPPER_INVOKE_MAIN))]
macro_rules! aloe_vst_wrapper_invoke_main {
    () => {
        /*
                effect = module->moduleMain ((typename Vst2audioMasterCallback) &audioMaster);
        */
    }
}

#[cfg(not(ALOE_VST_FALLBACK_HOST_NAME))]
pub const ALOE_VST_FALLBACK_HOST_NAME: &'static str = "Aloe VST Host";

/**
   Compares a magic value in either endianness.
  */
pub fn compare_magic(
        magic: i32,
        name:  *const u8) -> bool {
    
    todo!();
        /*
            return magic == (int32) ByteOrder::littleEndianInt (name)
            || magic == (int32) ByteOrder::bigEndianInt (name);
        */
}

pub fn fxb_name(name: *const u8) -> i32 {
    
    todo!();
        /*
            return (int32) ByteOrder::littleEndianInt (name);
        */
}

pub fn fxb_swap(x: i32) -> i32 {
    
    todo!();
        /*
            return (int32) ByteOrder::swapIfLittleEndian ((uint32) x);
        */
}

pub fn fxb_swap_float(x: f32) -> f32 {
    
    todo!();
        /*
            #ifdef ALOE_LITTLE_ENDIAN
        union { uint32 asInt; float asFloat; } n;
        n.asFloat = x;
        n.asInt = ByteOrder::swap (n.asInt);
        return n.asFloat;
       #else
        return x;
       #endif
        */
}

pub fn get_vst_host_time_nanoseconds() -> f64 {
    
    todo!();
        /*
            #if ALOE_WINDOWS
            return timeGetTime() * 1000000.0;
           #elif ALOE_LINUX || ALOE_BSD || ALOE_IOS || ALOE_ANDROID
            timeval micro;
            gettimeofday (&micro, nullptr);
            return (double) micro.tv_usec * 1000.0;
           #elif ALOE_MAC
            UnsignedWide micro;
            Microseconds (&micro);
            return micro.lo * 1000.0;
           #endif
        */
}

lazy_static!{
    /*
    static int shellUIDToCreate = 0;
        static int insideVSTCallback = 0;
    */
}

type FSRef = Missing;
///-------------------
#[cfg(target_os="macos")]
pub fn make_fs_ref_from_path(
    dest_fs_ref: *mut FSRef,
    path:        &String
) -> bool {

    todo!();
        /*
            return FSPathMakeRef (reinterpret_cast<const UInt8*> (path.toRawUTF8()), destFSRef, nullptr) == noErr;
        */
}

type AudioMasterCallback = Missing;

#[VSTCALLBACK]
pub type MainCall = fn(AudioMasterCallback);

/**
  | Change this to disable logging of various
  | VST activities
  |
  */
#[cfg(not(VST_LOGGING))]
pub const VST_LOGGING: bool = true;

#[cfg(VST_LOGGING)]
macro_rules! aloe_vst_log {
    ($a:ident) => {
        /*
                Logger::writeToLog(a);
        */
    }
}

#[cfg(not(VST_LOGGING))]
macro_rules! aloe_vst_log { ($a:ident) => { } }

#[cfg(any(target_os="linux",target_os="bsd"))]
pub type event_proc_ptr = fn(_0: *mut XEvent) -> c_void;

#[cfg(any(target_os="linux",target_os="bsd"))]
pub fn get_child_window(window_to_check: Window) -> Window {
    
    todo!();
        /*
            Window rootWindow, parentWindow;
            Window* childWindows;
            unsigned int numChildren = 0;

            X11Symbols::getInstance()->xQueryTree (XWindowSystem::getInstance()->getDisplay(),
                                                   windowToCheck, &rootWindow, &parentWindow, &childWindows, &numChildren);

            if (numChildren > 0)
                return childWindows [0];

            return 0;
        */
}

pub const default_vst_sample_rate_value: i32 = 44100;
pub const default_vst_block_size_value:  i32 = 512;

#[cfg(not(any(target_os="ios",target_os="android")))]
lazy_static!{
    /*
    static Vec<VSTPluginWindow*> activeVSTWindows;
    */
}

/**
  | entry point for all callbacks from the
  | plugin
  |
  */
#[VSTCALLBACK]
pub fn audio_master(
    effect: *mut AEffect,
    opcode: i32,
    index:  i32,
    value:  PointerSizedInt,
    ptr:    *mut c_void,
    opt:    f32

) -> PointerSizedInt {

    todo!();
        /*
            if (effect != nullptr)
            if (auto* instance = (VSTPluginInstance*) (effect->resvd2))
                return instance->handleCallback (opcode, index, value, ptr, opt);

        return VSTPluginInstance::handleGeneralCallback (opcode, index, value, ptr, opt);
        */
}

pub fn create_and_update_desc<'a>(
    format: &mut VSTPluginFormat,
    desc:   &mut PluginDescription

) -> Box<VSTPluginInstance<'a>> {

    todo!();
        /*
            if (auto p = format.createInstanceFromDescription (desc, 44100.0, 512))
        {
            if (auto instance = dynamic_cast<VSTPluginInstance*> (p.release()))
            {
               #if ALOE_MAC
                if (instance->vstModule->resFileId != 0)
                    UseResFile (instance->vstModule->resFileId);
               #endif

                instance->fillInPluginDescription (desc);
                return std::unique_ptr<VSTPluginInstance> (instance);
            }

            jassertfalse;
        }

        return {};
        */
}

