crate::ix!();

pub trait SymbolVisitor:
UseSymbol {}

pub trait UseSymbol {
    
    fn use_symbol(&mut self, _0: &ExpressionSymbol);
}
