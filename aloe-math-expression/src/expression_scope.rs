crate::ix!();

/**
  | Used as a callback by the Scope::visitRelativeScope()
  | method.
  | 
  | You should never create an instance
  | of this class yourself, it's used by
  | the expression evaluation code.
  |
  */
pub trait ExpressionScopeVisitor {

    fn visit(&mut self, _0: &dyn ExpressionScopeInterface);
}

/**
  | When evaluating an Expression object,
  | this class is used to resolve symbols
  | and perform functions that the expression
  | uses.
  |
  */
pub trait ExpressionScopeInterface {

    /**
      | Returns some kind of globally unique
      | ID that identifies this scope.
      |
      */
    fn get_scopeuid(&self) -> String;

    /**
      | Returns the value of a symbol.
      | 
      | If the symbol is unknown, this can throw
      | an Expression::EvaluationError exception.
      | 
      | The member value is set to the part of
      | the symbol that followed the dot, if
      | there is one, e.g. for "foo.bar", symbol
      | = "foo" and member = "bar". @throws Expression::EvaluationError
      |
      */
    fn get_symbol_value(&self, symbol: &String) -> Expression;

    /**
      | Executes a named function.
      | 
      | If the function name is unknown, this
      | can throw an Expression::EvaluationError
      | exception. @throws Expression::EvaluationError
      |
      */
    fn evaluate_function(
        &self, 
        function_name:  &String,
        parameters:     *const f64,
        num_parameters: i32

    ) -> f64;

    /**
      | Creates a Scope object for a named scope,
      | and then calls a visitor to do some kind
      | of processing with this new scope.
      | 
      | If the name is valid, this method must
      | create a suitable (temporary) Scope
      | object to represent it, and must call
      | the ExpressionScopeVisitor::visit() method with this
      | new scope.
      |
      */
    fn visit_relative_scope(
        &self, 
        scope_name: &String,
        visitor:    &mut dyn ExpressionScopeVisitor
    );
}

pub fn expression_scope_get_symbol_value(symbol: &String) -> Expression {
    
    todo!();
    /*
        if (symbol.isNotEmpty())
        throw EvaluationError ("Unknown symbol: " + symbol);

    return Expression();
    */
}

pub fn expression_scope_evaluate_function(
    function_name: &String,
    parameters:    *const f64,
    num_params:    i32

) -> f64 {
    
    todo!();
    /*
        if (numParams > 0)
    {
        if (functionName == "min")
        {
            double v = parameters[0];
            for (int i = 1; i < numParams; ++i)
                v = jmin (v, parameters[i]);

            return v;
        }

        if (functionName == "max")
        {
            double v = parameters[0];
            for (int i = 1; i < numParams; ++i)
                v = jmax (v, parameters[i]);

            return v;
        }

        if (numParams == 1)
        {
            if (functionName == "sin")  return std::sin (parameters[0]);
            if (functionName == "cos")  return std::cos (parameters[0]);
            if (functionName == "tan")  return std::tan (parameters[0]);
            if (functionName == "abs")  return std::abs (parameters[0]);
        }
    }

    throw EvaluationError ("Unknown function: \"" + functionName + "\"");
    */
}

pub fn expression_scope_visit_relative_scope(
    scope_name: &String,
    _1:         &mut dyn ExpressionScopeVisitor

) {
    
    todo!();
    /*
        throw EvaluationError ("Unknown symbol: " + scopeName);
    */
}

pub fn expression_scope_get_scopeuid() -> String {
    
    todo!();
    /*
        return {};
    */
}
