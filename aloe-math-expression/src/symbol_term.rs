crate::ix!();

pub struct SymbolTerm {
    base:   Term,
    symbol: String,
}

impl SymbolTerm {

    pub fn new(sym: &String) -> Self {
    
        todo!();
        /*
        : symbol(sym),

        
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
            return scope.getSymbolValue (symbol).term->resolve (scope, recursionDepth + 1);
        */
    }
    
    pub fn get_type(&self) -> ExpressionType {
        
        todo!();
        /*
            return symbolType;
        */
    }
    
    pub fn clone(&self) -> *mut Term {
        
        todo!();
        /*
            return new SymbolTerm (symbol);
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return symbol;
        */
    }
    
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return symbol;
        */
    }
    
    pub fn visit_all_symbols(
        &mut self, 
        visitor:         &mut dyn SymbolVisitor,
        scope:           &dyn ExpressionScopeInterface,
        recursion_depth: i32

    )  {
        
        todo!();
        /*
            checkRecursionDepth (recursionDepth);
            visitor.useSymbol (ExpressionSymbol (scope.getScopeUID(), symbol));
            scope.getSymbolValue (symbol).term->visitAllSymbols (visitor, scope, recursionDepth + 1);
        */
    }
    
    pub fn rename_symbol(
        &mut self, 
        old_symbol:      &ExpressionSymbol,
        new_name:        &String,
        scope:           &dyn ExpressionScopeInterface,
        recursion_depth: i32

    )  {
        
        todo!();
        /*
            if (oldSymbol.symbolName == symbol && scope.getScopeUID() == oldSymbol.scopeUID)
                symbol = newName;
        */
    }
}
