crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/native/aloe_android_Messaging.cpp]
#[cfg(target_os="android")]
pub struct AndroidRunnable {
    base: AndroidInterfaceImplementer,
}

#[cfg(target_os="android")]
pub trait AndroidRunnableInterface {
    fn run(&mut self);
}

#[cfg(target_os="android")]
impl AndroidRunnable {

    pub fn invoke(&mut self, 
        proxy:  jobject,
        method: jobject,
        args:   jobjectArray) -> jobject {
        
        todo!();
        /*
            auto* env = getEnv();
                auto methodName = aloeString ((jstring) env->CallObjectMethod (method, JavaMethod.getName));

                if (methodName == "run")
                {
                    run();
                    return nullptr;
                }

                // invoke base class
                return AndroidInterfaceImplementer::invoke (proxy, method, args);
        */
    }
}


