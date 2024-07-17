crate::ix!();

pub struct Function {
    base:          Term,
    function_name: String,
    parameters:    Vec<Expression>,
}

impl Function {
    
    pub fn new(name: &String) -> Self {
    
        todo!();
        /*
        : function_name(name),

        
        */
    }
    
    pub fn new_with_params(
        name:   &String,
        params: &Vec<Expression>) -> Self {
    
        todo!();
        /*
        : function_name(name),
        : parameters(params),

        
        */
    }
    
    pub fn get_type(&self) -> ExpressionType {
        
        todo!();
        /*
            return functionType;
        */
    }
    
    pub fn clone(&self) -> *mut Term {
        
        todo!();
        /*
            return new Function (functionName, parameters);
        */
    }
    
    pub fn get_num_inputs(&self) -> i32 {
        
        todo!();
        /*
            return parameters.size();
        */
    }
    
    pub fn get_input(&self, i: i32) -> *mut Term {
        
        todo!();
        /*
            return parameters.getReference(i).term.get();
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return functionName;
        */
    }
    
    pub fn resolve(&mut self, 
        scope:           &dyn ExpressionScopeInterface,
        recursion_depth: i32) -> TermPtr {
        
        todo!();
        /*
            checkRecursionDepth (recursionDepth);
            double result = 0;
            auto numParams = parameters.size();

            if (numParams > 0)
            {
                HeapBlock<double> params (numParams);

                for (int i = 0; i < numParams; ++i)
                    params[i] = parameters.getReference(i).term->resolve (scope, recursionDepth + 1)->toDouble();

                result = scope.evaluateFunction (functionName, params, numParams);
            }
            else
            {
                result = scope.evaluateFunction (functionName, nullptr, 0);
            }

            return *new Constant (result, false);
        */
    }
    
    pub fn get_input_index_for(&self, possible_input: *const Term) -> i32 {
        
        todo!();
        /*
            for (int i = 0; i < parameters.size(); ++i)
                if (parameters.getReference(i).term == possibleInput)
                    return i;

            return -1;
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            if (parameters.size() == 0)
                return functionName + "()";

            String s (functionName + " (");

            for (int i = 0; i < parameters.size(); ++i)
            {
                s << parameters.getReference(i).term->toString();

                if (i < parameters.size() - 1)
                    s << ", ";
            }

            s << ')';
            return s;
        */
    }
}
