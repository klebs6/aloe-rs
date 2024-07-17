crate::ix!();

pub struct BinaryOperatorBase {
    base:      Expression,
    lhs:       ExpPtr,
    rhs:       ExpPtr,
    operation: TokenType,
}

impl BinaryOperatorBase {

    pub fn new(
        l:  &CodeLocation,
        a:  &mut ExpPtr,
        b:  &mut ExpPtr,
        op: TokenType) -> Self {
    
        todo!();
        /*
        : expression(l),
        : lhs(a.release()),
        : rhs(b.release()),
        : operation(op),

        
        */
    }
}
