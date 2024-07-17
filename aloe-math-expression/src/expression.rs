crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/maths/aloe_Expression.h]

/**
  | Expression type. @see Expression::getType()
  |
  */
pub enum ExpressionType
{
    constantType,
    functionType,
    operatorType,
    symbolType
}

/**
  | A class for dynamically evaluating
  | simple numeric expressions.
  | 
  | This class can parse a simple C-style
  | string expression involving floating
  | point numbers, named symbols and functions.
  | The basic arithmetic operations of
  | +, -, *, / are supported, as well as parentheses,
  | and any alphanumeric identifiers are
  | assumed to be named symbols which will
  | be resolved when the expression is evaluated.
  | 
  | Expressions which use identifiers
  | and functions require a subclass of
  | Expression::dyn ExpressionScopeInterface to be supplied when
  | evaluating them, and this object is
  | expected to be able to resolve the symbol
  | names and perform the functions that
  | are used.
  | 
  | @tags{Core}
  |
  */
pub struct Expression {
    term: ReferenceCountedObjectPtr<Term>,
}

impl Clone for Expression {

    fn clone(&self) -> Self {
        todo!();
        /*
        : term(std::move (self.term)),

        
        */
    }
}

impl Add<&Expression> for Expression {

    type Output = Self;

    /**
      | Returns an expression which is an addition
      | operation of two existing expressions.
      |
      */
    #[inline] fn add(self, other: &Expression) -> Self::Output {
        todo!();
        /*
            return Expression (new Add (term, other.term));
        */
    }
}

impl Sub<&Expression> for Expression {

    type Output = Expression;
    
    /**
      | Returns an expression which is a subtraction
      | operation of two existing expressions.
      |
      */
    #[inline] fn sub(self, other: &Expression) -> Self::Output {
        todo!();
        /*
            return Expression (new Subtract (term, other.term));
        */
    }
}

impl Mul<&Expression> for Expression {
    type Output = Expression;

    /**
      | Returns an expression which is a multiplication
      | operation of two existing expressions.
      |
      */
    #[inline] fn mul(self, other: &Expression) -> Self::Output {
        todo!();
        /*
            return Expression (new Multiply (term, other.term));
        */
    }
}

impl Div<&Expression> for Expression {

    type Output = Expression;
    
    /**
      | Returns an expression which is a division
      | operation of two existing expressions.
      |
      */
    #[inline] fn div(self, other: &Expression) -> Self::Output {
        todo!();
        /*
            return Expression (new Divide (term, other.term));
        */
    }
}

impl Neg for Expression {
    type Output = Self;

    /**
      | Returns an expression which performs
      | a negation operation on an existing
      | expression.
      |
      */
    #[inline] fn neg(self) -> Self::Output {
        todo!();
        /*
            return Expression (term->negated().get());
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/maths/aloe_Expression.cpp]
impl Default for Expression {

    /**
      | Creates a simple expression with a value
      | of 0.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
            : term (new Expression::Constant (0, false))
        */
    }
}
    
impl From<*mut Term> for Expression {

    fn from(t: *mut Term) -> Self {
    
        todo!();
        /*
        : term(t),

            jassert (term != nullptr);
        */
    }
}
    
impl From<f64> for Expression {

    /**
      | Creates a simple expression with a specified
      | constant value.
      |
      */
    fn from(constant: f64) -> Self {
    
        todo!();
        /*


            : term (new Expression::Constant (constant, false))
        */
    }
}

impl From<&Expression> for Expression {
    
    /**
      | Creates a copy of an expression.
      |
      */
    fn from(other: &Expression) -> Self {
    
        todo!();
        /*


            : term (other.term)
        */
    }
}
    
impl Expression {

    /**
      | Copies another expression.
      |
      */
    pub fn assign_from_ref(&mut self, other: &Expression) -> &mut Expression {
        
        todo!();
        /*
            term = other.term;
        return *this;
        */
    }
    
    pub fn assign_from(&mut self, other: Expression) -> &mut Expression {
        
        todo!();
        /*
            term = std::move (other.term);
        return *this;
        */
    }
    
