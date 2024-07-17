crate::ix!();

pub struct SelfAssignment {
    base: Expression,

    /**
      | Careful! this pointer aliases a sub-term
      | of newValue!
      |
      */
    target:    *mut Expression,

    new_value: ExpPtr,
    op:        TokenType,
}

impl SelfAssignment {

    pub fn new(
        l:      &CodeLocation,
        dest:   *mut Expression,
        source: *mut Expression) -> Self {
    
        todo!();
        /*
        : expression(l),
        : target(dest),
        : new_value(source),

        
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
