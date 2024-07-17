crate::ix!();

pub struct NewOperator {
    base: FunctionCall,
}

impl NewOperator {
    
    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : function_call(l),

        
        */
    }
    
    pub fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            Var classOrFunc = object->getResult (s);
            const bool isFunc = isFunction (classOrFunc);

            if (! (isFunc || classOrFunc.getDynamicObject() != nullptr))
                return Var::undefined();

            DynamicObject::Ptr newObject (new DynamicObject());

            if (isFunc)
                invokeFunction (s, classOrFunc, newObject.get());
            else
                newObject->setProperty (getPrototypeIdentifier(), classOrFunc);

            return newObject.get();
        */
    }
}
