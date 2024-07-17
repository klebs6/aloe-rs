crate::ix!();

//#[cfg(target_os="android")]
pub struct ActivityLifecycleCallbacks {
    base: AndroidInterfaceImplementer,
}

//#[cfg(target_os="android")]
impl ActivityLifecycleCallbacks {
    
    pub fn invoke(&mut self, 
        proxy:  jobject,
        method: jobject,
        args:   jobjectArray) -> jobject {
        
        todo!();
        /*
            auto* env = getEnv();

        auto methodName = aloeString ((jstring) env->CallObjectMethod (method, JavaMethod.getName));

        auto activity = env->GetArrayLength (args) > 0 ? env->GetObjectArrayElement (args, 0) : (jobject) nullptr;
        auto bundle   = env->GetArrayLength (args) > 1 ? env->GetObjectArrayElement (args, 1) : (jobject) nullptr;

        if      (methodName == "onActivityPreCreated")             { onActivityPreCreated (activity, bundle);            return nullptr; }
        else if (methodName == "onActivityPreDestroyed")           { onActivityPreDestroyed (activity);                  return nullptr; }
        else if (methodName == "onActivityPrePaused")              { onActivityPrePaused (activity);                     return nullptr; }
        else if (methodName == "onActivityPreResumed")             { onActivityPreResumed (activity);                    return nullptr; }
        else if (methodName == "onActivityPreSaveInstanceState")   { onActivityPreSaveInstanceState (activity, bundle);  return nullptr; }
        else if (methodName == "onActivityPreStarted")             { onActivityPreStarted (activity);                    return nullptr; }
        else if (methodName == "onActivityPreStopped")             { onActivityPreStopped (activity);                    return nullptr; }
        else if (methodName == "onActivityCreated")                { onActivityCreated (activity, bundle);               return nullptr; }
        else if (methodName == "onActivityDestroyed")              { onActivityDestroyed (activity);                     return nullptr; }
        else if (methodName == "onActivityPaused")                 { onActivityPaused (activity);                        return nullptr; }
        else if (methodName == "onActivityResumed")                { onActivityResumed (activity);                       return nullptr; }
        else if (methodName == "onActivitySaveInstanceState")      { onActivitySaveInstanceState (activity, bundle);     return nullptr; }
        else if (methodName == "onActivityStarted")                { onActivityStarted (activity);                       return nullptr; }
        else if (methodName == "onActivityStopped")                { onActivityStopped (activity);                       return nullptr; }
        else if (methodName == "onActivityPostCreated")            { onActivityPostCreated (activity, bundle);           return nullptr; }
        else if (methodName == "onActivityPostDestroyed")          { onActivityPostDestroyed (activity);                 return nullptr; }
        else if (methodName == "onActivityPostPaused")             { onActivityPostPaused (activity);                    return nullptr; }
        else if (methodName == "onActivityPostResumed")            { onActivityPostResumed (activity);                   return nullptr; }
        else if (methodName == "onActivityPostSaveInstanceState")  { onActivityPostSaveInstanceState (activity, bundle); return nullptr; }
        else if (methodName == "onActivityPostStarted")            { onActivityPostStarted (activity);                   return nullptr; }
        else if (methodName == "onActivityPostStopped")            { onActivityPostStopped (activity);                   return nullptr; }

        return AndroidInterfaceImplementer::invoke (proxy, method, args);
        */
    }
}

//#[cfg(target_os="android")]
impl AndroidInterfaceImplementerInterface for ActivityLifecycleCallbacks {

    fn invoke(&mut self, 
        _0: jobject,
        _1: jobject,
        _2: jobjectArray) -> jobject {
        
        todo!();
        /*
        
        */
    }
}

//#[cfg(target_os="android")]
pub trait ActivityLifecycleCallbacksInterface {

    fn on_activity_pre_created(&mut self, 
        activity: jobject,
        bundle:   jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_pre_destroyed(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_pre_paused(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_pre_resumed(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_pre_save_instance_state(&mut self, 
        activity: jobject,
        bundle:   jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_pre_started(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_pre_stopped(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_created(&mut self, 
        activity: jobject,
        bundle:   jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_destroyed(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_paused(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_resumed(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_save_instance_state(&mut self, 
        activity: jobject,
        bundle:   jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_started(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_stopped(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_post_created(&mut self, 
        activity: jobject,
        bundle:   jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_post_destroyed(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_post_paused(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_post_resumed(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_post_save_instance_state(&mut self, 
        activity: jobject,
        bundle:   jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_post_started(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }

    fn on_activity_post_stopped(&mut self, activity: jobject)  {
        
        todo!();
        /*
        
        */
    }
}
