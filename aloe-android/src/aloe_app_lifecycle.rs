crate::ix!();

#[cfg(target_os="android")]
pub struct AloeAppLifecycle {
    base:                    ActivityLifecycleCallbacks,
    myself:                  GlobalRef,
    createApplicationSymbol: fn() -> *mut ALOEApplicationBase,
    has_been_initialised:    bool, // default = false
}

#[cfg(target_os="android")]
impl Drop for AloeAppLifecycle {

    fn drop(&mut self) {
        todo!();
        /* 
            LocalRef<jobject> appContext (getAppContext());

            if (appContext != nullptr && myself != nullptr)
            {
                auto* env = getEnv();

                clear();
                env->CallVoidMethod (appContext.get(), AndroidApplication.unregisterActivityLifecycleCallbacks, myself.get());
                myself.clear();
            }
         */
    }
}

#[cfg(target_os="android")]
impl AloeAppLifecycle {

    pub fn new(init_symbol_addr: fn() -> *mut ALOEApplicationBase) -> Self {
    
        todo!();
        /*


            : createApplicationSymbol (initSymbolAddr)

            LocalRef<jobject> appContext (getAppContext());

            if (appContext != nullptr)
            {
                auto* env = getEnv();

                myself = GlobalRef (CreateJavaInterface (this, "android/app/Application$ActivityLifecycleCallbacks"));
                env->CallVoidMethod (appContext.get(), AndroidApplication.registerActivityLifecycleCallbacks, myself.get());
            }
        */
    }
    
    pub fn on_activity_created(&mut self, 
        _0: jobject,
        _1: jobject)  {
        
        todo!();
        /*
            checkCreated();
        */
    }
    
    pub fn on_activity_destroyed(&mut self, activity: jobject)  {
        
        todo!();
        /*
            auto* env = getEnv();

            // if the main activity is being destroyed, only then tear-down Aloe
            if (env->IsSameObject (getMainActivity().get(), activity) != 0)
            {
                ALOEApplicationBase::appWillTerminateByForce();
                JNIClassBase::releaseAllClasses (env);

                jclass systemClass = (jclass) env->FindClass ("java/lang/System");
                jmethodID exitMethod = env->GetStaticMethodID (systemClass, "exit", "(I)V");
                env->CallStaticVoidMethod (systemClass, exitMethod, 0);
            }
        */
    }
    
    pub fn on_activity_started(&mut self, _0: jobject)  {
        
        todo!();
        /*
            checkCreated();
        */
    }
    
    pub fn on_activity_paused(&mut self, _0: jobject)  {
        
        todo!();
        /*
            if (auto* app = ALOEApplicationBase::getInstance())
                app->suspended();
        */
    }
    
    pub fn on_activity_resumed(&mut self, _0: jobject)  {
        
        todo!();
        /*
            checkInitialised();

            if (auto* app = ALOEApplicationBase::getInstance())
                app->resumed();
        */
    }
    
    pub fn get_instance<'a>(init_symbol_addr: fn() -> *mut ALOEApplicationBase) 
        -> &'a mut AloeAppLifecycle 
    {
        todo!();

        /*
            static AloeAppLifecycle aloeAppLifecycle (initSymbolAddr);
            return aloeAppLifecycle;
        */
    }
    
    pub fn check_created(&mut self)  {
        
        todo!();
        /*
            if (ALOEApplicationBase::getInstance() == nullptr)
            {
                DBG (SystemStats::getALOEVersion());

                ALOEApplicationBase::createInstance = createApplicationSymbol;

                initialiseAloe_GUI();

                if (! ALOEApplicationBase::createInstance())
                    jassertfalse; // you must supply an application object for an android app!

                jassert (MessageManager::getInstance()->isThisTheMessageThread());
            }
        */
    }
    
    pub fn check_initialised(&mut self)  {
        
        todo!();
        /*
            checkCreated();

            if (! hasBeenInitialised)
            {
                if (auto* app = ALOEApplicationBase::getInstance())
                {
                    hasBeenInitialised = app->initialiseApp();

                    if (! hasBeenInitialised)
                        exit (app->shutdownApp());
                }
            }
        */
    }
}

