crate::ix!();

/**
  | An expression context that can evaluate
  | expressions using "this"
  |
  */
#[no_copy]
pub struct RelativeRectangleLocalScope<'a> {
    rect: &'a RelativeRectangle,
}

impl<'a> ExpressionScopeInterface for RelativeRectangleLocalScope<'a> {

    fn get_symbol_value(&self, symbol: &String) -> Expression {
        
        todo!();
        /*
            switch (RelativeCoordinate::StandardStrings::getTypeOf (symbol))
            {
                case RelativeCoordinate::StandardStrings::x:
                case RelativeCoordinate::StandardStrings::left:     return rect.left.getExpression();
                case RelativeCoordinate::StandardStrings::y:
                case RelativeCoordinate::StandardStrings::top:      return rect.top.getExpression();
                case RelativeCoordinate::StandardStrings::right:    return rect.right.getExpression();
                case RelativeCoordinate::StandardStrings::bottom:   return rect.bottom.getExpression();
                case RelativeCoordinate::StandardStrings::width:
                case RelativeCoordinate::StandardStrings::height:
                case RelativeCoordinate::StandardStrings::parent:
                case RelativeCoordinate::StandardStrings::unknown:
                default: break;
            }

            return ExpressionScope::getSymbolValue (symbol);
        */
    }

    fn get_scopeuid(&self) -> String { 
        todo!() 
    }

    fn evaluate_function(&self, _: &String, _: *const f64, _: i32) -> f64 { 
        todo!() 
    }

    fn visit_relative_scope(&self, _: &String, _: &mut dyn ExpressionScopeVisitor) { 
        todo!() 
    }

}

impl<'a> RelativeRectangleLocalScope<'a> {

    pub fn new(rect: &RelativeRectangle) -> Self {
    
        todo!();
        /*
        : rect(rect_),

        
        */
    }
}
