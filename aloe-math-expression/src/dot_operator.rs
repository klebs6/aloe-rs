crate::ix!();

#[no_copy]
pub struct DotOperator {
    base: BinaryTerm,
}

impl DotOperator {
    
    pub fn new(
        l: *mut SymbolTerm,
        r: TermPtr) -> Self {
    
        todo!();
        /*
        : binary_term(TermPtr (l), r),

        
        */
    }
    
    pub fn resolve(
        &mut self, 
        scope:           &dyn ExpressionScopeInterface,
        recursion_depth: i32

    ) -> TermPtr {
        
        todo!();
        /*
            checkRecursionDepth (recursionDepth);

            DotOperatorEvaluationVisitor visitor (right, recursionDepth + 1);
            scope.visitRelativeScope (getSymbol()->symbol, visitor);
            return visitor.output;
        */
    }
    
    pub fn clone(&self) -> *mut Term {
        
        todo!();
        /*
            return new DotOperator (getSymbol(), *right);
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return ".";
        */
    }
    
    pub fn get_operator_precedence(&self) -> i32 {
        
        todo!();
        /*
            return 1;
        */
    }
    
    pub fn write_operator(&self, dest: &mut String)  {
        
        todo!();
        /*
            dest << '.';
        */
    }
    
    pub fn perform_function(&self, _0: f64, _1: f64) -> f64 {
        
        todo!();
        /*
            return 0.0;
        */
    }
    
    pub fn visit_all_symbols(
        &mut self, 
        visitor:         &mut dyn SymbolVisitor,
        scope:           &dyn ExpressionScopeInterface,
        recursion_depth: i32

    ) {
        
        todo!();
        /*
            checkRecursionDepth (recursionDepth);
            visitor.useSymbol (Symbol (scope.getScopeUID(), getSymbol()->symbol));

            DotOperatorSymbolVisitingVisitor v (right, visitor, recursionDepth + 1);

            try
            {
                scope.visitRelativeScope (getSymbol()->symbol, v);
            }
            catch (...) {}
        */
    }
    
    pub fn rename_symbol(
        &mut self, 
        old_symbol:      &ExpressionSymbol,
        new_name:        &String,
        scope:           &dyn ExpressionScopeInterface,
        recursion_depth: i32

    ) {
        
        todo!();
        /*
            checkRecursionDepth (recursionDepth);
            getSymbol()->renameSymbol (oldSymbol, newName, scope, recursionDepth);

            DotOperatorSymbolRenamingVisitor visitor (right, oldSymbol, newName, recursionDepth + 1);

            try
            {
                scope.visitRelativeScope (getSymbol()->symbol, visitor);
            }
            catch (...) {}
        */
    }
    
    pub fn get_symbol(&self) -> *mut SymbolTerm {
        
        todo!();
        /*
            return static_cast<SymbolTerm*> (left.get());
        */
    }
}
