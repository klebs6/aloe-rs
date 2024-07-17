crate::ix!();

#[no_copy]
pub struct SymbolListVisitor<'a> {
    list: &'a mut Vec<ExpressionSymbol>,
}

impl<'a> SymbolVisitor for SymbolListVisitor<'a> {

}

impl<'a> UseSymbol for SymbolListVisitor<'a> {

    fn use_symbol(&mut self, s: &ExpressionSymbol)  {
        
        todo!();
        /*
            list.addIfNotAlreadyThere (s);
        */
    }
}

impl<'a> SymbolListVisitor<'a> {
    
    pub fn new(list: &mut Vec<ExpressionSymbol>) -> Self {
    
        todo!();
        /*
        : list(list_),
        */
    }
}
