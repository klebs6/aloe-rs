crate::ix!();

type Handle         = Missing;
type CFBundleRefNum = Missing;
type FSSpec         = Missing;

#[no_copy]
#[leak_detector]
pub struct ModuleHandle {
    base:                            ReferenceCountedObject,
    file:                            File,
    module_main:                     MainCall,
    custom_main:                     MainCall, // default = {}
    plugin_name:                     String,
    vst_xml:                         Box<XmlElement>,

    #[cfg(not(target_os="macos"))]
    full_parent_directory_path_name: String,

    #[cfg(any(target_os="win32",target_os="linux",target_os="bsd",target_os="android"))]
    module:                          DynamicLibrary,

    #[cfg(not(any(target_os="win32",target_os="linux",target_os="bsd",target_os="android")))]
    res_handle:                      Handle,

    #[cfg(not(any(target_os="win32",target_os="linux",target_os="bsd",target_os="android")))]
    bundle_ref:                      CFUniquePtr<CFBundleRef>,

    #[cfg(target_os="macos")]
    #[cfg(not(any(target_os="win32",target_os="linux",target_os="bsd",target_os="android")))]
    res_file_id:                    CFBundleRefNum,

    #[cfg(target_os="macos")]
    #[cfg(not(any(target_os="win32",target_os="linux",target_os="bsd",target_os="android")))]
    parent_dir_fs_spec:             FSSpec,
}

pub type ModuleHandlePtr = ReferenceCountedObjectPtr<ModuleHandle>;

impl Drop for ModuleHandle {

    fn drop(&mut self) {
        todo!();
        /*
            getActiveModules().removeFirstMatchingValue (this);
            close();
        */
    }
}

impl ModuleHandle {

    pub fn get_active_modules<'a>() -> &'a mut Vec<*mut ModuleHandle> {
        
        todo!();
        /*
            static Vec<ModuleHandle*> activeModules;
            return activeModules;
        */
    }
    
    pub fn find_or_create_module(file: &File) -> ModuleHandlePtr {
        
        todo!();
        /*
            for (auto* module : getActiveModules())
                if (module->file == file)
                    return module;

            const IdleCallRecursionPreventer icrp;
            shellUIDToCreate = 0;
            _fpreset();

            ALOE_VST_LOG ("Attempting to load VST: " + file.getFullPathName());

            Ptr m = new ModuleHandle (file, nullptr);

            if (m->open())
            {
                _fpreset();
                return m;
            }

            return {};
        */
    }
    
    pub fn new(
        f:                &File,
        custom_main_call: MainCall) -> Self {
    
        todo!();
        /*
        : file(f),
        : module_main(customMainCall),

            getActiveModules().add (this);

           #if ALOE_WINDOWS || ALOE_LINUX || ALOE_BSD || ALOE_IOS || ALOE_ANDROID
            fullParentDirectoryPathName = f.getParentDirectory().getFullPathName();
           #elif ALOE_MAC
            FSRef ref;
            makeFSRefFromPath (&ref, f.getParentDirectory().getFullPathName());
            FSGetCatalogInfo (&ref, kFSCatInfoNone, nullptr, nullptr, &parentDirFSSpec, nullptr);
           #endif
        */
    }

    #[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="android"))]
    pub fn open(&mut self) -> bool {
        
        todo!();
        /*
            if (moduleMain != nullptr)
                return true;

            pluginName = file.getFileNameWithoutExtension();

            module.open (file.getFullPathName());

            moduleMain = (MainCall) module.getFunction ("VSTPluginMain");

            if (moduleMain == nullptr)
                moduleMain = (MainCall) module.getFunction ("main");

            ALOE_VST_WRAPPER_LOAD_CUSTOM_MAIN

            if (moduleMain != nullptr)
            {
                vstXml = parseXML (file.withFileExtension ("vstxml"));

               #if ALOE_WINDOWS
                if (vstXml == nullptr)
                    vstXml = parseXML (getDLLResource (file, "VSTXML", 1));
               #endif
            }

            return moduleMain != nullptr;
        */
    }
    
    #[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="android"))]
    pub fn close(&mut self)  {
        
        todo!();
        /*
            _fpreset(); // (doesn't do any harm)

            module.close();
        */
    }
    
    #[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="android"))]
    pub fn close_effect(&mut self, eff: *mut Vst2AEffect)  {
        
        todo!();
        /*
            eff->dispatcher (eff, typename Vst2EffClose, 0, 0, nullptr, 0);
        */
    }

