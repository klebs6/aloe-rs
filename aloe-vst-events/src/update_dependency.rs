crate::ix!();

#[cfg(CLASS_NAME_TRACKED)]
pub struct UpdateHandlerDependency {
    obj:       *mut FUnknown,
    dep:       *mut IDependent,
    obj_class: FClassID,
    dep_class: FClassID,
}

#[cfg(CLASS_NAME_TRACKED)]
impl UpdateHandlerDependency {

    pub fn new(
        o: *mut FUnknown,
        d: *mut IDependent) -> Self {
    
        todo!();
        /*
        : obj(o),
        : dep(d),
        : obj_class(nullptr),
        : dep_class(nullptr),

        
        */
    }
}

#[cfg(CLASS_NAME_TRACKED)]
impl PartialEq<UpdateHandlerDependency> for UpdateHandlerDependency {
    
    fn eq(&self, other: &UpdateHandlerDependency) -> bool {
        todo!();
        /*
            return obj == d.obj;
        */
    }
}

#[cfg(CLASS_NAME_TRACKED)]
impl Eq for UpdateHandlerDependency {}

#[cfg(CLASS_NAME_TRACKED)]
impl Ord<UpdateHandlerDependency> for UpdateHandlerDependency {
    
    fn cmp(&self, other: &UpdateHandlerDependency) -> Ordering {
        todo!();
        /*
            return obj < d.obj;
        */
    }
}

#[cfg(CLASS_NAME_TRACKED)]
impl PartialOrd<UpdateHandlerDependency> for UpdateHandlerDependency {
    fn partial_cmp(&self, other: &UpdateHandlerDependency) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
