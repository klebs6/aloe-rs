crate::ix!();

pub struct PostAssignment {
    base: SelfAssignment,
}

impl PostAssignment {
    
    pub fn new(
        l:      &CodeLocation,
        dest:   *mut Expression,
        source: *mut Expression) -> Self {
    
        todo!();
        /*
        : self_assignment(l, dest, source),

        
        */
    }
    
    pub fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            auto oldValue = target->getResult (s);
            target->assign (s, newValue->getResult (s));
            return oldValue;
        */
    }
}
