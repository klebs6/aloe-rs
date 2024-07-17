crate::ix!();

pub struct UpdateHandlerUpdateData {
    obj:        *mut dyn FUnknown,
    dependents: *mut *mut dyn IDependent,
    count:      u32,
}

impl PartialEq<UpdateHandlerUpdateData> for UpdateHandlerUpdateData {
    
    #[inline] fn eq(&self, other: &UpdateHandlerUpdateData) -> bool {
        todo!();
        /*
            return d.obj == obj && d.dependents == dependents;
        */
    }
}

impl Eq for UpdateHandlerUpdateData {}

impl UpdateHandlerUpdateData {

    pub fn new(
        o: *mut dyn FUnknown,
        d: *mut *mut dyn IDependent,
        c: u32) -> Self {
    
        todo!();
        /*
        : obj(o),
        : dependents(d),
        : count(c),

        
        */
    }
}
