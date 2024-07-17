crate::ix!();

#[no_copy]
pub struct Multiply {
    base: BinaryTerm,
}

impl Multiply {
    
    pub fn new(
        l: TermPtr,
        r: TermPtr) -> Self {
    
        todo!();
        /*
        : binary_term(l, r),

        
        */
    }
    
    pub fn clone(&self) -> *mut Term {
        
        todo!();
        /*
            return new Multiply (*left->clone(), *right->clone());
        */
    }
    
    pub fn perform_function(&self, 
        lhs: f64,
        rhs: f64) -> f64 {
        
        todo!();
        /*
            return lhs * rhs;
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return "*";
        */
    }
    
    pub fn write_operator(&self, dest: &mut String)  {
        
        todo!();
        /*
            dest << " * ";
        */
    }
    
    pub fn get_operator_precedence(&self) -> i32 {
        
        todo!();
        /*
            return 2;
        */
    }
    
    pub fn create_term_to_evaluate_input(&self, 
        scope:          &dyn ExpressionScopeInterface,
        input:          *const Term,
        overall_target: f64,
        top_level_term: *mut Term) -> TermPtr {
        
        todo!();
        /*
            if (auto newDest = createDestinationTerm (scope, input, overallTarget, topLevelTerm))
                return *new Divide (newDest, *(input == left ? right : left)->clone());

            return {};
        */
    }
}
