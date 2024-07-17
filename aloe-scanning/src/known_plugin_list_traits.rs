crate::ix!();

/**
  | Sort methods used to change the order
  | of the plugins in the list.
  |
  */
pub enum KnownPluginListSortMethod
{
    defaultOrder = 0,
    sortAlphabetically,
    sortByCategory,
    sortByManufacturer,
    sortByFormat,
    sortByFileSystemLocation,
    sortByInfoUpdateTime
}

pub trait FindPluginTypesFor {

    /**
      | Attempts to load the given file and find
      | a list of plugins in it.
      | 
      | -----------
      | @return
      | 
      | true if the plugin loaded, false if it
      | crashed
      |
      */
    fn find_plugin_types_for(&mut self, 
            format:             &mut AudioPluginFormat,
            result:             &mut Vec<Box<PluginDescription>>,
            file_or_identifier: &String) -> bool;

}

pub trait ScanFinished {

    /**
      | Called when a scan has finished, to allow
      | clean-up of resources.
      |
      */
    fn scan_finished(&mut self);
}
