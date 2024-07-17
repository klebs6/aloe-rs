crate::ix!();

pub struct DotOperator {
    base:   Expression,
    parent: ExpPtr,
    child:  Identifier,
}

impl ExpressionInterface for DotOperator {

    fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            auto p = parent->getResult (s);
                        static const Identifier lengthID ("length");

                        if (child == lengthID)
                        {
                            if (auto* array = p.getArray())   return array->size();
                            if (p.isString())                 return p.toString().length();
                        }

                        if (auto* o = p.getDynamicObject())
                            if (auto* v = getPropertyPointer (*o, child))
                                return *v;

                        return Var::undefined();
        */
    }
    
    fn assign(
        &self, 
        s:         &Scope,
        new_value: &Var

    ) {
        
        todo!();
        /*
            if (auto* o = parent->getResult (s).getDynamicObject())
                            o->setProperty (child, newValue);
                        else
                            Expression::assign (s, newValue);
        */
    }
}

impl DotOperator {

    pub fn new(
        l: &CodeLocation,
        p: &mut ExpPtr,
        c: &Identifier) -> Self {
    
        todo!();
        /*
        : expression(l),
        : parent(p.release()),
        : child(c),

        
        */
    }
}
