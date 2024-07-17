crate::ix!();

#[cfg(target_os="android")]
pub struct FragmentOverlay {
    native: GlobalRef,
}

#[cfg(target_os="android")]
impl Drop for FragmentOverlay {

    fn drop(&mut self) {
        todo!();
        /* 
        auto* env = getEnv();

        env->CallVoidMethod (native.get(), AloeFragmentOverlay.close);
         */
    }
}

#[cfg(target_os="android")]
pub trait FragmentOverlayInterface {

    fn on_created(&mut self, bundle: LocalRef<jobject>)  {
        
        todo!();
        /*
        
        */
    }

    fn on_start(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    fn on_request_permissions_result(&mut self, 
        request_code:  i32,
        permissions:   &Vec<String>,
        grant_results: &[i32])  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_result(&mut self, 
        request_code: i32,
        result_code:  i32,
        data:         LocalRef<jobject>)  {
        
        todo!();
        /*
        
        */
    }
}

#[cfg(target_os="android")]
impl FragmentOverlay {
    
    pub fn new() -> Self {
    
        todo!();
        /*


            : native (LocalRef<jobject> (getEnv()->NewObject (AloeFragmentOverlay, AloeFragmentOverlay.construct)))
        */
    }
    
    pub fn open(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();

        LocalRef<jobject> bundle (env->NewObject (AndroidBundle, AndroidBundle.constructor));
        env->CallVoidMethod (bundle.get(), AndroidBundle.putLong, javaString ("cppThis").get(), (jlong) this);
        env->CallVoidMethod (native.get(), AndroidFragment.setArguments, bundle.get());

        LocalRef<jobject> fm (env->CallObjectMethod (getCurrentActivity().get(), AndroidActivity.getFragmentManager));
        env->CallVoidMethod (native.get(), AndroidDialogFragment.show, fm.get(), javaString ("FragmentOverlay").get());
        */
    }
    
    pub fn on_activity_result_native(&mut self, 
        env:          *mut JNIEnv,
        _1:           jobject,
        host:         i64,
        request_code: i32,
        result_code:  i32,
        data:         jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<FragmentOverlay*> (host))
            myself->onActivityResult (requestCode, resultCode, LocalRef<jobject> (env->NewLocalRef (data)));
        */
    }
    
    pub fn on_create_native(&mut self, 
        env:    *mut JNIEnv,
        _1:     jobject,
        host:   i64,
        bundle: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<FragmentOverlay*> (host))
            myself->onCreated (LocalRef<jobject> (env->NewLocalRef (bundle)));
        */
    }
    
    pub fn on_start_native(&mut self, 
        _0:   *mut JNIEnv,
        _1:   jobject,
        host: i64)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<FragmentOverlay*> (host))
            myself->onStart();
        */
    }
    
    pub fn on_request_permissions_result_native(&mut self, 
        env:             *mut JNIEnv,
        _1:              jobject,
        host:            i64,
        request_code:    i32,
        j_permissions:   jobjectArray,
        j_grant_results: jintArray)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<FragmentOverlay*> (host))
        {
            Vec<int> grantResults;
            int n = (jGrantResults != nullptr ? env->GetArrayLength (jGrantResults) : 0);

            if (n > 0)
            {
                auto* data = env->GetIntArrayElements (jGrantResults, nullptr);

                for (int i = 0; i < n; ++i)
                    grantResults.add (data[i]);

                env->ReleaseIntArrayElements (jGrantResults, data, 0);
            }

            myself->onRequestPermissionsResult (requestCode,
                                                javaStringArrayToAloe (LocalRef<jobjectArray> (jPermissions)),
                                                grantResults);
        }
        */
    }
    
    pub fn get_native_handle(&mut self) -> jobject {
        
        todo!();
        /*
            return native.get();
        */
    }
}
