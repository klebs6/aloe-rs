crate::ix!();

pub struct LiteralValue {
    base:  Expression,
    value: Var,
}

impl ExpressionInterface for LiteralValue {

    fn get_result(&self, _0: &Scope) -> Var {
        
        todo!();
        /*
            return value;
        */
    }
}

impl LiteralValue {

    pub fn new(
        l: &CodeLocation,
        v: &Var) -> Self {
    
        todo!();
        /*
        : expression(l),
        : value(v),

        
        */
    }
}
