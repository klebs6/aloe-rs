crate::ix!();

pub const DLL_HANDLE_FACTORY_FN_NAME: &'static str = "GetPluginFactory";

#[cfg(target_os="windows")]
pub const DLL_HANDLE_ENTRY_FN_NAME: &'static str = "InitDll";

#[cfg(target_os="windows")]
pub const DLL_HANDLE_EXIT_FN_NAME: &'static str = "ExitDll";

#[cfg(target_os="windows")]
#[PLUGIN_API]
pub type DLL_HANDLE_ENTRY_PROC = fn() -> bool;

///-------------------------
#[cfg(any(target_os="linux",target_os="bsd"))]
pub const DLL_HANDLE_ENTRY_FN_NAME: &'static str = "ModuleEntry";

#[cfg(any(target_os="linux",target_os="bsd"))]
pub const DLL_HANDLE_EXIT_FN_NAME:  &'static str = "ModuleExit";

#[cfg(any(target_os="linux",target_os="bsd"))]
#[PLUGIN_API]
pub type DLL_HANDLE_ENTRY_PROC = fn(_0: *mut c_void) -> bool;

///-------------------------
#[cfg(target_os="macos")]
pub const DLL_HANDLE_ENTRY_FN_NAME: &'static str = "bundleEntry";

#[cfg(target_os="macos")]
pub const DLL_HANDLE_EXIT_FN_NAME:  &'static str = "bundleExit";

#[cfg(target_os="macos")]
#[PLUGIN_API]
pub type DLL_HANDLE_ENTRY_PROC = fn(_0: CFBundleRef) -> bool;

//--------------------------------
#[no_copy]
#[leak_detector]
pub struct DLLHandle {

    dll_file: File,
    factory:  *mut dyn IPluginFactory, // default = nullptr

    #[cfg(any(any(target_os="windows",target_os="linux"),target_os="bsd"))]
    library: DynamicLibrary,

    #[cfg(target_os="macos")]
    bundle_ref: CFUniquePtr<CFBundleRef>,
}

impl Drop for DLLHandle {

    fn drop(&mut self) {
        todo!();
        /*
            #if ALOE_MAC
            if (bundleRef != nullptr)
           #endif
            {
                if (factory != nullptr)
                    factory->release();

                using ExitModuleFn = bool (PLUGIN_API*) ();

                if (auto* exitFn = (ExitModuleFn) getFunction (exitFnName))
                    exitFn();

               #if ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD
                library.close();
               #endif
            }
        */
    }
}

impl DLLHandle {

    pub fn new(file_to_open: &File) -> Self {
    
        todo!();
        /*
        : dll_file(fileToOpen),

            open();
        */
    }
    
    /**
      | The factory should begin with a refCount
      | of 1, so don't increment the reference
      | count (ie: don't use a VstComSmartPtr
      | in here)! Its lifetime will be handled
      | by this DLLHandle.
      |
      */
    pub fn get_plugin_factory(&mut self) -> *mut dyn IPluginFactory {
        
        todo!();
        /*
            if (factory == nullptr)
                if (auto* proc = (GetFactoryProc) getFunction (factoryFnName))
                    factory = proc();

            // The plugin NEEDS to provide a factory to be able to be called a Vst3!
            // Most likely you are trying to load a 32-bit Vst3 from a 64-bit host
            // or vice versa.
            jassert (factory != nullptr);
            return factory;
        */
    }
    
    pub fn get_function(&mut self, function_name: *const u8)  {
        
        todo!();
        /*
            #if ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD
            return library.getFunction (functionName);
           #elif ALOE_MAC
            if (bundleRef == nullptr)
                return nullptr;

            CFUniquePtr<CFStringRef> name (String (functionName).toCFString());
            return CFBundleGetFunctionPointerForName (bundleRef.get(), name.get());
           #endif
        */
    }
    
    pub fn get_file(&self) -> File {
        
        todo!();
        /*
            return dllFile;
        */
    }

    #[cfg(any(any(target_os="windows",target_os="linux"),target_os="bsd"))]
    pub fn open(&mut self) -> bool {
        
        todo!();
        /*
            if (library.open (dllFile.getFullPathName()))
            {
                if (auto* proc = (EntryProc) getFunction (entryFnName))
                {
                   #if ALOE_WINDOWS
                    if (proc())
                   #else
                    if (proc (library.getNativeHandle()))
                   #endif
                        return true;
                }
                else
                {
                    // this is required for some plug-ins which don't export the dll entry point function
                    return true;
                }

                library.close();
            }

            return false;
        */
    }

    #[cfg(target_os="macos")]
    pub fn open(&mut self) -> bool {
        
        todo!();
        /*
            auto* utf8 = dllFile.getFullPathName().toRawUTF8();

            if (auto url = CFUniquePtr<CFURLRef> (CFURLCreateFromFileSystemRepresentation (nullptr,
                                                                                           (const UInt8*) utf8,
                                                                                           (CFIndex) std::strlen (utf8),
                                                                                           dllFile.isDirectory())))
            {
                bundleRef.reset (CFBundleCreate (kCFAllocatorDefault, url.get()));

                if (bundleRef != nullptr)
                {
                    CFObjectHolder<CFErrorRef> error;

                    if (CFBundleLoadExecutableAndReturnError (bundleRef.get(), &error.object))
                        if (auto* proc = (EntryProc) getFunction (entryFnName))
                            if (proc (bundleRef.get()))
                                return true;

                    if (error.object != nullptr)
                        if (auto failureMessage = CFUniquePtr<CFStringRef> (CFErrorCopyFailureReason (error.object)))
                            DBG (String::fromCFString (failureMessage.get()));

                    bundleRef = nullptr;
                }
            }

            return false;
        */
    }
}
