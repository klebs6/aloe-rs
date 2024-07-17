crate::ix!();

pub trait SearchPathsForPlugins {

    /**
      | Searches a suggested set of directories
      | for any plugins in this format.
      | 
      | The path might be ignored, e.g. by AUs,
      | which are found by the OS rather than
      | manually.
      | 
      | -----------
      | @param directoriesToSearch
      | 
      | This specifies which directories shall
      | be searched for plug-ins.
      | ----------
      | @param recursive
      | 
      | Should the search recursively traverse
      | folders.
      | ----------
      | @param allowPluginsWhichRequireAsynchronousInstantiation
      | 
      | If this is false then plug-ins which
      | require asynchronous creation will
      | be excluded.
      |
      */
    fn search_paths_for_plugins(&mut self, 
        directories_to_search:                                  &FileSearchPath,
        recursive:                                              bool,
        allow_plugins_which_require_asynchronous_instantiation: bool) -> Vec<String>;
}

pub trait FindAllTypesForFile {

    /**
      | This tries to create descriptions for
      | all the plugin types available in a binary
      | module file.
      | 
      | The file will be some kind of DLL or bundle.
      | 
      | Normally there will only be one type
      | returned, but some plugins (e.g. VST
      | shells) can use a single DLL to create
      | a set of different plugin subtypes,
      | so in that case, each subtype is returned
      | as a separate object.
      |
      */
    fn find_all_types_for_file(&mut self, 
        results:            &mut Vec<Box<PluginDescription>>,
        file_or_identifier: &String);
}

pub trait GetDefaultLocationsToSearch {

    /**
      | Returns the typical places to look for
      | this kind of plugin.
      | 
      | -----------
      | @note
      | 
      | if this returns no paths, it means that
      | the format doesn't search in files or
      | folders, e.g. AudioUnits.
      |
      */
    fn get_default_locations_to_search(&mut self) -> FileSearchPath;
}
