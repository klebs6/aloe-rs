crate::ix!();

pub fn read_dead_mans_pedal_file(file: &File) -> StringArray {
    
    todo!();
        /*
            StringArray lines;
        file.readLines (lines);
        lines.removeEmptyStrings();
        return lines;
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/scanning/aloe_PluginDirectoryScanner.h]

/**
  | Scans a directory for plugins, and adds
  | them to a KnownPluginList.
  | 
  | To use one of these, create it and call
  | scanNextFile() repeatedly, until
  | it returns false.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct PluginDirectoryScanner<'a> {
    list:                         KnownPluginList<'a>,
    format:                       AudioPluginFormat,
    files_or_identifiers_to_scan: StringArray,
    dead_mans_pedal_file:         File,
    failed_files:                 StringArray,
    next_index:                   Atomic<i32>,
    progress:                     f32, // default = 0
    allow_async:                  bool,
}

impl<'a> Drop for PluginDirectoryScanner<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            list.scanFinished();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/scanning/aloe_PluginDirectoryScanner.cpp]
impl<'a> PluginDirectoryScanner<'a> {

    /**
      | Returns the estimated progress, between
      | 0 and 1.
      |
      */
    pub fn get_progress(&self) -> f32 {
        
        todo!();
        /*
            return progress;
        */
    }

    /**
      | This returns a list of all the filenames
      | of things that looked like being a plugin
      | file, but which failed to open for some
      | reason.
      |
      */
    pub fn get_failed_files(&self) -> &StringArray {
        
        todo!();
        /*
            return failedFiles;
        */
    }

    /**
      | Creates a scanner.
      | 
      | -----------
      | @param listToAddResultsTo
      | 
      | this will get the new types added to it.
      | ----------
      | @param formatToLookFor
      | 
      | this is the type of format that you want
      | to look for
      | ----------
      | @param directoriesToSearch
      | 
      | the path to search
      | ----------
      | @param searchRecursively
      | 
      | true to search recursively
      | ----------
      | @param deadMansPedalFile
      | 
      | if this isn't File(), then it will be
      | used as a file to store the names of any
      | plugins that crash during initialisation.
      | If there are any plugins listed in it,
      | then these will always be scanned after
      | all other possible files have been tried
      | - in this way, even if there's a few dodgy
      | plugins in your path, then a couple of
      | rescans will still manage to find all
      | the proper plugins.
      | 
      | It's probably best to choose a file in
      | the user's application data directory
      | (alongside your app's settings file)
      | for this. The file format it uses is just
      | a list of filenames of the modules that
      | failed.
      | ----------
      | @param allowPluginsWhichRequireAsynchronousInstantiation
      | 
      | If this is false then the scanner will
      | exclude plug-ins asynchronous creation
      | - such as AUv3 plug-ins.
      |
      */
    pub fn new(
        list_to_add_to:                                         &mut KnownPluginList,
        format_to_look_for:                                     &mut AudioPluginFormat,
        directories_to_search:                                  FileSearchPath,
        recursive:                                              bool,
        dead_mans_pedal:                                        &File,
        allow_plugins_which_require_asynchronous_instantiation: Option<bool>

    ) -> Self {

        let allow_plugins_which_require_asynchronous_instantiation: bool =
            allow_plugins_which_require_asynchronous_instantiation.unwrap_or(false);
    
        todo!();
        /*
        : list(listToAddTo),
        : format(formatToLookFor),
        : dead_mans_pedal_file(deadMansPedal),
        : allow_async(allowPluginsWhichRequireAsynchronousInstantiation),

            directoriesToSearch.removeRedundantPaths();
        setFilesOrIdentifiersToScan (format.searchPathsForPlugins (directoriesToSearch, recursive, allowAsync));
        */
    }
    
    /**
      | Sets a specific list of filesOrIdentifiersToScan
      | to scan.
      | 
      | N.B. This list must match the format
      | passed to the constructor. @see AudioPluginFormat::searchPathsForPlugins
      |
      */
    pub fn set_files_or_identifiers_to_scan(&mut self, files_or_identifiers: &StringArray)  {
        
        todo!();
        /*
            filesOrIdentifiersToScan = filesOrIdentifiers;

        // If any plugins have crashed recently when being loaded, move them to the
        // end of the list to give the others a chance to load correctly..
        for (auto& crashed : readDeadMansPedalFile (deadMansPedalFile))
            for (int j = filesOrIdentifiersToScan.size(); --j >= 0;)
                if (crashed == filesOrIdentifiersToScan[j])
                    filesOrIdentifiersToScan.move (j, -1);

        applyBlacklistingsFromDeadMansPedal (list, deadMansPedalFile);
        nextIndex.set (filesOrIdentifiersToScan.size());
        */
    }
    
    /**
      | Returns the description of the plugin
      | that will be scanned during the next
      | call to scanNextFile().
      | 
      | This is handy if you want to show the user
      | which file is currently getting scanned.
      |
      */
    pub fn get_next_plugin_file_that_will_be_scanned(&self) -> String {
        
        todo!();
        /*
            return format.getNameOfPluginFromIdentifier (filesOrIdentifiersToScan [nextIndex.get() - 1]);
        */
    }
    
    pub fn update_progress(&mut self)  {
        
        todo!();
        /*
            progress = (1.0f - (float) nextIndex.get() / (float) filesOrIdentifiersToScan.size());
        */
    }
    
    /**
      | Tries the next likely-looking file.
      | 
      | If dontRescanIfAlreadyInList is true,
      | then the file will only be loaded and
      | re-tested if it's not already in the
      | list, or if the file's modification
      | time has changed since the list was created.
      | If dontRescanIfAlreadyInList is false,
      | the file will always be reloaded and
      | tested.
      | 
      | The nameOfPluginBeingScanned will
      | be updated to the name of the plugin being
      | scanned before the scan starts.
      | 
      | Returns false when there are no more
      | files to try.
      |
      */
    pub fn scan_next_file(&mut self, 
        dont_rescan_if_already_in_list: bool,
        name_of_plugin_being_scanned:   &mut String) -> bool {
        
        todo!();
        /*
            const int index = --nextIndex;

        if (index >= 0)
        {
            auto file = filesOrIdentifiersToScan [index];

            if (file.isNotEmpty() && ! (dontRescanIfAlreadyInList && list.isListingUpToDate (file, format)))
            {
                nameOfPluginBeingScanned = format.getNameOfPluginFromIdentifier (file);

                Vec<Box<PluginDescription>> typesFound;

                // Add this plugin to the end of the dead-man's pedal list in case it crashes...
                auto crashedPlugins = readDeadMansPedalFile (deadMansPedalFile);
                crashedPlugins.removeString (file);
                crashedPlugins.add (file);
                setDeadMansPedalFile (crashedPlugins);

                list.scanAndAddFile (file, dontRescanIfAlreadyInList, typesFound, format);

                // Managed to load without crashing, so remove it from the dead-man's-pedal..
                crashedPlugins.removeString (file);
                setDeadMansPedalFile (crashedPlugins);

                if (typesFound.size() == 0 && ! list.getBlacklistedFiles().contains (file))
                    failedFiles.add (file);
            }
        }

        updateProgress();
        return index > 0;
        */
    }
    
    /**
      | Skips over the next file without scanning
      | it.
      | 
      | Returns false when there are no more
      | files to try.
      |
      */
    pub fn skip_next_file(&mut self) -> bool {
        
        todo!();
        /*
            updateProgress();
        return --nextIndex > 0;
        */
    }
    
    pub fn set_dead_mans_pedal_file(&mut self, new_contents: &StringArray)  {
        
        todo!();
        /*
            if (deadMansPedalFile.getFullPathName().isNotEmpty())
            deadMansPedalFile.replaceWithText (newContents.joinIntoString ("\n"), true, true);
        */
    }
    
    /**
      | Reads the given dead-mans-pedal file
      | and applies its contents to the list.
      |
      */
    pub fn apply_blacklistings_from_dead_mans_pedal(&mut self, 
        list: &mut KnownPluginList,
        file: &File)  {
        
        todo!();
        /*
            // If any plugins have crashed recently when being loaded, move them to the
        // end of the list to give the others a chance to load correctly..
        for (auto& crashedPlugin : readDeadMansPedalFile (file))
            list.addToBlacklist (crashedPlugin);
        */
    }
}
