crate::ix!();

#[cfg(target_os="android")]
pub struct PermissionsOverlayPermissionResult {
    request: PermissionsRequest,
    granted: bool,
}

#[cfg(target_os="android")]
pub struct PermissionsOverlay<'a> {
    base:          FragmentOverlay,
    overlay_guard: &'a mut CriticalSection,
    requests:      Vec<PermissionsRequest>,
}

#[cfg(target_os="android")]
impl<'a> PermissionsOverlay<'a> {

    pub fn new(cs: &mut CriticalSection) -> Self {
    
        todo!();
        /*
        : overlay_guard(cs),

        
        */
    }
    
    pub fn on_start(&mut self)  {
        
        todo!();
        /*
            onRequestPermissionsResult (0, {}, {});
        */
    }
    
    pub fn on_request_permissions_result(&mut self, 
        request_code:  i32,
        permissions:   &StringArray,
        grant_results: &[i32])  {
        
        todo!();
        /*
            std::vector<PermissionsOverlayPermissionResult> results;

            {
                ScopedLock lock (overlayGuard);

                for (auto it = requests.begin(); it != requests.end();)
                {
                    auto& request = *it;

                    if (RuntimePermissions::isGranted (request.permission))
                    {
                        results.push_back ({std::move (request), true});
                        it = requests.erase (it);
                    }
                    else
                    {
                        ++it;
                    }
                }

                auto n = permissions.size();

                for (int i = 0; i < n; ++i)
                {
                    auto permission = androidPermissionToAloePermission (permissions[i]);
                    auto granted = (grantResults.getReference (i) == 0);

                    for (auto it = requests.begin(); it != requests.end();)
                    {
                        auto& request = *it;

                        if (request.permission == permission)
                        {
                            results.push_back ({std::move (request), granted});
                            it = requests.erase (it);
                        }
                        else
                        {
                            ++it;
                        }
                    }
                }
            }

            for (const auto& result : results)
                if (result.request.callback)
                    result.request.callback (result.granted);

            {
                auto* env = getEnv();
                ScopedLock lock (overlayGuard);

                if (requests.size() > 0)
                {
                    auto &request = requests.front();

                    StringArray permissionsArray{
                            aloePermissionToAndroidPermission (request.permission)};
                    auto jPermissionsArray = aloeStringArrayToJava (permissionsArray);

                    auto requestPermissionsMethodID
                        = env->GetMethodID(AndroidFragment, "requestPermissions", "([Ljava/lang/String;I)V");

                    // this code should only be reached for SDKs >= 23, so this method should be
                    // be available
                    jassert(requestPermissionsMethodID != nullptr);

                    env->CallVoidMethod (getNativeHandle(), requestPermissionsMethodID, jPermissionsArray.get (), 0);
                }
                else
                {
                    getSingleton() = nullptr;
                }
            }
        */
    }
    
    pub fn get_singleton() -> &'a mut Box<PermissionsOverlay<'a>> {
        
        todo!();
        /*
            static std::unique_ptr<PermissionsOverlay> instance;
            return instance;
        */
    }
}
