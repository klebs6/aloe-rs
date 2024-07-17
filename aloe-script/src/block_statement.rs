crate::ix!();

pub struct BlockStatement {
    base:       Statement,
    statements: Vec<Box<Statement>>,
}

impl StatementInterface for BlockStatement {

    fn perform(
        &self, 
        s:              &Scope,
        returned_value: *mut Var

    ) -> StatementResultCode {
        
        todo!();
        /*
            for (auto* statement : statements)
                            if (auto r = statement->perform (s, returnedValue))
                                return r;

                        return ok;
        */
    }
}

impl BlockStatement {

    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : statement(l),

        
        */
    }
}
