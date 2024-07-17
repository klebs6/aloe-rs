crate::ix!();

#[cfg(target_os="android")]
pub fn get_multicast_lock() -> LocalRef<jobject> {
    
    todo!();
    /*
        static LocalRef<jobject> multicastLock;
        static bool hasChecked = false;

        if (! hasChecked)
        {
            hasChecked = true;

            auto* env = getEnv();

            LocalRef<jobject> wifiManager (env->CallObjectMethod (getAppContext().get(),
                                                                  AndroidContext.getSystemService,
                                                                  javaString ("wifi").get()));

            if (wifiManager != nullptr)
            {
                multicastLock = LocalRef<jobject> (env->CallObjectMethod (wifiManager.get(),
                                                                          AndroidWifiManager.createMulticastLock,
                                                                          javaString ("ALOE_MulticastLock").get()));
            }
        }

        return multicastLock;
    */
}

pub fn acquire_multicast_lock()  {
    
    todo!();
    /*
        auto multicastLock = getMulticastLock();

        if (multicastLock != nullptr)
            getEnv()->CallVoidMethod (multicastLock.get(), AndroidMulticastLock.acquire);
    */
}

pub fn release_multicast_lock()  {
    
    todo!();
    /*
        auto multicastLock = getMulticastLock();

        if (multicastLock != nullptr)
            getEnv()->CallVoidMethod (multicastLock.get(), AndroidMulticastLock.release);
    */
}
