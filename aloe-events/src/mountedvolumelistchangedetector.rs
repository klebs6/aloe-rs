crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/messages/aloe_MountedVolumeListChangeDetector.h]

/**
  | An instance of this class will provide
  | callbacks when drives are mounted or
  | unmounted on the system.
  | 
  | Just inherit from this class and implement
  | the pure virtual method to get the callbacks,
  | there's no need to do anything else.
  | 
  | @see File::findFileSystemRoots()
  | 
  | @tags{Events}
  |
  */
#[cfg(any(target_os="macos",target_os="windows"))]
pub trait MountedVolumeListChangeDetectorInterface {

    /**
      | This method is called when a volume is
      | mounted or unmounted.
      |
      */
    fn mounted_volume_list_changed(&mut self);
}
