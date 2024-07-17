crate::ix!();

#[no_copy]
#[leak_detector]
pub struct Vst3ModuleHandle {
    base:    ReferenceCountedObject,
    file:    File,
    name:    String,
    is_open: bool, // default = false
}

impl Drop for Vst3ModuleHandle {

    fn drop(&mut self) {
        todo!();
        /*
            if (isOpen)
                getActiveModules().removeFirstMatchingValue (this);
        */
    }
}

pub type Vst3ModuleHandlePtr = ReferenceCountedObjectPtr<Vst3ModuleHandle>;

impl Vst3ModuleHandle {

    pub fn new(
        plugin_file: &File,
        plugin_desc: &PluginDescription) -> Self {
    
        todo!();
        /*
        : file(pluginFile),

            if (open (pluginDesc))
            {
                isOpen = true;
                getActiveModules().add (this);
            }
        */
    }
    
    pub fn find_or_create_module(
        file:        &File,
        description: &PluginDescription

    ) -> Vst3ModuleHandlePtr {
        
        todo!();
        /*
            for (auto* module : getActiveModules())
            {
                // Vst3s are basically shells, you must therefore check their name along with their file:
                if (module->file == file && module->name == description.name)
                    return module;
            }

            Vst3ModuleHandle::Ptr modulePtr (new Vst3ModuleHandle (file, description));

            if (! modulePtr->isOpen)
                modulePtr = nullptr;

            return modulePtr;
        */
    }
    
    pub fn get_plugin_factory(&mut self) -> *mut dyn IPluginFactory {
        
        todo!();
        /*
            return DLLHandleCache::getInstance()->findOrCreateHandle (file.getFullPathName()).getPluginFactory();
        */
    }
    
    pub fn get_file(&self) -> File {
        
        todo!();
        /*
            return file;
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return name;
        */
    }
    
    pub fn get_active_modules<'a>() -> &'a mut Vec<*mut Vst3ModuleHandle> {
        
        todo!();
        /*
            static Vec<Vst3ModuleHandle*> activeModules;
            return activeModules;
        */
    }
    
    pub fn open(&mut self, description: &PluginDescription) -> bool {
        
        todo!();
        /*
            VstComSmartPtr<IPluginFactory> pluginFactory (DLLHandleCache::getInstance()->findOrCreateHandle (file.getFullPathName())
                                                                                        .getPluginFactory());

            if (pluginFactory != nullptr)
            {
                auto numClasses = pluginFactory->countClasses();

                for (i32 i = 0; i < numClasses; ++i)
                {
                    PClassInfo info;
                    pluginFactory->getClassInfo (i, &info);

                    if (std::strcmp (info.category, kVstAudioEffectClass) != 0)
                        continue;

                    if (toString (info.name).trim() == description.name
                        && (getHashForRange (getNormalisedTUID (info.cid)) == description.uniqueId
                            || getHashForRange (info.cid) == description.deprecatedUid))
                    {
                        name = description.name;
                        return true;
                    }
                }
            }

            return false;
        */
    }
}
