crate::ix!();

pub struct UnqualifiedName {
    base: Expression,
    name: Identifier,
}

impl ExpressionInterface for UnqualifiedName {

    fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            return s.findSymbolInParentScopes (name);
        */
    }
    
    fn assign(&self, 
        s:         &Scope,
        new_value: &Var)  {
        
        todo!();
        /*
            if (auto* v = getPropertyPointer (*s.scope, name))
                            *v = newValue;
                        else
                            s.root->setProperty (name, newValue);
        */
    }
}

impl UnqualifiedName {

    pub fn new(
        l: &CodeLocation,
        n: &Identifier) -> Self {
    
        todo!();
        /*
        : expression(l),
        : name(n),

        
        */
    }
}
