crate::ix!();

pub struct IfStatement {
    base:         Statement,
    condition:    ExpPtr,
    true_branch:  Box<Statement>,
    false_branch: Box<Statement>,
}

impl StatementInterface for IfStatement {

    fn perform(
        &self, 
        s:              &Scope,
        returned_value: *mut Var

    ) -> StatementResultCode {
        
        todo!();
        /*
            return (condition->getResult(s) ? trueBranch : falseBranch)->perform (s, returnedValue);
        */
    }
}

impl IfStatement {

    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : statement(l),

        
        */
    }
}
