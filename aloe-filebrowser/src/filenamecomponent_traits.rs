crate::ix!();

/**
  | Listens for events happening to a FilenameComponent.
  | 
  | Use FilenameComponent::addListener()
  | and FilenameComponent::removeListener()
  | to register one of these objects for
  | event callbacks when the filename is
  | changed.
  | 
  | @see FilenameComponent
  | 
  | @tags{GUI}
  |
  */
pub trait FilenameComponentListener {

    /**
      | This method is called after the FilenameComponent's
      | file has been changed.
      |
      */
    fn filename_component_changed(&mut self, file_component_that_has_changed: *mut FilenameComponent);
}

pub trait GetLocationToBrowse {

    /**
      | This can be overridden to return a custom
      | location that you want the dialog box
      | to show when the browse button is pushed.
      | 
      | The default implementation of this
      | method will return either the current
      | file (if one has been chosen) or the location
      | that was set by setDefaultBrowseTarget().
      |
      */
    fn get_location_to_browse(&mut self) -> File;
}
