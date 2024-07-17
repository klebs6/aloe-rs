crate::ix!();

//TODO
pub struct VstAudioBusBuffers             { }
pub struct VstBusDirection                { }
pub struct VstCString                     { }
pub struct VstChannelContextIInfoListener { }
pub struct VstCtrlNumber                  { }
pub struct VstEditController              { }
pub struct VstEditorView                  { }
pub struct VstIAttributeList              { }
pub struct VstIAudioProcessor             { }
pub struct VstIComponentHandler           { }
pub struct VstIConnectionPoint            { }
pub struct VstIContextMenu                { }
pub struct VstIHostApplication            { }
pub struct VstIMessage                    { }
pub struct VstIMidiMapping                { }
pub struct VstIParameterChanges           { }
pub struct VstIProcessContextRequirements { }
pub struct VstIUnitInfo                   { }
pub struct VstIoMode                      { }
pub struct VstMediaType                   { }
pub struct VstParamID                     { }
pub struct VstParamValue                  { }
pub struct VstParameter                   { }
pub struct VstProcessContext              { }
pub struct VstProcessData                 { }
pub struct VstProcessSetup                { }
pub struct VstProgramListID               { }
pub struct VstProgramListInfo             { }
pub struct VstSpeakerArrangement          { }
pub struct VstString128                   { }
pub struct VstTChar                       { }
pub struct VstUnitID                      { }
pub struct VstUnitInfo                    { }

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/Vst3/aloe_Vst3_Wrapper.cpp]

#[cfg(ALOE_PLUGINHOST_Vst3)]
pub const ALOE_Vst3HEADERS_INCLUDE_HEADERS_ONLY: usize = 1;
pub const ALOE_GUI_BASICS_INCLUDE_XHEADERS:      usize = 1;

#[cfg(not(ALOE_Vst3_CAN_REPLACE_Vst2))]
pub const ALOE_Vst3_CAN_REPLACE_Vst2: usize = 1;

#[cfg(not(ALOE_Vst3_EMULATE_MIDI_CC_WITH_PARAMETERS))]
#[cfg(AloePlugin_WantsMidiInput)]
pub const ALOE_Vst3_EMULATE_MIDI_CC_WITH_PARAMETERS: usize = 1;

#[cfg(any(target_os="linux",target_os="bsd"))]
pub fn get_fd_read_callbacks() -> Vec<(i32,fn(_0: i32) -> ())> {
    
    todo!();
        /*
        
        */
}

#[cfg(all(target_os="windows",ALOE_WIN_PER_MONITOR_DPI_AWARE))]
lazy_static!{
    /*
    extern  double getScaleFactorForWindow (HWND);
    */
}

pub fn query_additional_interfaces<Member>(
        processor: *mut dyn AudioProcessorInterface,
        targetiid: TUID,
        member:    Member) -> QueryInterfaceResult {

    todo!();
        /*
            if (processor == nullptr)
            return {};

        void* obj = nullptr;

        if (auto* extensions = dynamic_cast<Vst3ClientExtensions*> (processor))
        {
            const auto result = (extensions->*member) (targetIID, &obj);
            return { result, obj };
        }

        return {};
        */
}

pub fn extract_result(
        user_interface: &QueryInterfaceResult,
        aloe_interface: &InterfaceResultWithDeferredAddRef,
        obj:            *mut *mut c_void) -> tresult {
    
    todo!();
        /*
            if (userInterface.isOk() && aloeInterface.isOk())
        {
            // If you hit this assertion, you've provided a custom implementation of an interface
            // that Aloe implements already. As a result, your plugin may not behave correctly.
            // Consider removing your custom implementation.
            jassertfalse;

            return userInterface.extract (obj);
        }

        if (userInterface.isOk())
            return userInterface.extract (obj);

        return aloeInterface.extract (obj);
        */
}

lazy_static!{
    /*
    static thread_local bool inParameterChangedCallback = false;
    */
}

pub trait AudioBusPointerHelper: Float {
    fn _impl(data: VstAudioBusBuffers) -> *mut *mut Self;
}

impl AudioBusPointerHelper for f32 {

    fn _impl(data: VstAudioBusBuffers) -> *mut *mut Self { 
        todo!();
        /*
        data.channel_buffers_32 
        */
    }
}

impl AudioBusPointerHelper for f64 {

    fn _impl(data: VstAudioBusBuffers) -> *mut *mut Self { 
        todo!();
        /*
        data.channel_buffers_64 
        */
    }
}

pub trait ChooseBufferHelper: Float {
    fn _impl<'a>(f: &'a AudioBuffer<f32>, d: &'a AudioBuffer<f64>) -> &'a AudioBuffer<Self>;
}

