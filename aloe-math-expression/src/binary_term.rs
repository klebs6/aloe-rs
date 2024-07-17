crate::ix!();

pub trait BinaryTermInterface:
PerformFunction
+ WriteOperator {}

pub trait PerformFunction {
    
    fn perform_function(&self, 
        left:  f64,
        right: f64) -> f64;
}

pub trait WriteOperator {

    fn write_operator(&self, dest: &mut String);
}

//---------------------------------
pub struct BinaryTerm {
    base:  Term,
    left:  TermPtr,
    right: TermPtr,
}

impl BinaryTerm {

    pub fn new(
        l: TermPtr,
        r: TermPtr) -> Self {
    
        todo!();
        /*
        : left(std::move (l)),
        : right(std::move (r)),

            jassert (left != nullptr && right != nullptr);
        */
    }
    
    pub fn get_input_index_for(&self, possible_input: *const Term) -> i32 {
        
        todo!();
        /*
            return possibleInput == left ? 0 : (possibleInput == right ? 1 : -1);
        */
    }
    
    pub fn get_type(&self) -> ExpressionType {
        
        todo!();
        /*
            return operatorType;
        */
    }
    
    pub fn get_num_inputs(&self) -> i32 {
        
        todo!();
        /*
            return 2;
        */
    }
    
    pub fn get_input(&self, index: i32) -> *mut Term {
        
        todo!();
        /*
            return index == 0 ? left.get() : (index == 1 ? right.get() : nullptr);
        */
    }
    
    pub fn resolve(
        &mut self, 
        scope:           &dyn ExpressionScopeInterface,
        recursion_depth: i32

    ) -> TermPtr {
        
        todo!();
        /*
            return *new Constant (performFunction (left ->resolve (scope, recursionDepth)->toDouble(),
                                                   right->resolve (scope, recursionDepth)->toDouble()), false);
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            String s;
            auto ourPrecendence = getOperatorPrecedence();

            if (left->getOperatorPrecedence() > ourPrecendence)
                s << '(' << left->toString() << ')';
            else
                s = left->toString();

            writeOperator (s);

            if (right->getOperatorPrecedence() >= ourPrecendence)
                s << '(' << right->toString() << ')';
            else
                s << right->toString();

            return s;
        */
    }
    
    pub fn create_destination_term(
        &self, 
        scope:          &dyn ExpressionScopeInterface,
        input:          *const Term,
        overall_target: f64,
        top_level_term: *mut Term

    ) -> TermPtr {
        
        todo!();
        /*
            jassert (input == left || input == right);
            if (input != left && input != right)
                return {};

            if (auto dest = findDestinationFor (topLevelTerm, this))
                return dest->createTermToEvaluateInput (scope, this, overallTarget, topLevelTerm);

            return *new Constant (overallTarget, false);
        */
    }
}
