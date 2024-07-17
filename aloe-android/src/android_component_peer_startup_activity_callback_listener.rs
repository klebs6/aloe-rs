crate::ix!();

#[cfg(target_os="android")]
pub struct AndroidComponentPeerStartupActivityCallbackListener {
    base: ActivityLifecycleCallbacks,
}

#[cfg(target_os="android")]
impl AndroidComponentPeerStartupActivityCallbackListener {

    pub fn on_activity_started(&mut self, activity: jobject)  {
        
        todo!();
        /*
            auto* env = getEnv();
                LocalRef<jobject> appContext (getAppContext());

                if (appContext.get() != nullptr)
                {
                    env->CallVoidMethod (appContext.get(),
                                         AndroidApplication.unregisterActivityLifecycleCallbacks,
                                         activityCallbackListener.get());
                    clear();
                    activityCallbackListener.clear();

                    forceDisplayUpdate();
                }
        */
    }
}
