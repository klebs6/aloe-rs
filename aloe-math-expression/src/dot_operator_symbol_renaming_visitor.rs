crate::ix!();

#[no_copy]
pub struct DotOperatorSymbolRenamingVisitor<'a> {
    input:           TermPtr,
    symbol:          &'a ExpressionSymbol,
    new_name:        String,
    recursion_count: i32,
}

impl<'a> ExpressionScopeVisitor for DotOperatorSymbolRenamingVisitor<'a> {

    fn visit(&mut self, scope: &dyn ExpressionScopeInterface)  {
        
        todo!();
        /*
            input->renameSymbol (symbol, newName, scope, recursionCount);
        */
    }
}

impl<'a> DotOperatorSymbolRenamingVisitor<'a> {
    
    pub fn new(
        t:               &TermPtr,
        symbol:          &ExpressionSymbol,
        new_name:        &String,
        recursion_count: i32) -> Self {
    
        todo!();
        /*
        : input(t),
        : symbol(symbol_),
        : new_name(newName_),
        : recursion_count(recursionCount_),

        
        */
    }
}
