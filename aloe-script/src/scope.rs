crate::ix!();

#[no_copy]
#[leak_detector]
pub struct Scope {
    parent: *const Scope,
    root:   ReferenceCountedObjectPtr<JavascriptEngineRootObject>,
    scope:  DynamicObjectPtr,
}

impl Scope {
    
    pub fn new(
        p:   *const Scope,
        rt:  ReferenceCountedObjectPtr<JavascriptEngineRootObject>,
        scp: DynamicObjectPtr

    ) -> Self {
    
        todo!();
        /*


            : parent (p), root (std::move (rt)),
                        scope (std::move (scp))
        */
    }
    
    pub fn find_function_call(
        &self, 
        location:      &CodeLocation,
        target_object: &Var,
        function_name: &Identifier

    ) -> Var {
        
        todo!();
        /*
            if (auto* o = targetObject.getDynamicObject())
                        {
                            if (auto* prop = getPropertyPointer (*o, functionName))
                                return *prop;

                            for (auto* p = o->getProperty (getPrototypeIdentifier()).getDynamicObject(); p != nullptr;
                                p = p->getProperty (getPrototypeIdentifier()).getDynamicObject())
                            {
                                if (auto* prop = getPropertyPointer (*p, functionName))
                                    return *prop;
                            }

                            // if there's a class with an overridden DynamicObject::hasMethod, this avoids an error
                            if (o->hasMethod (functionName))
                                return {};
                        }

                        if (targetObject.isString())
                            if (auto* m = findRootClassProperty (StringClass::getClassName(), functionName))
                                return *m;

                        if (targetObject.isArray())
                            if (auto* m = findRootClassProperty (ArrayClass::getClassName(), functionName))
                                return *m;

                        if (auto* m = findRootClassProperty (ObjectClass::getClassName(), functionName))
                            return *m;

                        location.throwError ("Unknown function '" + functionName.toString() + "'");
                        return {};
        */
    }
    
    pub fn find_root_class_property(
        &self, 
        class_name: &Identifier,
        prop_name:  &Identifier

    ) -> *mut Var {
        
        todo!();
        /*
            if (auto* cls = root->getProperty (className).getDynamicObject())
                            return getPropertyPointer (*cls, propName);

                        return nullptr;
        */
    }
    
    pub fn find_symbol_in_parent_scopes(&self, name: &Identifier) -> Var {
        
        todo!();
        /*
            if (auto v = getPropertyPointer (*scope, name))
                            return *v;

                        return parent != nullptr ? parent->findSymbolInParentScopes (name)
                            : Var::undefined();
        */
    }
    
    pub fn find_and_invoke_method(
        &self, 
        function: &Identifier,
        args:     &VarNativeFunctionArgs,
        result:   &mut Var

    ) -> bool {
        
        todo!();
        /*
            auto* target = args.thisObject.getDynamicObject();

                        if (target == nullptr || target == scope.get())
                        {
                            if (auto* m = getPropertyPointer (*scope, function))
                            {
                                if (auto fo = dynamic_cast<FunctionObject*> (m->getObject()))
                                {
                                    result = fo->invoke (*this, args);
                                    return true;
                                }
                            }
                        }

                        const auto& props = scope->getProperties();

                        for (int i = 0; i < props.size(); ++i)
                            if (auto* o = props.getValueAt (i).getDynamicObject())
                                if (Scope (this, *root, *o).findAndInvokeMethod (function, args, result))
                                    return true;

                        return false;
        */
    }
    
    pub fn invoke_method(
        &self, 
        m:      &Var,
        args:   &VarNativeFunctionArgs,
        result: &mut Var

    ) -> bool {
        
        todo!();
        /*
            if (isFunction (m))
                        {
                            auto* target = args.thisObject.getDynamicObject();

                            if (target == nullptr || target == scope.get())
                            {
                                if (auto fo = dynamic_cast<FunctionObject*> (m.getObject()))
                                {
                                    result = fo->invoke (*this, args);
                                    return true;
                                }
                            }
                        }

                        return false;
        */
    }
    
    pub fn check_time_out(&self, location: &CodeLocation)  {
        
        todo!();
        /*
            if (Time::getCurrentTime() > root->timeout)
                            location.throwError (root->timeout == Time() ? "Interrupted" : "Execution timed-out");
        */
    }
}
