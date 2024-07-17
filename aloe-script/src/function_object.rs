crate::ix!();

pub struct FunctionObject {
    base:          DynamicObject,
    function_code: String,
    parameters:    Vec<Identifier>,
    body:          Box<Statement>,
}

impl Default for FunctionObject {
    
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl FunctionObject {
    
    pub fn new(other: &FunctionObject) -> Self {
    
        todo!();
        /*
        : dynamic_object(),
        : function_code(other.functionCode),

            ExpressionTreeBuilder tb (functionCode);
            tb.parseFunctionParamsAndBody (*this);
        */
    }
    
    pub fn clone(&mut self) -> DynamicObjectPtr {
        
        todo!();
        /*
            return *new FunctionObject (*this);
        */
    }
    
    pub fn write_asjson(
        &mut self, 
        out:                    &mut dyn Write,
        indent_level:           i32,
        all_on_one_line:        bool,
        maximum_decimal_places: i32

    ) {
        
        todo!();
        /*
            out << "function " << functionCode;
        */
    }
    
    pub fn invoke(&self, 
        s:    &Scope,
        args: &VarNativeFunctionArgs) -> Var {
        
        todo!();
        /*
            DynamicObject::Ptr functionRoot (new DynamicObject());

            static const Identifier thisIdent ("this");
            functionRoot->setProperty (thisIdent, args.thisObject);

            for (int i = 0; i < parameters.size(); ++i)
                functionRoot->setProperty (parameters.getReference(i),
                i < args.numArguments ? args.arguments[i] : Var::undefined());

            Var result;
            body->perform (Scope (&s, s.root, functionRoot), &result);
            return result;
        */
    }
}
