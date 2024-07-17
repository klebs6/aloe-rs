crate::ix!();

#[no_copy]
pub struct SymbolCheckVisitor<'a> {
    was_found: bool, // default = false
    symbol:    &'a ExpressionSymbol,
}

impl<'a> SymbolVisitor for SymbolCheckVisitor<'a> {

}

impl<'a> UseSymbol for SymbolCheckVisitor<'a> {

    fn use_symbol(&mut self, s: &ExpressionSymbol)  {
        
        todo!();
        /*
            wasFound = wasFound || s == symbol;
        */
    }
}

impl<'a> SymbolCheckVisitor<'a> {
    
    pub fn new(s: &ExpressionSymbol) -> Self {
    
        todo!();
        /*
        : symbol(s),

        
        */
    }
}
