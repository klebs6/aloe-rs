crate::ix!();

pub struct Negate {
    base: Term,
    input: TermPtr,
}

impl Negate {

    pub fn new(t: &TermPtr) -> Self {
    
        todo!();
        /*
        : input(t),

            jassert (t != nullptr);
        */
    }
    
    pub fn get_type(&self) -> ExpressionType {
        
        todo!();
        /*
            return operatorType;
        */
    }
    
    pub fn get_input_index_for(&self, possible_input: *const Term) -> i32 {
        
        todo!();
        /*
            return possibleInput == input ? 0 : -1;
        */
    }
    
    pub fn get_num_inputs(&self) -> i32 {
        
        todo!();
        /*
            return 1;
        */
    }
    
    pub fn get_input(&self, index: i32) -> *mut Term {
        
        todo!();
        /*
            return index == 0 ? input.get() : nullptr;
        */
    }
    
    pub fn clone(&self) -> *mut Term {
        
        todo!();
        /*
            return new Negate (*input->clone());
        */
    }
    
    pub fn resolve(&mut self, 
        scope:           &dyn ExpressionScopeInterface,
        recursion_depth: i32) -> TermPtr {
        
        todo!();
        /*
            return *new Constant (-input->resolve (scope, recursionDepth)->toDouble(), false);
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return "-";
        */
    }
    
    pub fn negated(&mut self) -> TermPtr {
        
        todo!();
        /*
            return input;
        */
    }
    
    pub fn create_term_to_evaluate_input(&self, 
        scope:          &dyn ExpressionScopeInterface,
        t:              *const Term,
        overall_target: f64,
        top_level_term: *mut Term) -> TermPtr {
        
        todo!();
        /*
            ignoreUnused (t);
            jassert (t == input);

            const Term* const dest = findDestinationFor (topLevelTerm, this);

            return *new Negate (dest == nullptr ? TermPtr (*new Constant (overallTarget, false))
                                                : dest->createTermToEvaluateInput (scope, this, overallTarget, topLevelTerm));
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            if (input->getOperatorPrecedence() > 0)
                return "-(" + input->toString() + ")";

            return "-" + input->toString();
        */
    }
}
