crate::ix!();

//#[cfg(target_os="android")]
pub struct GlobalRef {
    obj: jobject, // default = nullptr
}

//#[cfg(target_os="android")]
impl Drop for GlobalRef {
    fn drop(&mut self) {
        todo!();
        /*      clear();  */
    }
}

//#[cfg(target_os="android")]
impl Into<jobject> for GlobalRef {
    
    #[inline] fn into(self) -> jobject {
        todo!();
        /*
            return obj;
        */
    }
}

//#[cfg(target_os="android")]
macro_rules! declare_call_type_method {
    ($returnType:ident, $typeName:ident) => {
        /*
        
                returnType call##typeName##Method (jmethodID methodID, ... ) const 
                { 
                    va_list args; 
                    va_start (args, methodID); 
                    returnType result = getEnv()->Call##typeName##MethodV (obj, methodID, args); 
                    va_end (args); 
                    return result; 
                }
        */
    }
}

//#[cfg(target_os="android")]
impl GlobalRef {
    
    pub fn new() -> Self {
    
        todo!();
        /*
        : obj(nullptr),

        
        */
    }
    
    pub fn new_from_local_ref(o: &LocalRef<jobject>) -> Self {
    
        todo!();
        /*
        : obj(retain (o.get(), getEnv())),

        
        */
    }
    
    pub fn new_from_local_ref_with_env(
        o:   &LocalRef<jobject>,
        env: *mut JNIEnv) -> Self {
    
        todo!();
        /*
        : obj(retain (o.get(), env)),

        
        */
    }
    
    pub fn new_from_global_ref_ref(other: &GlobalRef) -> Self {
    
        todo!();
        /*
        : obj(retain (other.obj, getEnv())),

        
        */
    }
    
    pub fn new_from_global_ref(other: GlobalRef) -> Self {
    
        todo!();
        /*
        : obj(nullptr),

            std::swap (other.obj, obj);
        */
    }
    
    #[inline] pub fn clear(&mut self)  {
        
        todo!();
        /*
            if (obj != nullptr) clear (getEnv());
        */
    }
    
    #[inline] pub fn clear_with_env(&mut self, env: *mut JNIEnv)  {
        
        todo!();
        /*
            if (obj != nullptr)
            {
                env->DeleteGlobalRef (obj);
                obj = nullptr;
            }
        */
    }
    
    #[inline] pub fn assign_from_ref(&mut self, other: &GlobalRef) -> &mut GlobalRef {
        
        todo!();
        /*
            jobject newObj = retain (other.obj, getEnv());
            clear();
            obj = newObj;
            return *this;
        */
    }
    
    #[inline] pub fn assign_from(&mut self, other: GlobalRef) -> &mut GlobalRef {
        
        todo!();
        /*
            clear();
            std::swap (obj, other.obj);

            return *this;
        */
    }
    
    #[inline] pub fn get(&self) -> jobject {
        
        todo!();
        /*
            return obj;
        */
    }

    declare_call_type_method!{ jobject, Object }
    declare_call_type_method!{ jboolean, Boolean }
    declare_call_type_method!{ jbyte, Byte }
    declare_call_type_method!{ jchar, Char }
    declare_call_type_method!{ jshort, Short }
    declare_call_type_method!{ jint, Int }
    declare_call_type_method!{ jlong, Long }
    declare_call_type_method!{ jfloat, Float }
    declare_call_type_method!{ jdouble, Double }
    
    pub fn call_void_method(&self, 
        methodid: jmethodID,
        args:     &[&str])  {
        
        todo!();
        /*
            va_list args;
            va_start (args, methodID);
            getEnv()->CallVoidMethodV (obj, methodID, args);
            va_end (args);
        */
    }
    
    pub fn retain(
        obj: jobject,
        env: *mut JNIEnv) -> jobject {
        
        todo!();
        /*
            return obj == nullptr ? nullptr : env->NewGlobalRef (obj);
        */
    }
}

