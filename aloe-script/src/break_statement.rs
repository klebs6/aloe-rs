crate::ix!();

pub struct BreakStatement {
    base: Statement,
}

impl StatementInterface for BreakStatement {

    fn perform(
        &self, 
        _0: &Scope,
        _1: *mut Var

    ) -> StatementResultCode {
        
        todo!();
        /*
            return breakWasHit;
        */
    }
}

impl BreakStatement {

    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : statement(l),

        
        */
    }
}
