crate::ix!();

pub struct ArraySubscript {
    base:   Expression,
    object: ExpPtr,
    index:  ExpPtr,
}

impl ExpressionInterface for ArraySubscript {

    fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            auto arrayVar = object->getResult (s); // must stay alive for the scope of this method
                        auto key = index->getResult (s);

                        if (const auto* array = arrayVar.getArray())
                            if (key.isInt() || key.isInt64() || key.isDouble())
                                return (*array) [static_cast<int> (key)];

                        if (auto* o = arrayVar.getDynamicObject())
                            if (key.isString())
                                if (auto* v = getPropertyPointer (*o, Identifier (key)))
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
            auto arrayVar = object->getResult (s); // must stay alive for the scope of this method
                        auto key = index->getResult (s);

                        if (auto* array = arrayVar.getArray())
                        {
                            if (key.isInt() || key.isInt64() || key.isDouble())
                            {
                                const int i = key;
                                while (array->size() < i)
                                    array->add (Var::undefined());

                                array->set (i, newValue);
                                return;
                            }
                        }

                        if (auto* o = arrayVar.getDynamicObject())
                        {
                            if (key.isString())
                            {
                                o->setProperty (Identifier (key), newValue);
                                return;
                            }
                        }

                        Expression::assign (s, newValue);
        */
    }
}

impl ArraySubscript {

    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : expression(l),

        
        */
    }
}
