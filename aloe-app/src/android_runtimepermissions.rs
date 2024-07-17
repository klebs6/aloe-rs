crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_android_RuntimePermissions.cpp]

#[cfg(target_os="android")]
pub fn aloe_permission_to_android_permission(permission: RuntimePermissionsPermissionID) -> String {
    
    todo!();
    /*
        switch (permission)
        {
            case RuntimePermissions::recordAudio:           return "android.permission.RECORD_AUDIO";
            case RuntimePermissions::bluetoothMidi:         return "android.permission.ACCESS_FINE_LOCATION";
            case RuntimePermissions::readExternalStorage:   return "android.permission.READ_EXTERNAL_STORAGE";
            case RuntimePermissions::writeExternalStorage:  return "android.permission.WRITE_EXTERNAL_STORAGE";
            case RuntimePermissions::camera:                return "android.permission.CAMERA";
        }

        // invalid permission
        jassertfalse;
        return {};
    */
}

#[cfg(target_os="android")]
pub fn android_permission_to_aloe_permission(permission: &String) -> RuntimePermissionsPermissionID {
    
    todo!();
    /*
        if      (permission == "android.permission.RECORD_AUDIO")            return RuntimePermissions::recordAudio;
        else if (permission == "android.permission.ACCESS_FINE_LOCATION")    return RuntimePermissions::bluetoothMidi;
        else if (permission == "android.permission.READ_EXTERNAL_STORAGE")   return RuntimePermissions::readExternalStorage;
        else if (permission == "android.permission.WRITE_EXTERNAL_STORAGE")  return RuntimePermissions::writeExternalStorage;
        else if (permission == "android.permission.CAMERA")                  return RuntimePermissions::camera;

        return static_cast<RuntimePermissions::RuntimePermissionsPermissionID> (-1);
    */
}

impl RuntimePermissions {
    
    #[cfg(target_os="android")]
    pub fn request(
        &mut self, 
        permission: RuntimePermissionsPermissionID,
        callback:   RuntimePermisisonCallback

    ) {
        
        todo!();
        /*
            auto requestedPermission = aloePermissionToAndroidPermission (permission);

        if (! isPermissionDeclaredInManifest (requestedPermission))
        {
            // Error! If you want to be able to request this runtime permission, you
            // also need to declare it in your app's manifest. You can do so via
            // the Proaloer. Otherwise this can't work.
            jassertfalse;

            callback (false);
            return;
        }

        auto alreadyGranted = isGranted (permission);

        if (alreadyGranted || getAndroidSDKVersion() < 23)
        {
            callback (alreadyGranted);
            return;
        }

        PermissionsRequest request (std::move (callback), permission);

        static CriticalSection overlayGuard;
        ScopedLock lock (overlayGuard);

        std::unique_ptr<PermissionsOverlay>& overlay = PermissionsOverlay::getSingleton();

        bool alreadyOpen = true;

        if (overlay == nullptr)
        {
            overlay.reset (new PermissionsOverlay (overlayGuard));
            alreadyOpen = false;
        }

        overlay->requests.push_back (std::move (request));

        if (! alreadyOpen)
            overlay->open();
        */
    }
    
    #[cfg(target_os="android")]
    pub fn is_required(&mut self, permission: RuntimePermissionsPermissionID) -> bool {
        
        todo!();
        /*
            return getAndroidSDKVersion() >= 23;
        */
    }
    
    #[cfg(target_os="android")]
    pub fn is_granted(&mut self, permission: RuntimePermissionsPermissionID) -> bool {
        
        todo!();
        /*
            auto* env = getEnv();

        auto requestedPermission = aloePermissionToAndroidPermission (permission);
        int result = env->CallIntMethod (getAppContext().get(), AndroidContext.checkCallingOrSelfPermission,
                                         javaString (requestedPermission).get());

        return result == 0 /* PERMISSION_GRANTED */;
        */
    }
}
