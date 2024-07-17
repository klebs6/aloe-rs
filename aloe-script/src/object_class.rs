crate::ix!();

pub struct ObjectClass {
    base: DynamicObject,
}

impl Default for ObjectClass {
    
    fn default() -> Self {
        todo!();
        /*


            setMethod ("dump",  dump);
            setMethod ("clone", cloneFn);
        */
    }
}

impl ObjectClass {
    
    pub fn get_class_name() -> Identifier {
        
        todo!();
        /*
            static const Identifier i ("Object"); return i;
        */
    }
    
    pub fn dump(a: Args) -> Var {
        
        todo!();
        /*
            DBG (JSON::toString (a.thisObject)); ignoreUnused (a); return Var::undefined();
        */
    }
    
    pub fn clone_fn(a: Args) -> Var {
        
        todo!();
        /*
            return a.thisObject.clone();
        */
    }
}