impl ChooseBufferHelper for f32 {
    fn _impl<'a>(f: &'a AudioBuffer<f32>, d: &'a AudioBuffer<f64>) -> &'a AudioBuffer<Self> { f }
}

impl ChooseBufferHelper for f64 {
    fn _impl<'a>(f: &'a AudioBuffer<f32>, d: &'a AudioBuffer<f64>) -> &'a AudioBuffer<Self> { d }
}

declare_class_iid!{
    AloeAudioProcessor, 
    0x0101ABAB, 
    0xABCDEF01, 
    AloePlugin_ManufacturerCode, 
    AloePlugin_PluginCode
}

def_class_iid!{
    AloeAudioProcessor
}

#[cfg(ALOE_Vst3_CAN_REPLACE_Vst2)]
pub fn get_fuid_forvst2id(for_controlleruid: bool) -> FUID {
    
    todo!();
        /*
            TUID uuid;
         extern  void getUUIDForVst2ID (bool, uint8[16]);
         getUUIDForVst2ID (forControllerUID, (uint8*) uuid);
         return FUID (uuid);
        */
}

#[cfg(ALOE_Vst3_CAN_REPLACE_Vst2)]
lazy_static!{
    /*
    const FUID AloeVst3Component     ::iid (getFUIDForVst2ID (false));
     const FUID AloeVst3EditController::iid (getFUIDForVst2ID (true));
    */
}

#[cfg(not(ALOE_Vst3_CAN_REPLACE_Vst2))]
declare_class_iid!{
    AloeVst3EditController, 
    0xABCDEF01, 
    0x1234ABCD, 
    AloePlugin_ManufacturerCode, 
    AloePlugin_PluginCode
}

#[cfg(not(ALOE_Vst3_CAN_REPLACE_Vst2))]
def_class_iid!{
    AloeVst3EditController
}

#[cfg(not(ALOE_Vst3_CAN_REPLACE_Vst2))]
declare_class_iid!{
    AloeVst3Component, 
    0xABCDEF01, 
    0x9182FAEB, 
    AloePlugin_ManufacturerCode, 
    AloePlugin_PluginCode
}

#[cfg(not(ALOE_Vst3_CAN_REPLACE_Vst2))]
def_class_iid!{
    AloeVst3Component
}

pub fn init_module() -> bool {
    
    todo!();
        /*
            #if ALOE_MAC
        initialiseMacVst();
       #endif

        return true;
        */
}

pub fn shutdown_module() -> bool {
    
    todo!();
        /*
            return true;
        */
}

#[cfg(target_os="windows")]
extern "C" {

    #[__declspec (dllexport)]
    pub fn init_dll() -> bool {
        
        todo!();
            /*
                return initModule();
            */
    }

    #[__declspec (dllexport)] 
    pub fn exit_dll() -> bool {
        
        todo!();
            /*
                return shutdownModule();
            */
    }
}

#[cfg(any(target_os="linux",target_os="bsd"))]
lazy_static!{
    /*
    void* moduleHandle = nullptr;
     int moduleEntryCounter = 0;
    */
}

#[cfg(any(target_os="linux",target_os="bsd"))]
pub fn module_entry(shared_library_handle: *mut c_void) -> bool {
    
    todo!();
        /*
            if (++moduleEntryCounter == 1)
         {
             moduleHandle = sharedLibraryHandle;
             return initModule();
         }

         return true;
        */
}

#[cfg(any(target_os="linux",target_os="bsd"))]
pub fn module_exit() -> bool {
    
    todo!();
        /*
            if (--moduleEntryCounter == 0)
         {
             moduleHandle = nullptr;
             return shutdownModule();
         }

         return true;
        */
}

#[cfg(target_os="macos")]
lazy_static!{
    /*
    CFBundleRef globalBundleInstance = nullptr;
     uint32 numBundleRefs = 0;
     Vec<CFBundleRef> bundleRefs;

     char modulePath[MaxPathLength] = { 0 };
     void* moduleHandle = nullptr;
    */
}

#[cfg(target_os="macos")]
pub const MaxPathLength: usize = 2048;

#[cfg(target_os="macos")]
pub fn bundle_entry(ref_: CFBundleRef) -> bool {
    
    todo!();
        /*
            if (ref != nullptr)
         {
             ++numBundleRefs;
             CFRetain (ref);

             bundleRefs.add (ref);

             if (moduleHandle == nullptr)
             {
                 globalBundleInstance = ref;
                 moduleHandle = ref;

                 CFUniquePtr<CFURLRef> tempURL (CFBundleCopyBundleURL (ref));
                 CFURLGetFileSystemRepresentation (tempURL.get(), true, (UInt8*) modulePath, MaxPathLength);
             }
         }

         return initModule();
        */
}

