crate::ix!();

pub struct DotOperatorSymbolVisitingVisitor<'a> {
    input:           TermPtr,
    visitor:         &'a mut dyn SymbolVisitor,
    recursion_count: i32,
}

impl<'a> ExpressionScopeVisitor for DotOperatorSymbolVisitingVisitor<'a> {

    fn visit(&mut self, scope: &dyn ExpressionScopeInterface)  {
        
        todo!();
        /*
            input->visitAllSymbols (visitor, scope, recursionCount);
        */
    }
}

impl<'a> DotOperatorSymbolVisitingVisitor<'a> {
    
    pub fn new(
        t:         &TermPtr,
        v:         &mut dyn SymbolVisitor,
        recursion: i32) -> Self {
    
        todo!();
        /*
        : input(t),
        : visitor(v),
        : recursion_count(recursion),

        
        */
    }
}
