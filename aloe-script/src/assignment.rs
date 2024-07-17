crate::ix!();

pub struct Assignment {
    base:      Expression,
    target:    ExpPtr,
    new_value: ExpPtr,
}

impl Assignment {

    pub fn new(
        l:      &CodeLocation,
        dest:   &mut ExpPtr,
        source: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : expression(l),
        : target(dest.release()),
        : new_value(source.release()),

        
        */
    }
    
    pub fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            auto value = newValue->getResult (s);
            target->assign (s, value);
            return value;
        */
    }
}
