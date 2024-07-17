crate::ix!();

#[cfg(target_os="android")]
impl Thread {
    
    pub fn initialisealoe(&mut self, 
        jni_env: *mut c_void,
        context: *mut c_void)  {
        
        todo!();
        /*
            static CriticalSection cs;
        ScopedLock lock (cs);

        // jniEnv and context should not be null!
        jassert (jniEnv != nullptr && context != nullptr);

        auto* env = static_cast<JNIEnv*> (jniEnv);

        if (androidJNIJavaVM == nullptr)
        {
            JavaVM* javaVM = nullptr;

            auto status = env->GetJavaVM (&javaVM);
            jassert (status == 0 && javaVM != nullptr);

            androidJNIJavaVM = javaVM;
        }

        static bool firstCall = true;

        if (firstCall)
        {
            firstCall = false;

            // if we ever support unloading then this should probably be a weak reference
            androidApkContext = env->NewGlobalRef (static_cast<jobject> (context));
            AloeActivityWatcher::getInstance();

           #if ALOE_MODULE_AVAILABLE_aloe_events && ALOE_ANDROID
            aloe_aloeEventsAndroidStartApp();
           #endif
        }
        */
    }
}
