crate::ix!();

///----------------------------
#[cfg(target_os="android")]
pub struct AloeActivityWatcher {
    base:                  ActivityLifecycleCallbacks,
    myself:                GlobalRef,
    current_activity_lock: CriticalSection,
    current_activity:      jweak, // default = nullptr
    main_activity:         jweak, // default = nullptr
}

#[cfg(target_os="android")]
impl Default for AloeActivityWatcher {
    
    fn default() -> Self {
        todo!();
        /*


            LocalRef<jobject> appContext (getAppContext());

            if (appContext != nullptr)
            {
                auto* env = getEnv();

                myself = GlobalRef (CreateJavaInterface (this, "android/app/Application$ActivityLifecycleCallbacks"));
                env->CallVoidMethod (appContext.get(), AndroidApplication.registerActivityLifecycleCallbacks, myself.get());
            }

            checkActivityIsMain (androidApkContext)
        */
    }
}

#[cfg(target_os="android")]
impl Drop for AloeActivityWatcher {
    fn drop(&mut self) {
        todo!();
        /* 
            LocalRef<jobject> appContext (getAppContext());

            if (appContext != nullptr && myself != nullptr)
            {
                auto* env = getEnv();

                env->CallVoidMethod (appContext.get(), AndroidApplication.unregisterActivityLifecycleCallbacks, myself.get());
                clear();
                myself.clear();
            }
         */
    }
}

#[cfg(target_os="android")]
impl AloeActivityWatcher {

    pub fn on_activity_started(&mut self, activity: jobject)  {
        
        todo!();
        /*
            auto* env = getEnv();

            checkActivityIsMain (activity);

            ScopedLock lock (currentActivityLock);

            if (currentActivity != nullptr)
            {
                // see Clarification June 2001 in JNI reference for why this is
                // necessary
                LocalRef<jobject> localStorage (env->NewLocalRef (currentActivity));

                if (env->IsSameObject (localStorage.get(), activity) != 0)
                    return;

                env->DeleteWeakGlobalRef (currentActivity);
                currentActivity = nullptr;
            }

            if (activity != nullptr)
                currentActivity = env->NewWeakGlobalRef (activity);
        */
    }
    
    pub fn on_activity_stopped(&mut self, activity: jobject)  {
        
        todo!();
        /*
            auto* env = getEnv();

            ScopedLock lock (currentActivityLock);

            if (currentActivity != nullptr)
            {
                // important that the comparison happens in this order
                // to avoid race condition where the weak reference becomes null
                // just after the first check
                if (env->IsSameObject (currentActivity, activity) != 0
                    || env->IsSameObject (currentActivity, nullptr) != 0)
                {
                    env->DeleteWeakGlobalRef (currentActivity);
                    currentActivity = nullptr;
                }
            }
        */
    }
    
    pub fn get_current(&mut self) -> LocalRef<jobject> {
        
        todo!();
        /*
            ScopedLock lock (currentActivityLock);
            return LocalRef<jobject> (getEnv()->NewLocalRef (currentActivity));
        */
    }
    
    pub fn get_main(&mut self) -> LocalRef<jobject> {
        
        todo!();
        /*
            ScopedLock lock (currentActivityLock);
            return LocalRef<jobject> (getEnv()->NewLocalRef (mainActivity));
        */
    }
    
    pub fn get_instance<'a>() -> &'a mut AloeActivityWatcher {
        
        todo!();
        /*
            static AloeActivityWatcher activityWatcher;
            return activityWatcher;
        */
    }
    
    pub fn check_activity_is_main(&mut self, context: jobject)  {
        
        todo!();
        /*
            auto* env = getEnv();

            ScopedLock lock (currentActivityLock);

            if (mainActivity != nullptr)
            {
                if (env->IsSameObject (mainActivity, nullptr) != 0)
                {
                    env->DeleteWeakGlobalRef (mainActivity);
                    mainActivity = nullptr;
                }
            }

            if (mainActivity == nullptr)
            {
                LocalRef<jobject> appContext (getAppContext());
                auto mainActivityPath = getMainActivityClassPath();

                if (mainActivityPath.isNotEmpty())
                {
                    auto clasz = env->GetObjectClass (context);
                    auto activityPath = aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (clasz, JavaClass.getName)));

                    // This may be problematic for apps which use several activities with the same type. We just
                    // assume that the very first activity of this type is the main one
                    if (activityPath == mainActivityPath)
                        mainActivity = env->NewWeakGlobalRef (context);
                }
            }
        */
    }
    
    pub fn get_main_activity_class_path() -> String {
        
        todo!();
        /*
            static String mainActivityClassPath;

            if (mainActivityClassPath.isEmpty())
            {
                LocalRef<jobject> appContext (getAppContext());

                if (appContext != nullptr)
                {
                    auto* env = getEnv();

                    LocalRef<jobject> pkgManager (env->CallObjectMethod (appContext.get(), AndroidContext.getPackageManager));
                    LocalRef<jstring> pkgName ((jstring) env->CallObjectMethod (appContext.get(), AndroidContext.getPackageName));

                    LocalRef<jobject> intent (env->NewObject (AndroidIntent, AndroidIntent.constructWithString,
                                                              javaString ("android.intent.action.MAIN").get()));

                    intent = LocalRef<jobject> (env->CallObjectMethod (intent.get(),
                                                                       AndroidIntent.setPackage,
                                                                       pkgName.get()));

                    LocalRef<jobject> resolveInfo (env->CallObjectMethod (pkgManager.get(), AndroidPackageManager.resolveActivity, intent.get(), 0));

                    if (resolveInfo != nullptr)
                    {
                        LocalRef<jobject> activityInfo (env->GetObjectField (resolveInfo.get(), AndroidResolveInfo.activityInfo));
                        LocalRef<jstring> jName ((jstring) env->GetObjectField (activityInfo.get(), AndroidPackageItemInfo.name));
                        LocalRef<jstring> jPackage ((jstring) env->GetObjectField (activityInfo.get(), AndroidPackageItemInfo.packageName));

                        mainActivityClassPath = aloeString (jName);
                    }
                }
            }

            return mainActivityClassPath;
        */
    }
}