    /**
      | Attempts to create an expression by
      | parsing a string.
      | 
      | Any errors are returned in the parseError
      | argument provided.
      |
      */
    pub fn new(
        string_to_parse: &String,
        parse_error:     &mut String) -> Self {
    
        todo!();
        /*


            auto text = stringToParse.getCharPointer();
        Parser parser (text);
        term = parser.readUpToComma();
        parseError = parser.error;
        */
    }
    
    /**
      | Returns an Expression which parses
      | a string from a character pointer, and
      | updates the pointer to indicate where
      | it finished.
      | 
      | The pointer is incremented so that on
      | return, it indicates the character
      | that follows the end of the expression
      | that was parsed.
      | 
      | If there's a syntax error in parsing,
      | the parseError argument will be set
      | to a description of the problem.
      |
      */
    pub fn parse(&mut self, 
        string_to_parse: &mut CharPointerType,
        parse_error:     &mut String) -> Expression {
        
        todo!();
        /*
            Parser parser (stringToParse);
        Expression e (parser.readUpToComma().get());
        parseError = parser.error;
        return e;
        */
    }
    
    /**
      | Evaluates this expression, without
      | using a dyn ExpressionScopeInterface.
      | 
      | Without a dyn ExpressionScopeInterface, no symbols can be used,
      | and only basic functions such as sin,
      | cos, tan, min, max are available.
      | 
      | To find out about any errors during evaluation,
      | use the other version of this method
      | which takes a String parameter.
      |
      */
    pub fn evaluate(&self) -> f64 {
        
        todo!();
        /*
            return evaluate (Expression::dyn ExpressionScopeInterface());
        */
    }
    
    /**
      | Evaluates this expression, providing
      | a scope that should be able to evaluate
      | any symbols or functions that it uses.
      | 
      | To find out about any errors during evaluation,
      | use the other version of this method
      | which takes a String parameter.
      |
      */
    pub fn evaluate_with_expression_scope(&self, scope: &dyn ExpressionScopeInterface) -> f64 {
        
        todo!();
        /*
            String err;
        return evaluate (scope, err);
        */
    }
    
    /**
      | Evaluates this expression, providing
      | a scope that should be able to evaluate
      | any symbols or functions that it uses.
      |
      */
    pub fn evaluate_with_error(
        &self, 
        scope:            &dyn ExpressionScopeInterface,
        evaluation_error: &mut String

    ) -> f64 {
        
        todo!();
        /*
            try
        {
            return term->resolve (scope, 0)->toDouble();
        }
        catch (EvaluationError& e)
        {
            evaluationError = e.description;
        }

        return 0;
        */
    }
    
    /**
      | Returns an Expression which is an identifier
      | reference.
      |
      */
    pub fn symbol(&mut self, symbol: &String) -> Expression {
        
        todo!();
        /*
            return Expression (new SymbolTerm (symbol));
        */
    }
    
    /**
      | Returns an Expression which is a function
      | call.
      |
      */
    pub fn function(
        &mut self, 
        function_name: &String,
        parameters:    &[Expression]

    ) -> Expression {
        
        todo!();
        /*
            return Expression (new Function (functionName, parameters));
        */
    }
    
    /**
      | Attempts to return an expression which
      | is a copy of this one, but with a constant
      | adjusted to make the expression resolve
      | to a target value.
      | 
      | E.g. if the expression is "x + 10" and
      | x is 5, then asking for a target value
      | of 8 will return the expression "x + 3".
      | Obviously some expressions can't be
      | reversed in this way, in which case they
      | might just be adjusted by adding a constant
      | to the original expression.
      | 
      | @throws Expression::EvaluationError
      |
      */
    pub fn adjusted_to_give_new_result(&self, 
        target_value: f64,
        scope:        &dyn ExpressionScopeInterface) -> Expression {
        
        todo!();
        /*
            std::unique_ptr<Term> newTerm (term->clone());

        auto termToAdjust = findTermToAdjust (newTerm.get(), true);

        if (termToAdjust == nullptr)
            termToAdjust = findTermToAdjust (newTerm.get(), false);

        if (termToAdjust == nullptr)
        {
            newTerm.reset (new Add (*newTerm.release(), *new Constant (0, false)));
            termToAdjust = findTermToAdjust (newTerm.get(), false);
        }

        jassert (termToAdjust != nullptr);

        if (const Term* parent = findDestinationFor (newTerm.get(), termToAdjust))
        {
            if (TermPtr reverseTerm = parent->createTermToEvaluateInput (scope, termToAdjust, targetValue, newTerm.get()))
                termToAdjust->value = Expression (reverseTerm.get()).evaluate (scope);
            else
                return Expression (targetValue);
        }
        else
        {
            termToAdjust->value = targetValue;
        }

        return Expression (newTerm.release());
        */
    }
    
