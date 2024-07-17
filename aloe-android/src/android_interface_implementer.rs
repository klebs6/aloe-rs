crate::ix!();

//#[cfg(target_os="android")]
pub trait AndroidInterfaceImplementerInterface {

    fn invoke(&mut self, 
        proxy:  jobject,
        method: jobject,
        args:   jobjectArray) -> jobject;
}

///-------------------------
//#[cfg(target_os="android")]
pub struct AndroidInterfaceImplementer {
    java_sub_class:     GlobalRef,
    invocation_handler: GlobalRef,
}

//#[cfg(target_os="android")]
impl Drop for AndroidInterfaceImplementer {

    fn drop(&mut self) {
        todo!();
        /* 
        clear();
         */
    }
}

//#[cfg(target_os="android")]
impl AndroidInterfaceImplementer {

    pub fn clear(&mut self)  {
        
        todo!();
        /*
            if (invocationHandler != nullptr)
            getEnv()->CallVoidMethod (invocationHandler,
                                      AloeInvocationHandler.clear);
        */
    }
    
    pub fn invoke(&mut self, 
        proxy:  jobject,
        method: jobject,
        args:   jobjectArray) -> jobject {
        
        todo!();
        /*
            auto* env = getEnv();
        return env->CallObjectMethod (method, JavaMethod.invoke, javaSubClass.get(), args);
        */
    }
}
