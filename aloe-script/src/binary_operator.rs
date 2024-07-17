crate::ix!();

pub struct BinaryOperator {
    base: BinaryOperatorBase,
}

impl ExpressionInterface for BinaryOperator {

    fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            Var a (lhs->getResult (s)), b (rhs->getResult (s));

                        if ((a.isUndefined() || a.isVoid()) && (b.isUndefined() || b.isVoid()))
                            return getWithUndefinedArg();

                        if (isNumericOrUndefined (a) && isNumericOrUndefined (b))
                            return (a.isDouble() || b.isDouble()) ? getWithDoubles (a, b) : getWithInts (a, b);

                        if (a.isArray() || a.isObject())
                            return getWithArrayOrObject (a, b);

                        return getWithStrings (a.toString(), b.toString());
        */
    }
}

impl BinaryOperator {

    pub fn new(
        l:  &CodeLocation,
        a:  &mut ExpPtr,
        b:  &mut ExpPtr,
        op: TokenType) -> Self {
    
        todo!();
        /*
        : binary_operator_base(l, a, b, op),

        
        */
    }
    
    pub fn throw_error(&self, type_name: *const u8) -> Var {
        
        todo!();
        /*
            location.throwError (getTokenName (operation) + " is not allowed on the " + typeName + " type"); return {};
        */
    }
}