    /**
      | Returns a copy of this expression in
      | which all instances of a given symbol
      | have been renamed.
      |
      */
    pub fn with_renamed_symbol(&self, 
        old_symbol: &ExpressionSymbol,
        new_name:   &String,
        scope:      &dyn ExpressionScopeInterface) -> Expression {
        
        todo!();
        /*
            jassert (newName.toLowerCase().containsOnly ("abcdefghijklmnopqrstuvwxyz0123456789_"));

        if (oldSymbol.symbolName == newName)
            return *this;

        Expression e (term->clone());
        e.term->renameSymbol (oldSymbol, newName, scope, 0);
        return e;
        */
    }
    
    /**
      | Returns true if this expression makes
      | use of the specified symbol.
      | 
      | If a suitable scope is supplied, the
      | search will dereference and recursively
      | check all symbols, so that it can be determined
      | whether this expression relies on the
      | given symbol at any level in its evaluation.
      | If the scope parameter is null, this
      | just checks whether the expression
      | contains any direct references to the
      | symbol.
      | 
      | @throws Expression::EvaluationError
      |
      */
    pub fn references_symbol(
        &self, 
        symbol_to_check: &ExpressionSymbol,
        scope:           &dyn ExpressionScopeInterface

    ) -> bool {
        
        todo!();
        /*
            SymbolCheckVisitor visitor (symbolToCheck);

        try
        {
            term->visitAllSymbols (visitor, scope, 0);
        }
        catch (EvaluationError&)
        {}

        return visitor.wasFound;
        */
    }
    
    /**
      | Returns a list of all symbols that may
      | be needed to resolve this expression
      | in the given scope.
      |
      */
    pub fn find_referenced_symbols(&self, 
        results: &mut Vec<ExpressionSymbol>,
        scope:   &dyn ExpressionScopeInterface)  {
        
        todo!();
        /*
            try
        {
            SymbolListVisitor visitor (results);
            term->visitAllSymbols (visitor, scope, 0);
        }
        catch (EvaluationError&)
        {}
        */
    }
    
    /**
      | Returns a string version of the expression.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return term->toString();
        */
    }
    
    /**
      | Returns true if this expression contains
      | any symbols.
      |
      */
    pub fn uses_any_symbols(&self) -> bool {
        
        todo!();
        /*
            return containsAnySymbols (*term);
        */
    }
    
    /**
      | Returns the type of this expression.
      |
      */
    pub fn get_type(&self) -> ExpressionType {
        
        todo!();
        /*
            return term->getType();
        */
    }
    
    /**
      | If this expression is a symbol, function
      | or operator, this returns its identifier.
      |
      */
    pub fn get_symbol_or_function(&self) -> String {
        
        todo!();
        /*
            return term->getName();
        */
    }
    
    /**
      | Returns the number of inputs to this
      | expression. @see getInput
      |
      */
    pub fn get_num_inputs(&self) -> i32 {
        
        todo!();
        /*
            return term->getNumInputs();
        */
    }
    
    /**
      | Retrieves one of the inputs to this expression.
      | @see getNumInputs
      |
      */
    pub fn get_input(&self, index: i32) -> Expression {
        
        todo!();
        /*
            return Expression (term->getInput (index));
        */
    }
}
