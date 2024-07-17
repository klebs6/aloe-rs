crate::ix!();

pub trait TermInterface:
GetExpressionType
+ CloneTerm
+ Resolve
+ ToString 
+ ToDouble 
+ GetInputIndexForTerm 
+ GetOperatorPrecedence 
+ GetNumInputs 
+ GetInputTerm 
+ NegatedTerm 
+ CreateTermToEvaluateInput 
+ GetName 
+ RenameSymbol 
+ VisitAllSymbols 
{
    
}

//----------------------------
pub trait GetExpressionType {
    
    fn get_type(&self) -> ExpressionType;
}

pub trait CloneTerm {
    
    fn clone(&self) -> *mut Term;
}

pub trait Resolve {
    
    fn resolve(
        &mut self, 
        _0:              &dyn ExpressionScopeInterface,
        recursion_depth: i32

    ) -> ReferenceCountedObjectPtr<Term>;
}

pub trait ToString {

    fn to_string(&self) -> String;
}

pub trait ToDouble {

    fn to_double(&self) -> f64 { 0.0 }
}

pub trait GetInputIndexForTerm {

    fn get_input_index_for(&self, _0: *const Term) -> i32 { -1 }
}

pub trait GetOperatorPrecedence {

    fn get_operator_precedence(&self) -> i32 { 0 }
}

pub trait GetNumInputs {

    fn get_num_inputs(&self) -> i32 { 0 }
}

pub trait GetInputTerm {

    fn get_input(&self, _0: i32) -> *mut Term { null_mut() }
}

pub trait NegatedTerm {
    
    fn negated(&mut self) -> ReferenceCountedObjectPtr<Term> {
        
        todo!();
        /*
            return *new Negate (*this);
        */
    }
}

pub trait CreateTermToEvaluateInput {

    fn create_term_to_evaluate_input(
        &self, 
        _0:             &dyn ExpressionScopeInterface,
        input_term:     *const Term,
        overall_target: f64,
        top_level_term: *mut Term

    ) -> ReferenceCountedObjectPtr<Term> {
        
        todo!();
        /*
            jassertfalse;
            return ReferenceCountedObjectPtr<Term>();
        */
    }
}

pub trait RenameSymbol {

    fn rename_symbol(
        &mut self, 
        old_symbol:      &ExpressionSymbol,
        new_name:        &String,
        scope:           &dyn ExpressionScopeInterface,
        recursion_depth: i32

    )  {
        
        todo!();
        /*
            for (int i = getNumInputs(); --i >= 0;)
                getInput (i)->renameSymbol (oldSymbol, newName, scope, recursionDepth);
        */
    }
}

pub trait VisitAllSymbols {

    fn visit_all_symbols(
        &mut self, 
        visitor:         &mut dyn SymbolVisitor,
        scope:           &dyn ExpressionScopeInterface,
        recursion_depth: i32

    )  {
        
        todo!();
        /*
            for (int i = getNumInputs(); --i >= 0;)
                getInput(i)->visitAllSymbols (visitor, scope, recursionDepth);
        */
    }
}

//-------------------------------------
pub type TermPtr = ReferenceCountedObjectPtr<Term>;

#[derive(Default)]
#[no_copy]
pub struct Term {
    base: SingleThreadedReferenceCountedObject,
}
