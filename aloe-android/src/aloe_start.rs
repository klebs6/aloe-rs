crate::ix!();

#[cfg(all(ALOE_MODULE_AVAILABLE_aloe_events,target_os="android"))]
pub fn aloe_aloe_events_android_start_app()  {
    
    todo!();
    /*
    
    */
}


pub fn aloe_get_executable_file() -> File {
    
    todo!();
    /*
    
    */
}

#[cfg(feature = "aloe_posix")]
pub fn aloe_get_executable_file() -> File {
    
    todo!();
    /*
        struct DLAddrReader
        {
            static String getFilename()
            {
                Dl_info exeInfo;

                auto localSymbol = (void*) aloe_getExecutableFile;
                dladdr (localSymbol, &exeInfo);
                return CharPointer_UTF8 (exeInfo.dli_fname);
            }
        };

        static String filename = DLAddrReader::getFilename();
        return File::getCurrentWorkingDirectory().getChildFile (filename);
    */
}

pub fn aloe_aloe_events_android_start_app()  {
    
    todo!();
    /*
        auto dllPath = aloe_getExecutableFile().getFullPathName();
        auto addr = reinterpret_cast<ALOEApplicationBase*(*)()> (DynamicLibrary (dllPath)
                                                                        .getFunction ("aloe_CreateApplication"));

        if (addr != nullptr)
            AloeAppLifecycle::getInstance (addr);
    */
}

#[cfg(target_os="android")]
pub fn get_app_context() -> LocalRef<jobject> {
    
    todo!();
    /*
        auto* env = getEnv();
        auto context = androidApkContext;

        // You did not call Thread::initialiseALOE which must be called at least once in your apk
        // before using any Aloe APIs. The Proaloer will automatically generate java code
        // which will invoke Thread::initialiseALOE for you.
        jassert (env != nullptr && context != nullptr);

        if (context == nullptr)
            return LocalRef<jobject>();

        if (env->IsInstanceOf (context, AndroidApplication) != 0)
            return LocalRef<jobject> (env->NewLocalRef (context));

        LocalRef<jobject> applicationContext (env->CallObjectMethod (context, AndroidContext.getApplicationContext));

        if (applicationContext == nullptr)
            return LocalRef<jobject> (env->NewLocalRef (context));

        return applicationContext;
    */
}
