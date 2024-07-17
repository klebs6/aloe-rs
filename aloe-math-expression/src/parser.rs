crate::ix!();

#[no_copy]
pub struct Parser<'a> {
    error: String,
    text:  &'a mut CharPointerType,
}

impl<'a> Parser<'a> {

    pub fn new(string_to_parse: &mut CharPointerType) -> Self {
    
        todo!();
        /*
        : text(stringToParse),

        
        */
    }
    
    pub fn read_up_to_comma(&mut self) -> TermPtr {
        
        todo!();
        /*
            if (text.isEmpty())
                return *new Constant (0.0, false);

            auto e = readExpression();

            if (e == nullptr || ((! readOperator (",")) && ! text.isEmpty()))
                return parseError ("Syntax error: \"" + String (text) + "\"");

            return e;
        */
    }
    
    pub fn parse_error(&mut self, message: &String) -> TermPtr {
        
        todo!();
        /*
            if (error.isEmpty())
                error = message;

            return {};
        */
    }
    
    pub fn is_decimal_digit(c: wchar_t) -> bool {
        
        todo!();
        /*
            return c >= '0' && c <= '9';
        */
    }
    
    pub fn read_char(&mut self, required: wchar_t) -> bool {
        
        todo!();
        /*
            if (*text == required)
            {
                ++text;
                return true;
            }

            return false;
        */
    }
    
    pub fn read_operator(
        &mut self, 
        ops:     *const u8,
        op_type: *mut u8

    ) -> bool {

        todo!();
        /*
            text.incrementToEndOfWhitespace();

            while (*ops != 0)
            {
                if (readChar ((aloe_wchar) (uint8) *ops))
                {
                    if (opType != nullptr)
                        *opType = *ops;

                    return true;
                }

                ++ops;
            }

            return false;
        */
    }
    
    pub fn read_identifier(&mut self, identifier: &mut String) -> bool {
        
        todo!();
        /*
            text.incrementToEndOfWhitespace();
            auto t = text;
            int numChars = 0;

            if (t.isLetter() || *t == '_')
            {
                ++t;
                ++numChars;

                while (t.isLetterOrDigit() || *t == '_')
                {
                    ++t;
                    ++numChars;
                }
            }

            if (numChars > 0)
            {
                identifier = String (text, (size_t) numChars);
                text = t;
                return true;
            }

            return false;
        */
    }
    
    pub fn read_number(&mut self) -> *mut Term {
        
        todo!();
        /*
            text.incrementToEndOfWhitespace();
            auto t = text;
            bool isResolutionTarget = (*t == '@');

            if (isResolutionTarget)
            {
                ++t;
                t.incrementToEndOfWhitespace();
                text = t;
            }

            if (*t == '-')
            {
                ++t;
                t.incrementToEndOfWhitespace();
            }

            if (isDecimalDigit (*t) || (*t == '.' && isDecimalDigit (t[1])))
                return new Constant (CharacterFunctions::readDoubleValue (text), isResolutionTarget);

            return nullptr;
        */
    }
    
    pub fn read_expression(&mut self) -> TermPtr {
        
        todo!();
        /*
            auto lhs = readMultiplyOrDivideExpression();
            char opType;

            while (lhs != nullptr && readOperator ("+-", &opType))
            {
                auto rhs = readMultiplyOrDivideExpression();

                if (rhs == nullptr)
                    return parseError ("Expected expression after \"" + String::charToString ((aloe_wchar) (uint8) opType) + "\"");

                if (opType == '+')
                    lhs = *new Add (lhs, rhs);
                else
                    lhs = *new Subtract (lhs, rhs);
            }

            return lhs;
        */
    }
    
    pub fn read_multiply_or_divide_expression(&mut self) -> TermPtr {
        
        todo!();
        /*
            auto lhs = readUnaryExpression();
            char opType;

            // TODO fix the star <space> slash symbol in the quotes -- need to remove the <space>
            //
            // this was done to allow block commenting
            //
            while (lhs != nullptr && readOperator ("* /", &opType))
            {
                TermPtr rhs (readUnaryExpression());

                if (rhs == nullptr)
                    return parseError ("Expected expression after \"" + String::charToString ((aloe_wchar) (uint8) opType) + "\"");

                if (opType == '*')
                    lhs = *new Multiply (lhs, rhs);
                else
                    lhs = *new Divide (lhs, rhs);
            }

            return lhs;
        */
    }
    
    pub fn read_unary_expression(&mut self) -> TermPtr {
        
        todo!();
        /*
            char opType;
            if (readOperator ("+-", &opType))
            {
                TermPtr e (readUnaryExpression());

                if (e == nullptr)
                    return parseError ("Expected expression after \"" + String::charToString ((aloe_wchar) (uint8) opType) + "\"");

                if (opType == '-')
                    e = e->negated();

                return e;
            }

            return readPrimaryExpression();
        */
    }
    
    pub fn read_primary_expression(&mut self) -> TermPtr {
        
        todo!();
        /*
            if (auto e = readParenthesisedExpression())
                return e;

            if (auto e = readNumber())
                return e;

            return readSymbolOrFunction();
        */
    }
    
    pub fn read_symbol_or_function(&mut self) -> TermPtr {
        
        todo!();
        /*
            String identifier;

            if (readIdentifier (identifier))
            {
                if (readOperator ("(")) // method call...
                {
                    auto f = new Function (identifier);
                    std::unique_ptr<Term> func (f);  // (can't use std::unique_ptr<Function> in MSVC)

                    auto param = readExpression();

                    if (param == nullptr)
                    {
                        if (readOperator (")"))
                            return TermPtr (func.release());

                        return parseError ("Expected parameters after \"" + identifier + " (\"");
                    }

                    f->parameters.add (Expression (param.get()));

                    while (readOperator (","))
                    {
                        param = readExpression();

                        if (param == nullptr)
                            return parseError ("Expected expression after \",\"");

                        f->parameters.add (Expression (param.get()));
                    }

                    if (readOperator (")"))
                        return TermPtr (func.release());

                    return parseError ("Expected \")\"");
                }

                if (readOperator ("."))
                {
                    TermPtr rhs (readSymbolOrFunction());

                    if (rhs == nullptr)
                        return parseError ("Expected symbol or function after \".\"");

                    if (identifier == "this")
                        return rhs;

                    return *new DotOperator (new SymbolTerm (identifier), rhs);
                }

                // just a symbol..
                jassert (identifier.trim() == identifier);
                return *new SymbolTerm (identifier);
            }

            return {};
        */
    }
    
    pub fn read_parenthesised_expression(&mut self) -> TermPtr {
        
        todo!();
        /*
            if (! readOperator ("("))
                return {};

            auto e = readExpression();

            if (e == nullptr || ! readOperator (")"))
                return {};

            return e;
        */
    }
}