    #[cfg(any(target_os="windows",target_os="linux",target_os="bsd",target_os="android"))]
    #[cfg(target_os="windows")]
    pub fn get_dll_resource(
        dll_file: &File,
        ty:       &String,
        resid:    i32) -> String {
        
        todo!();
        /*
            DynamicLibrary dll (dllFile.getFullPathName());
            auto dllModule = (HMODULE) dll.getNativeHandle();

            if (dllModule != INVALID_HANDLE_VALUE)
            {
                if (auto res = FindResource (dllModule, MAKEINTRESOURCE (resID), type.toWideCharPointer()))
                {
                    if (auto hGlob = LoadResource (dllModule, res))
                    {
                        auto* data = static_cast<const char*> (LockResource (hGlob));
                        return String::fromUTF8 (data, SizeofResource (dllModule, res));
                    }
                }
            }

            return {};
        */
    }


    #[cfg(not(any(target_os="windows",target_os="linux",target_os="bsd",target_os="android")))]
    pub fn open(&mut self) -> bool {
        
        todo!();
        /*
            if (moduleMain != nullptr)
                return true;

            bool ok = false;

            if (file.hasFileExtension (".vst"))
            {
                auto* utf8 = file.getFullPathName().toRawUTF8();

                if (auto url = CFUniquePtr<CFURLRef> (CFURLCreateFromFileSystemRepresentation (nullptr, (const UInt8*) utf8,
                                                                                               (CFIndex) strlen (utf8), file.isDirectory())))
                {
                    bundleRef.reset (CFBundleCreate (kCFAllocatorDefault, url.get()));

                    if (bundleRef != nullptr)
                    {
                        if (CFBundleLoadExecutable (bundleRef.get()))
                        {
                            moduleMain = (MainCall) CFBundleGetFunctionPointerForName (bundleRef.get(), CFSTR("main_macho"));

                            if (moduleMain == nullptr)
                                moduleMain = (MainCall) CFBundleGetFunctionPointerForName (bundleRef.get(), CFSTR("VSTPluginMain"));

                            ALOE_VST_WRAPPER_LOAD_CUSTOM_MAIN

                            if (moduleMain != nullptr)
                            {
                                if (CFTypeRef name = CFBundleGetValueForInfoDictionaryKey (bundleRef.get(), CFSTR("CFBundleName")))
                                {
                                    if (CFGetTypeID (name) == CFStringGetTypeID())
                                    {
                                        char buffer[1024];

                                        if (CFStringGetCString ((CFStringRef) name, buffer, sizeof (buffer), CFStringGetSystemEncoding()))
                                            pluginName = buffer;
                                    }
                                }

                                if (pluginName.isEmpty())
                                    pluginName = file.getFileNameWithoutExtension();

                               #if ALOE_MAC
                                resFileId = CFBundleOpenBundleResourceMap (bundleRef.get());
                               #endif

                                ok = true;

                                auto vstXmlFiles = file
                                                       #if ALOE_MAC
                                                        .getChildFile ("Contents")
                                                        .getChildFile ("Resources")
                                                       #endif
                                                        .findChildFiles (File::findFiles, false, "*.vstxml");

                                if (! vstXmlFiles.isEmpty())
                                    vstXml = parseXML (vstXmlFiles.getReference(0));
                            }
                        }

                        if (! ok)
                        {
                            CFBundleUnloadExecutable (bundleRef.get());
                            bundleRef = nullptr;
                        }
                    }
                }
            }

            return ok;
        */
    }
    
    #[cfg(not(any(target_os="windows",target_os="linux",target_os="bsd",target_os="android")))]
    pub fn close(&mut self)  {
        
        todo!();
        /*
            if (bundleRef != nullptr)
            {
               #if ALOE_MAC
                CFBundleCloseBundleResourceMap (bundleRef.get(), resFileId);
               #endif

                if (CFGetRetainCount (bundleRef.get()) == 1)
                    CFBundleUnloadExecutable (bundleRef.get());

                if (CFGetRetainCount (bundleRef.get()) > 0)
                    bundleRef = nullptr;
            }
        */
    }
    
    #[cfg(not(any(target_os="windows",target_os="linux",target_os="bsd",target_os="android")))]
    pub fn close_effect(&mut self, eff: *mut AEffect)  {
        
        todo!();
        /*
            eff->dispatcher (eff, typename Vst2EffClose, 0, 0, nullptr, 0);
        */
    }
}
