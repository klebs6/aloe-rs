crate::ix!();

pub struct FunctionCall {
    base:      Expression,
    object:    ExpPtr,
    arguments: Vec<Box<Expression>>,
}

impl FunctionCall {
    
    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : expression(l),

        
        */
    }
    
    pub fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            if (auto* dot = dynamic_cast<DotOperator*> (object.get()))
            {
                auto thisObject = dot->parent->getResult (s);
                return invokeFunction (s, s.findFunctionCall (location, thisObject, dot->child), thisObject);
            }

            auto function = object->getResult (s);
            return invokeFunction (s, function, Var (s.scope.get()));
        */
    }
    
    pub fn invoke_function(&self, 
        s:           &Scope,
        function:    &Var,
        this_object: &Var) -> Var {
        
        todo!();
        /*
            s.checkTimeOut (location);
            Vec<Var> argVars;

            for (auto* a : arguments)
                argVars.add (a->getResult (s));

            const Var::NativeFunctionArgs args (thisObject, argVars.begin(), argVars.size());

            if (Var::NativeFunction nativeFunction = function.getNativeFunction())
                return nativeFunction (args);

            if (auto* fo = dynamic_cast<FunctionObject*> (function.getObject()))
                return fo->invoke (s, args);

            if (auto* dot = dynamic_cast<DotOperator*> (object.get()))
                if (auto* o = thisObject.getDynamicObject())
                    if (o->hasMethod (dot->child)) // allow an overridden DynamicObject::invokeMethod to accept a method call.
                        return o->invokeMethod (dot->child, args);

            location.throwError ("This expression is not a function!"); return {};
        */
    }
}
