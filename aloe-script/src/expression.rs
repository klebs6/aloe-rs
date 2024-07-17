crate::ix!();

pub trait ExpressionInterface {

    fn get_result(&self, _0: &Scope) -> Var {
        
        todo!();
        /*
            return Var::undefined();
        */
    }
    
    fn assign(
        &self, 
        _0: &Scope,
        _1: &Var

    ) {
        
        todo!();
        /*
            location.throwError ("Cannot assign to this expression!");
        */
    }
}

pub struct Expression {
    base: Statement,
}

pub type ExpPtr = Box<Expression>;

impl StatementInterface for Expression {

    fn perform(
        &self, 
        s:  &Scope,
        _1: *mut Var

    ) -> StatementResultCode {
        
        todo!();
        /*
            getResult (s); return ok;
        */
    }
}

impl Expression {

    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : statement(l),

        
        */
    }
}
