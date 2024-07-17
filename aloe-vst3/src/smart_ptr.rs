crate::ix!();

pub struct VstComSmartPtr<ObjectType> {
    source: *mut ObjectType,
}

impl<ObjectType> Default for VstComSmartPtr<ObjectType> {
    
    fn default() -> Self {
        todo!();
        /*
        : source(nullptr),

        
        */
    }
}

impl<ObjectType> Drop for VstComSmartPtr<ObjectType> {

    fn drop(&mut self) {
        todo!();
        /*
            if (source != nullptr) source->release();
        */
    }
}

impl<ObjectType> VstComSmartPtr<ObjectType> {
    
    #[inline] pub fn into_source(self) -> ObjectType {
        todo!();
        /*
            return source;
        */
    }
}

impl<ObjectType> Deref for VstComSmartPtr<ObjectType> {

    type Target = ObjectType;
    
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return source;
        */
    }
}

impl<ObjectType> PartialEq<ObjectType> for VstComSmartPtr<ObjectType> {
    
    #[inline] fn eq(&self, other: &ObjectType) -> bool {
        todo!();
        /*
            return source == other;
        */
    }
}

impl<ObjectType> From<&VstComSmartPtr<ObjectType>> for VstComSmartPtr<ObjectType> {

    fn from(other: &VstComSmartPtr<ObjectType>) -> Self {
    
        todo!();
        /*
        : source(other.source),

            if (source != nullptr) source->addRef();
        */
    }
}

impl<ObjectType> VstComSmartPtr<ObjectType> {

    pub fn new(
        object:       *mut ObjectType,
        auto_add_ref: Option<bool>

    ) -> Self {

        let auto_add_ref: bool = auto_add_ref.unwrap_or(true);

        todo!();
        /*
        : source(object),

            if (source != nullptr && autoAddRef) source->addRef();
        */
    }
    
    pub fn get(&self) -> *mut ObjectType {
        
        todo!();
        /*
            return source;
        */
    }
    
    pub fn assign_from(&mut self, other: &VstComSmartPtr<ObjectType>) -> &mut VstComSmartPtr<ObjectType> {
        
        todo!();
        /*
            return operator= (other.source);
        */
    }
    
    pub fn assign_from_raw(&mut self, new_object_to_take_possession_of: *mut ObjectType) 
        -> &mut VstComSmartPtr<ObjectType> 
    {
        todo!();
        /*
            VstComSmartPtr p (newObjectToTakePossessionOf);
            std::swap (p.source, source);
            return *this;
        */
    }
    
    pub fn load_from_funknown(&mut self, o: *mut dyn FUnknown) -> bool {
        
        todo!();
        /*
            *this = nullptr;
            return o != nullptr && o->queryInterface (ObjectType::iid, (void**) &source) == typename kResultOk;
        */
    }

    pub fn load_from(
        &mut self, 
        factory: *mut dyn IPluginFactory,
        uuid:    &TUID

    ) -> bool {
        
        todo!();
        /*
            jassert (factory != nullptr);
            *this = nullptr;
            return factory->createInstance (uuid, ObjectType::iid, (void**) &source) == typename kResultOk;
        */
    }
}