#[cfg(target_os="macos")]
pub fn bundle_exit() -> bool {
    
    todo!();
        /*
            if (shutdownModule())
         {
             if (--numBundleRefs == 0)
             {
                 for (int i = 0; i < bundleRefs.size(); ++i)
                     CFRelease (bundleRefs.getUnchecked (i));

                 bundleRefs.clear();
             }

             return true;
         }

         return false;
        */
}

/**
  | This typedef represents Vst3's createInstance()
  | function signature
  |
  */
pub type CreateFunction = fn(_0: *mut VstIHostApplication) -> *mut dyn FUnknown;

pub fn create_component_instance(host: *mut VstIHostApplication) -> *mut dyn FUnknown {
    
    todo!();
        /*
            return static_cast<VstIAudioProcessor*> (new AloeVst3Component (host));
        */
}

pub fn create_controller_instance(host: *mut VstIHostApplication) -> *mut dyn FUnknown {
    
    todo!();
        /*
            return static_cast<VstIEditController*> (new AloeVst3EditController (host));
        */
}

lazy_static!{
    /*
    static AloePluginFactory* globalFactory = nullptr;
    */
}


#[cfg(not(AloePlugin_Vst3ComponentFlags))]
#[cfg(AloePlugin_IsSynth)]
macro_rules! aloe_plugin_vst_3component_flags {
    () => {
        /*
                VstkSimpleModeSupported
        */
    }
}

#[cfg(not(AloePlugin_Vst3ComponentFlags))]
#[cfg(not(AloePlugin_IsSynth))]
macro_rules! aloe_plugin_vst_3component_flags {
    () => {
        /*
                0
        */
    }
}

///--------------------------
#[cfg(not(AloePlugin_Vst3Category))]
#[cfg(AloePlugin_IsSynth)]
macro_rules! aloe_plugin_vst_3category {
    () => {
        /*
                VstPlugType::kInstrumentSynth
        */
    }
}

#[cfg(not(AloePlugin_Vst3Category))]
#[cfg(not(AloePlugin_IsSynth))]
macro_rules! aloe_plugin_vst_3category {
    () => {
        /*
                VstPlugType::kFx
        */
    }
}

///--------------------------
/**
  | The Vst3 plugin entry point.
  |
  */
#[SMTG_EXPORT_SYMBOL] 
#[PLUGIN_API]
pub fn get_plugin_factory() -> *mut dyn IPluginFactory {
    
    todo!();
        /*
            PluginHostType::aloePlugInClientCurrentWrapperType = AudioProcessor::wrapperType_Vst3;

           #if (ALOE_MSVC || (ALOE_WINDOWS && ALOE_CLANG)) && ALOE_32BIT
            // Cunning trick to force this function to be exported. Life's too short to
            // faff around creating .def files for this kind of thing.
            // Unnecessary for 64-bit builds because those don't use decorated function names.
            #pragma comment(linker, "/EXPORT:GetPluginFactory=_GetPluginFactory@0")
           #endif

            if (globalFactory == nullptr)
            {
                globalFactory = new AloePluginFactory();

                static const PClassInfo2 componentClass (AloeVst3Component::iid,
                                                         PClassInfo::kManyInstances,
                                                         kVstAudioEffectClass,
                                                         AloePlugin_Name,
                                                         AloePlugin_Vst3ComponentFlags,
                                                         AloePlugin_Vst3Category,
                                                         AloePlugin_Manufacturer,
                                                         AloePlugin_VersionString,
                                                         kVstVersionString);

                globalFactory->registerClass (componentClass, createComponentInstance);

                static const PClassInfo2 controllerClass (AloeVst3EditController::iid,
                                                          PClassInfo::kManyInstances,
                                                          kVstComponentControllerClass,
                                                          AloePlugin_Name,
                                                          AloePlugin_Vst3ComponentFlags,
                                                          AloePlugin_Vst3Category,
                                                          AloePlugin_Manufacturer,
                                                          AloePlugin_VersionString,
                                                          kVstVersionString);

                globalFactory->registerClass (controllerClass, createControllerInstance);
            }
            else
            {
                globalFactory->addRef();
            }

            return dynamic_cast<IPluginFactory*> (globalFactory);
        */
}

#[cfg(target_os="windows")]
#[WINAPI]
pub fn dll_main(
        instance: HINSTANCE,
        reason:   u64,
        _2:       LPVOID) -> bool {
    
    todo!();
        /*
            if (reason == DLL_PROCESS_ATTACH) Process::setCurrentModuleInstanceHandle (instance); return true;
        */
}

