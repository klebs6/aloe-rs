crate::ix!();

pub struct ContinueStatement {
    base: Statement,
}

impl StatementInterface for ContinueStatement {

    fn perform(
        &self, 
        _0: &Scope,
        _1: *mut Var

    ) -> StatementResultCode {
        
        todo!();
        /*
            return continueWasHit;
        */
    }
}

impl ContinueStatement {

    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : statement(l),

        
        */
    }
}

