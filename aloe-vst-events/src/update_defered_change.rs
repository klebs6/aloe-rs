crate::ix!();

pub struct UpdateHandlerDeferedChange {
    obj: *mut dyn FUnknown,
    msg: i32,
}

impl PartialEq<UpdateHandlerDeferedChange> for UpdateHandlerDeferedChange {
    
    fn eq(&self, other: &UpdateHandlerDeferedChange) -> bool {
        todo!();
        /*
            return obj == d.obj;
        */
    }
}

impl UpdateHandlerDeferedChange {

    pub fn new(
        o: *mut dyn FUnknown,
        m: Option<i32>

    ) -> Self {

        let m: i32 = m.unwrap_or(0);

        todo!();
        /*
        : obj(o),
        : msg(m),

        
        */
    }
}

impl From<&UpdateHandlerDeferedChange> for UpdateHandlerDeferedChange {
    
    fn from(r: &UpdateHandlerDeferedChange) -> Self {
    
        todo!();
        /*
        : obj(r.obj),
        : msg(r.msg),

        
        */
    }
}
