crate::ix!();

//#[cfg(target_os="android")]
pub struct LocalRef<JavaType> {
    obj: JavaType,
}

//#[cfg(target_os="android")]
impl<JavaType> Drop for LocalRef<JavaType> {

    fn drop(&mut self) {
        todo!();
        /*      clear();  */
    }
}

//#[cfg(target_os="android")]
impl<JavaType> Default for LocalRef<JavaType> {

    fn default() -> Self {
    
        todo!();
        /*
        : obj(nullptr),

        
        */
    }
}

//#[cfg(target_os="android")]
impl<JavaType> LocalRef<JavaType> {
    
    pub fn new_from_java_type(o: JavaType) -> Self {
    
        todo!();
        /*
        : obj(o),

        
        */
    }
    
    pub fn new_from_ref(other: &LocalRef<JavaType>) -> Self {
    
        todo!();
        /*
        : obj(retain (other.obj)),

        
        */
    }
    
    pub fn new(other: LocalRef<JavaType>) -> Self {
    
        todo!();
        /*
        : obj(nullptr),

            std::swap (obj, other.obj);
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            if (obj != nullptr)
            {
                getEnv()->DeleteLocalRef (obj);
                obj = nullptr;
            }
        */
    }
    
    pub fn assign_from_ref(&mut self, other: &LocalRef<JavaType>) -> &mut LocalRef<JavaType> {
        
        todo!();
        /*
            JavaType newObj = retain (other.obj);
            clear();
            obj = newObj;
            return *this;
        */
    }
    
    pub fn assign_from(&mut self, other: LocalRef<JavaType>) -> &mut LocalRef<JavaType> {
        
        todo!();
        /*
            clear();
            std::swap (other.obj, obj);
            return *this;
        */
    }

    
    fn from(other: Self) -> Self {
        todo!();
        /*
            return obj;
        */
    }
    
    #[inline] pub fn get(&self) -> JavaType {
        
        todo!();
        /*
            return obj;
        */
    }
    
    pub fn retain(obj: JavaType) -> JavaType {
        
        todo!();
        /*
            return obj == nullptr ? nullptr : (JavaType) getEnv()->NewLocalRef (obj);
        */
    }
}
