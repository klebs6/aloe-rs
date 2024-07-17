crate::ix!();

pub struct VarStatement {
    base:        Statement,
    name:        Identifier,
    initialiser: ExpPtr,
}

impl StatementInterface for VarStatement {

    fn perform(&self, 
        s:  &Scope,
        _1: *mut Var) -> StatementResultCode {
        
        todo!();
        /*
            s.scope->setProperty (name, initialiser->getResult (s));
                        return ok;
        */
    }
}

impl VarStatement {

    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : statement(l),

        
        */
    }
}
