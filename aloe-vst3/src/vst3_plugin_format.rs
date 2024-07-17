crate::ix!();

/**
  | Implements a plugin format for Vst3s.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Vst3PluginFormat {
    base: AudioPluginFormat,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_Vst3PluginFormat.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_Vst3PluginFormat.cpp]
impl Vst3PluginFormat {
    
    pub fn get_format_name() -> String {
        
        todo!();
        /*
            return "Vst3";
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
    
    pub fn set_state_from_vst_preset_file(&mut self, 
        api:      *mut AudioPluginInstance,
        raw_data: &MemoryBlock) -> bool {
        
        todo!();
        /*
            if (auto vst3 = dynamic_cast<Vst3PluginInstance*> (api))
            return vst3->setStateFromPresetFile (rawData);

        return false;
        */
    }
    
    pub fn find_all_types_for_file(&mut self, 
        results:            &mut Vec<Box<PluginDescription>>,
        file_or_identifier: &String)  {
        
        todo!();
        /*
            if (fileMightContainThisPluginType (fileOrIdentifier))
        {
            /**
                Since there is no apparent indication if a Vst3 plugin is a shell or not,
                we're stuck iterating through a Vst3's factory, creating a description
                for every housed plugin.
            */

            VstComSmartPtr<IPluginFactory> pluginFactory (DLLHandleCache::getInstance()->findOrCreateHandle (fileOrIdentifier)
                                                                                        .getPluginFactory());

            if (pluginFactory != nullptr)
            {
                VstComSmartPtr<Vst3HostContext> host (new Vst3HostContext());
                DescriptionLister lister (host, pluginFactory);
                lister.findDescriptionsAndPerform (File (fileOrIdentifier));

                results.addCopiesOf (lister.list);
            }
            else
            {
                jassertfalse;
            }
        }
        */
    }
    
    pub fn create_plugin_instance(&mut self, 
        description: &PluginDescription,
        _1:          f64,
        _2:          i32,
        callback:    PluginCreationCallback)  {
        
        todo!();
        /*
            std::unique_ptr<Vst3PluginInstance> result;

        if (fileMightContainThisPluginType (description.fileOrIdentifier))
        {
            File file (description.fileOrIdentifier);

            auto previousWorkingDirectory = File::getCurrentWorkingDirectory();
            file.getParentDirectory().setAsCurrentWorkingDirectory();

            if (const Vst3ModuleHandle::Ptr module = Vst3ModuleHandle::findOrCreateModule (file, description))
            {
                std::unique_ptr<Vst3ComponentHolder> holder (new Vst3ComponentHolder (module));

                if (holder->initialise())
                {
                    result.reset (new Vst3PluginInstance (holder.release()));

                    if (! result->initialise())
                        result.reset();
                }
            }

            previousWorkingDirectory.setAsCurrentWorkingDirectory();
        }

        String errorMsg;

        if (result == nullptr)
            errorMsg = TRANS ("Unable to load XXX plug-in file").replace ("XXX", "Vst-3");

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

        return f.hasFileExtension (".vst3")
              #if ALOE_MAC || ALOE_LINUX || ALOE_BSD
               && f.exists();
              #else
               && f.existsAsFile();
              #endif
        */
    }
    
    pub fn get_name_of_plugin_from_identifier(&mut self, file_or_identifier: &String) -> String {
        
        todo!();
        /*
            return fileOrIdentifier; //Impossible to tell because every Vst3 is a type of shell...
        */
    }
    
    pub fn plugin_needs_rescanning(&mut self, description: &PluginDescription) -> bool {
        
        todo!();
        /*
            return File (description.fileOrIdentifier).getLastModificationTime() != description.lastFileModTime;
        */
    }
    
    pub fn does_plugin_still_exist(&mut self, description: &PluginDescription) -> bool {
        
        todo!();
        /*
            return File (description.fileOrIdentifier).exists();
        */
    }
    
    pub fn search_paths_for_plugins(&mut self, 
        directories_to_search: &FileSearchPath,
        recursive:             bool,
        _2:                    bool) -> StringArray {
        
        todo!();
        /*
            StringArray results;

        for (int i = 0; i < directoriesToSearch.getNumPaths(); ++i)
            recursiveFileSearch (results, directoriesToSearch[i], recursive);

        return results;
        */
    }
    
    pub fn recursive_file_search(&mut self, 
        results:   &mut StringArray,
        directory: &File,
        recursive: bool)  {
        
        todo!();
        /*
            for (const auto& iter : RangedDirectoryIterator (directory, false, "*", File::findFilesAndDirectories))
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
            #if ALOE_WINDOWS
        auto programFiles = File::getSpecialLocation (File::globalApplicationsDirectory).getFullPathName();
        return FileSearchPath (programFiles + "\\Common Files\\Vst3");
       #elif ALOE_MAC
        return FileSearchPath ("/Library/Audio/Plug-Ins/Vst3;~/Library/Audio/Plug-Ins/Vst3");
       #else
        return FileSearchPath ("/usr/lib/vst3/;/usr/local/lib/vst3/;~/.vst3/");
       #endif
        */
    }
}
