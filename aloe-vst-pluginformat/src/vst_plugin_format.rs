crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_VSTPluginFormat.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_VSTPluginFormat.cpp]

/**
  | Implements a plugin format manager
  | for VSTs.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct VSTPluginFormat {
    base: AudioPluginFormat,
}

pub trait AboutToScanVstShellPlugin {

    fn about_to_scan_vst_shell_plugin(&mut self, _0: &PluginDescription);
}

impl AboutToScanVstShellPlugin for VSTPluginFormat {

    /**
      | Can be overridden to receive a callback
      | when each member of a shell plugin is
      | about to be tested during a call to findAllTypesForFile().
      | 
      | Only the name and uid members of the PluginDescription
      | are guaranteed to be valid when this
      | is called.
      |
      */
    fn about_to_scan_vst_shell_plugin(&mut self, _0: &PluginDescription) {

        todo!();
    }
}

impl VSTPluginFormat {
    
    pub fn get_format_name() -> String {
        
        todo!();
        /*
            return "VST";
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return getFormatName();
        */
    }
    
    pub fn can_scan_for_plugins(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn is_trivial_to_scan(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn find_all_types_for_file(&mut self, 
        results:            &mut Vec<Box<PluginDescription>>,
        file_or_identifier: &String)  {
        
        todo!();
        /*
            if (! fileMightContainThisPluginType (fileOrIdentifier))
            return;

        PluginDescription desc;
        desc.fileOrIdentifier = fileOrIdentifier;
        desc.uniqueId = desc.deprecatedUid = 0;

        auto instance = createAndUpdateDesc (*this, desc);

        if (instance == nullptr)
            return;

        if (instance->getVstCategory() != typename Vst2kPlugCategShell)
        {
            // Normal plugin...
            results.add (new PluginDescription (desc));

            instance->dispatch (typename Vst2EffOpen, 0, 0, nullptr, 0);
        }
        else
        {
            // It's a shell plugin, so iterate all the subtypes...
            for (;;)
            {
                char shellEffectName [256] = { 0 };
                auto uid = (int) instance->dispatch (typename Vst2EffShellGetNextPlugin, 0, 0, shellEffectName, 0);

                if (uid == 0)
                    break;

                desc.uniqueId = desc.deprecatedUid = uid;
                desc.name = shellEffectName;

                aboutToScanVSTShellPlugin (desc);

                std::unique_ptr<VSTPluginInstance> shellInstance (createAndUpdateDesc (*this, desc));

                if (shellInstance != nullptr)
                {
                    jassert (desc.deprecatedUid == uid);
                    jassert (desc.uniqueId == uid);
                    desc.hasSharedContainer = true;
                    desc.name = shellEffectName;

                    if (! arrayContainsPlugin (results, desc))
                        results.add (new PluginDescription (desc));
                }
            }
        }
        */
    }
    
    pub fn create_plugin_instance(&mut self, 
        desc:        &PluginDescription,
        sample_rate: f64,
        block_size:  i32,
        callback:    PluginCreationCallback)  {
        
        todo!();
        /*
            std::unique_ptr<VSTPluginInstance> result;

        if (fileMightContainThisPluginType (desc.fileOrIdentifier))
        {
            File file (desc.fileOrIdentifier);

            auto previousWorkingDirectory = File::getCurrentWorkingDirectory();
            file.getParentDirectory().setAsCurrentWorkingDirectory();

            if (auto module = ModuleHandle::findOrCreateModule (file))
            {
                shellUIDToCreate = desc.uniqueId != 0 ? desc.uniqueId : desc.deprecatedUid;

                result.reset (VSTPluginInstance::create (module, sampleRate, blockSize));

                if (result != nullptr && ! result->initialiseEffect (sampleRate, blockSize))
                    result.reset();
            }

            previousWorkingDirectory.setAsCurrentWorkingDirectory();
        }

        String errorMsg;

        if (result == nullptr)
            errorMsg = TRANS ("Unable to load XXX plug-in file").replace ("XXX", "VST-2");

        callback (std::move (result), errorMsg);
        */
    }
    
    pub fn requires_unblocked_message_thread_during_creation(&self, _0: &PluginDescription) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn file_might_contain_this_plugin_type(&mut self, file_or_identifier: &String) -> bool {
        
        todo!();
        /*
            auto f = File::createFileWithoutCheckingPath (fileOrIdentifier);

      #if ALOE_MAC || ALOE_IOS
        return f.isDirectory() && f.hasFileExtension (".vst");
      #elif ALOE_WINDOWS
        return f.existsAsFile() && f.hasFileExtension (".dll");
      #elif ALOE_LINUX || ALOE_BSD || ALOE_ANDROID
        return f.existsAsFile() && f.hasFileExtension (".so");
      #endif
        */
    }
    
    pub fn get_name_of_plugin_from_identifier(&mut self, file_or_identifier: &String) -> String {
        
        todo!();
        /*
            return fileOrIdentifier;
        */
    }
    
    pub fn plugin_needs_rescanning(&mut self, desc: &PluginDescription) -> bool {
        
        todo!();
        /*
            return File (desc.fileOrIdentifier).getLastModificationTime() != desc.lastFileModTime;
        */
    }
    
    pub fn does_plugin_still_exist(&mut self, desc: &PluginDescription) -> bool {
        
        todo!();
        /*
            return File (desc.fileOrIdentifier).exists();
        */
    }
    
    pub fn search_paths_for_plugins(&mut self, 
        directories_to_search: &FileSearchPath,
        recursive:             bool,
        _2:                    bool) -> StringArray {
        
        todo!();
        /*
            StringArray results;

        for (int j = 0; j < directoriesToSearch.getNumPaths(); ++j)
            recursiveFileSearch (results, directoriesToSearch [j], recursive);

        return results;
        */
    }
    
    pub fn recursive_file_search(&mut self, 
        results:   &mut StringArray,
        dir:       &File,
        recursive: bool)  {
        
        todo!();
        /*
            // avoid allowing the dir iterator to be recursive, because we want to avoid letting it delve inside
        // .component or .vst directories.
        for (const auto& iter : RangedDirectoryIterator (dir, false, "*", File::findFilesAndDirectories))
        {
            auto f = iter.getFile();
            bool isPlugin = false;

            if (fileMightContainThisPluginType (f.getFullPathName()))
            {
                isPlugin = true;
                results.add (f.getFullPathName());
            }

            if (recursive && (! isPlugin) && f.isDirectory())
                recursiveFileSearch (results, f, true);
        }
        */
    }
    
    pub fn get_default_locations_to_search(&mut self) -> FileSearchPath {
        
        todo!();
        /*
            #if ALOE_MAC
        return FileSearchPath ("~/Library/Audio/Plug-Ins/VST;/Library/Audio/Plug-Ins/VST");
       #elif ALOE_LINUX || ALOE_BSD || ALOE_ANDROID
        return FileSearchPath (SystemStats::getEnvironmentVariable ("VST_PATH",
                                                                    "/usr/lib/vst;/usr/local/lib/vst;~/.vst")
                                 .replace (":", ";"));
       #elif ALOE_WINDOWS
        auto programFiles = File::getSpecialLocation (File::globalApplicationsDirectory).getFullPathName();

        FileSearchPath paths;
        paths.add (WindowsRegistry::getValue ("HKEY_LOCAL_MACHINE\\Software\\VST\\VSTPluginsPath"));
        paths.addIfNotAlreadyThere (programFiles + "\\Steinberg\\VstPlugins");
        paths.addIfNotAlreadyThere (programFiles + "\\VstPlugins");
        paths.removeRedundantPaths();
        return paths;
       #elif ALOE_IOS
        // on iOS you can only load plug-ins inside the hosts bundle folder
        CFUniquePtr<CFURLRef> relativePluginDir (CFBundleCopyBuiltInPlugInsURL (CFBundleGetMainBundle()));
        CFUniquePtr<CFURLRef> pluginDir (CFURLCopyAbsoluteURL (relativePluginDir.get()));

        CFUniquePtr<CFStringRef> path (CFURLCopyFileSystemPath (pluginDir.get(), kCFURLPOSIXPathStyle));
        FileSearchPath retval (String (CFStringGetCStringPtr (path.get(), kCFStringEncodingUTF8)));
        return retval;
       #endif
        */
    }
    
    /**
      | Attempts to retrieve the VSTXML data
      | from a plugin.
      | 
      | Will return nullptr if the plugin isn't
      | a VST, or if it doesn't have any VSTXML.
      |
      */
    pub fn getvstxml(&mut self, plugin: *mut AudioPluginInstance) -> *const XmlElement {
        
        todo!();
        /*
            if (auto* vst = dynamic_cast<VSTPluginInstance*> (plugin))
            if (vst->vstModule != nullptr)
                return vst->vstModule->vstXml.get();

        return nullptr;
        */
    }
    
    /**
      | Attempts to reload a VST plugin's state
      | from some FXB or FXP data.
      |
      */
    pub fn load_from_fxb_file(&mut self, 
        plugin:    *mut AudioPluginInstance,
        data:      *const c_void,
        data_size: usize) -> bool {
        
        todo!();
        /*
            if (auto* vst = dynamic_cast<VSTPluginInstance*> (plugin))
            return vst->loadFromFXBFile (data, dataSize);

        return false;
        */
    }
    
    /**
      | Attempts to save a VST's state to some
      | FXP or FXB data.
      |
      */
    pub fn save_to_fxb_file(&mut self, 
        plugin: *mut AudioPluginInstance,
        dest:   &mut MemoryBlock,
        asfxb:  bool) -> bool {
        
        todo!();
        /*
            if (auto* vst = dynamic_cast<VSTPluginInstance*> (plugin))
            return vst->saveToFXBFile (dest, asFXB);

        return false;
        */
    }
    
    /**
      | Attempts to get a VST's state as a chunk
      | of memory.
      |
      */
    pub fn get_chunk_data(&mut self, 
        plugin:    *mut AudioPluginInstance,
        result:    &mut MemoryBlock,
        is_preset: bool) -> bool {
        
        todo!();
        /*
            if (auto* vst = dynamic_cast<VSTPluginInstance*> (plugin))
            return vst->getChunkData (result, isPreset, 128);

        return false;
        */
    }
    
    /**
      | Attempts to set a VST's state from a chunk
      | of memory.
      |
      */
    pub fn set_chunk_data(&mut self, 
        plugin:    *mut AudioPluginInstance,
        data:      *const c_void,
        size:      i32,
        is_preset: bool) -> bool {
        
        todo!();
        /*
            if (auto* vst = dynamic_cast<VSTPluginInstance*> (plugin))
            return vst->setChunkData (data, size, isPreset);

        return false;
        */
    }
    
    /**
      | Given a suitable function pointer to
      | a VSTPluginMain function, this will
      | attempt to instantiate and return a
      | plugin for it.
      |
      */
    pub fn create_custom_vst_from_main_call(&mut self, 
        entry_point_function: *mut c_void,
        initial_sample_rate:  f64,
        initial_buffer_size:  i32) -> *mut AudioPluginInstance {
        
        todo!();
        /*
            ModuleHandle::Ptr module = new ModuleHandle (File(), (MainCall) entryPointFunction);

        if (module->open())
        {
            std::unique_ptr<VSTPluginInstance> result (VSTPluginInstance::create (module, initialSampleRate, initialBufferSize));

            if (result != nullptr)
                if (result->initialiseEffect (initialSampleRate, initialBufferSize))
                    return result.release();
        }

        return nullptr;
        */
    }
    
    /**
      | Provides an VstPluginFormatExtraFunctions callback
      | object for a plugin to use.
      | 
      | The plugin will take ownership of the
      | object and will delete it automatically.
      |
      */
    pub fn set_extra_functions(
        &mut self, 
        plugin:    *mut AudioPluginInstance,
        functions: *mut dyn VstPluginFormatExtraFunctions
    ) {
        
        todo!();
        /*
            std::unique_ptr<VstPluginFormatExtraFunctions> f (functions);

        if (auto* vst = dynamic_cast<VSTPluginInstance*> (plugin))
            std::swap (vst->extraFunctions, f);
        */
    }
    
    /**
      | Given a VstEffectInterface* (aka vst::AEffect*),
      | this will return the aloe AudioPluginInstance
      | that is being used to wrap it
      |
      */
    pub fn get_plugin_instance_from_vst_effect_interface(&mut self, a_effect: *mut c_void) -> *mut AudioPluginInstance {
        
        todo!();
        /*
            if (auto* vstAEffect = reinterpret_cast<typename Vst2AEffect*> (aEffect))
            if (auto* instanceVST = reinterpret_cast<VSTPluginInstance*> (vstAEffect->resvd2))
                return dynamic_cast<AudioPluginInstance*> (instanceVST);

        return nullptr;
        */
    }
    
    /**
      | This simply calls directly to the VST's
      | AEffect::dispatcher() function.
      |
      */
    pub fn dispatcher(&mut self, 
        plugin: *mut AudioPluginInstance,
        opcode: i32,
        index:  i32,
        value:  PointerSizedInt,
        ptr:    *mut c_void,
        opt:    f32) -> PointerSizedInt {
        
        todo!();
        /*
            if (auto* vst = dynamic_cast<VSTPluginInstance*> (plugin))
            return vst->dispatch (opcode, index, value, ptr, opt);

        return {};
        */
    }
}
