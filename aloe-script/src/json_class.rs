crate::ix!();

pub struct JSONClass {
    base: DynamicObject,
}

impl Default for JSONClass {
    
    fn default() -> Self {
        todo!();
        /*
            setMethod ("stringify", stringify);
        */
    }
}

impl JSONClass {
    
    pub fn get_class_name() -> Identifier {
        
        todo!();
        /*
            static const Identifier i ("JSON"); return i;
        */
    }
    
    pub fn stringify(a: Args) -> Var {
        
        todo!();
        /*
            return JSON::toString (get (a, 0));
        */
    }
}
