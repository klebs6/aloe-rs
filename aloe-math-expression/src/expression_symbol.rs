crate::ix!();

/**
  | Represents a symbol that is used in an
  | Expression.
  |
  */
pub struct ExpressionSymbol {

    /**
      | The unique ID of the Scope that contains
      | this symbol.
      |
      */
    scopeuid:    String,

    /**
      | The name of the symbol.
      |
      */
    symbol_name: String,
}

impl PartialEq<ExpressionSymbol> for ExpressionSymbol {
    
    #[inline] fn eq(&self, other: &ExpressionSymbol) -> bool {
        todo!();
        /*
            return symbolName == other.symbolName && scopeUID == other.scopeUID;
        */
    }
}

impl Eq for ExpressionSymbol {}

impl ExpressionSymbol {
    
    pub fn new(
        scope:  &String,
        symbol: &String) -> Self {
    
        todo!();
        /*
        : scopeuid(scope),
        : symbol_name(symbol),

        
        */
    }
}
