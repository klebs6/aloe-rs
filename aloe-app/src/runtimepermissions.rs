crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/misc/aloe_RuntimePermissions.h]

/**
  | Function type of runtime permission
  | request callbacks.
  |
  */
pub type RuntimePermissionsCallback = fn(_0: bool) -> ();

/**
  | Class to handle app runtime permissions
  | for certain functionality on some platforms.
  | 
  | The use of this class is currently only
  | required if the app should run on Android
  | API level 23 and higher.
  | 
  | On lower API levels, the permissions
  | are specified in the app manifest. On
  | iOS, runtime permission requests are
  | handled automatically by the Apple
  | APIs and not manually in the app code.
  | On Windows, OS X, and Linux, runtime
  | permissions are not used at all. In all
  | these cases, request() will simply
  | call through to the callback with no
  | overhead and pass true (making it safe
  | to use on all platforms).
  | 
  | For example, to enable audio recording
  | on Android in your cross-platform app,
  | you could modify your code as follows:
  | 
  | Old:
  | 
  | -----------
  | @code
  | 
  | audioDeviceManager.initialise (2, 2, nullptr, true, String(), nullptr);
  | 
  | New:
  | ----------
  | @code
  | 
  | RuntimePermissions::request (
  |     RuntimePermissions::audioRecording,
  |     [this] (bool wasGranted)
  |     {
  |          if (! wasGranted)
  |          {
  |              // e.g. display an error or initialise with 0 input channels
  |              return;
  |          }
  | 
  |          audioDeviceManager.initialise (2, 2, nullptr, true, String(), nullptr);
  |     }
  | );
  | 
  | @tags{Core}
  |
  */
pub struct RuntimePermissions {

}


//-------------------------------------------[.cpp/Aloe/modules/aloe_core/misc/aloe_RuntimePermissions.cpp]
impl RuntimePermissions {

    /**
      | We currently don't request runtime
      | permissions on any other platform than
      | 
      | Android, so this file contains a dummy
      | implementation for those. This may
      | change in the future.
      |
      ---------------------------------
      |
      | Call this method to request a runtime
      | permission.
      | 
      | -----------
      | @param permission
      | 
      | The RuntimePermissionsPermissionID of the permission
      | you want to request.
      | ----------
      | @param callback
      | 
      | The callback to be called after the request
      | has been granted or denied; the argument
      | passed will be true if the permission
      | has been granted and false otherwise.
      | 
      | If no runtime request is required or
      | possible to obtain the permission,
      | the callback will be called immediately.
      | The argument passed in will be true if
      | the permission is granted or no permission
      | is required on this platform, and false
      | otherwise.
      | 
      | If a runtime request is required to obtain
      | the permission, the callback will be
      | called asynchronously after the OS
      | has granted or denied the requested
      | permission (typically by displaying
      | a dialog box to the user and waiting until
      | the user has responded).
      |
      */
    #[cfg(not(target_os="android"))]
    pub fn request(&mut self, 
        _0:       RuntimePermissionsPermissionID,
        callback: RuntimePermissionsCallback)  {
        
        todo!();
        /*
            callback (true);
        */
    }
    
    /**
      | Returns whether a runtime request is
      | required to obtain the permission on
      | the current platform.
      |
      */
    #[cfg(not(target_os="android"))]
    pub fn is_required(&mut self, _0: RuntimePermissionsPermissionID) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    /**
      | Returns true if the app has been already
      | granted this permission, either via
      | a previous runtime request or otherwise,
      | or no permission is necessary.
      | 
      | -----------
      | @note
      | 
      | this can be false even if isRequired
      | returns false. In this case, the permission
      | can not be obtained at all at runtime.
      |
      */
    #[cfg(not(target_os="android"))]
    pub fn is_granted(&mut self, _0: RuntimePermissionsPermissionID) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
