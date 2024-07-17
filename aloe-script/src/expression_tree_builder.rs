crate::ix!();

#[no_copy]
#[leak_detector]
pub struct ExpressionTreeBuilder {
    base: TokenIterator,
}

impl ExpressionTreeBuilder {

    pub fn new(code: String) -> Self {
    
        todo!();
        /*
        : token_iterator(code),

        
        */
    }
    
    pub fn parse_statement_list(&mut self) -> *mut BlockStatement {
        
        todo!();
        /*
            std::unique_ptr<BlockStatement> b (new BlockStatement (location));

            while (currentType != TokenTypes::closeBrace && currentType != TokenTypes::eof)
                b->statements.add (parseStatement());

            return b.release();
        */
    }
    
    pub fn parse_function_params_and_body(&mut self, fo: &mut FunctionObject)  {
        
        todo!();
        /*
            match (TokenTypes::openParen);

            while (currentType != TokenTypes::closeParen)
            {
                auto paramName = currentValue.toString();
                match (TokenTypes::identifier);
                fo.parameters.add (paramName);

                if (currentType != TokenTypes::closeParen)
                    match (TokenTypes::comma);
            }

            match (TokenTypes::closeParen);
            fo.body.reset (parseBlock());
        */
    }
    
    pub fn parse_expression(&mut self) -> *mut Expression {
        
        todo!();
        /*
            ExpPtr lhs (parseLogicOperator());

            if (matchIf (TokenTypes::question))          return parseTernaryOperator (lhs);
            if (matchIf (TokenTypes::assign))            { ExpPtr rhs (parseExpression()); return new Assignment (location, lhs, rhs); }
            if (matchIf (TokenTypes::plusEquals))        return parseInPlaceOpExpression<AdditionOp> (lhs);
            if (matchIf (TokenTypes::minusEquals))       return parseInPlaceOpExpression<SubtractionOp> (lhs);
            if (matchIf (TokenTypes::timesEquals))       return parseInPlaceOpExpression<MultiplyOp> (lhs);
            if (matchIf (TokenTypes::divideEquals))      return parseInPlaceOpExpression<DivideOp> (lhs);
            if (matchIf (TokenTypes::moduloEquals))      return parseInPlaceOpExpression<ModuloOp> (lhs);
            if (matchIf (TokenTypes::leftShiftEquals))   return parseInPlaceOpExpression<LeftShiftOp> (lhs);
            if (matchIf (TokenTypes::rightShiftEquals))  return parseInPlaceOpExpression<RightShiftOp> (lhs);

            return lhs.release();
        */
    }
    
    pub fn throw_error(&self, err: &String)  {
        
        todo!();
        /*
            location.throwError (err);
        */
    }
    
    pub fn parse_in_place_op_expression<OpType>(&mut self, lhs: &mut ExpPtr) -> *mut Expression {
    
        todo!();
        /*
            ExpPtr rhs (parseExpression());
                Expression* bareLHS = lhs.get(); // careful - bare pointer is deliberately aliased
                return new SelfAssignment (location, bareLHS, new OpType (location, lhs, rhs));
        */
    }
    
    pub fn parse_block(&mut self) -> *mut BlockStatement {
        
        todo!();
        /*
            match (TokenTypes::openBrace);
            std::unique_ptr<BlockStatement> b (parseStatementList());
            match (TokenTypes::closeBrace);
            return b.release();
        */
    }
    
    pub fn parse_statement(&mut self) -> *mut Statement {
        
        todo!();
        /*
            if (currentType == TokenTypes::openBrace)   return parseBlock();
            if (matchIf (TokenTypes::Var))              return parseVar();
            if (matchIf (TokenTypes::if_))              return parseIf();
            if (matchIf (TokenTypes::while_))           return parseDoOrWhileLoop (false);
            if (matchIf (TokenTypes::do_))              return parseDoOrWhileLoop (true);
            if (matchIf (TokenTypes::for_))             return parseForLoop();
            if (matchIf (TokenTypes::return_))          return parseReturn();
            if (matchIf (TokenTypes::break_))           return new BreakStatement (location);
            if (matchIf (TokenTypes::continue_))        return new ContinueStatement (location);
            if (matchIf (TokenTypes::function))         return parseFunction();
            if (matchIf (TokenTypes::semicolon))        return new Statement (location);
            if (matchIf (TokenTypes::plusplus))         return parsePreIncDec<AdditionOp>();
            if (matchIf (TokenTypes::minusminus))       return parsePreIncDec<SubtractionOp>();

            if (matchesAny (TokenTypes::openParen, TokenTypes::openBracket))
                return matchEndOfStatement (parseFactor());

            if (matchesAny (TokenTypes::identifier, TokenTypes::literal, TokenTypes::minus))
                return matchEndOfStatement (parseExpression());

            throwError ("Found " + getTokenName (currentType) + " when expecting a statement");
            return nullptr;
        */
    }
    
    pub fn match_end_of_statement(&mut self, ex: *mut Expression) -> *mut Expression {
        
        todo!();
        /*
            ExpPtr e (ex); if (currentType != TokenTypes::eof) match (TokenTypes::semicolon); return e.release();
        */
    }
    
    pub fn match_close_paren(&mut self, ex: *mut Expression) -> *mut Expression {
        
        todo!();
        /*
            ExpPtr e (ex); match (TokenTypes::closeParen); return e.release();
        */
    }
    
    pub fn parse_if(&mut self) -> *mut Statement {
        
        todo!();
        /*
            std::unique_ptr<IfStatement> s (new IfStatement (location));
            match (TokenTypes::openParen);
            s->condition.reset (parseExpression());
            match (TokenTypes::closeParen);
            s->trueBranch.reset (parseStatement());
            s->falseBranch.reset (matchIf (TokenTypes::else_) ? parseStatement() : new Statement (location));
            return s.release();
        */
    }
    
    pub fn parse_return(&mut self) -> *mut Statement {
        
        todo!();
        /*
            if (matchIf (TokenTypes::semicolon))
                return new ReturnStatement (location, new Expression (location));

            auto* r = new ReturnStatement (location, parseExpression());
            matchIf (TokenTypes::semicolon);
            return r;
        */
    }
    
    pub fn parse_var(&mut self) -> *mut Statement {
        
        todo!();
        /*
            std::unique_ptr<VarStatement> s (new VarStatement (location));
            s->name = parseIdentifier();
            s->initialiser.reset (matchIf (TokenTypes::assign) ? parseExpression() : new Expression (location));

            if (matchIf (TokenTypes::comma))
            {
                std::unique_ptr<BlockStatement> block (new BlockStatement (location));
                block->statements.add (std::move (s));
                block->statements.add (parseVar());
                return block.release();
            }

            match (TokenTypes::semicolon);
            return s.release();
        */
    }
    
    pub fn parse_function(&mut self) -> *mut Statement {
        
        todo!();
        /*
            Identifier name;
            auto fn = parseFunctionDefinition (name);

            if (name.isNull())
                throwError ("Functions defined at statement-level must have a name");

            ExpPtr nm (new UnqualifiedName (location, name)), value (new LiteralValue (location, fn));
            return new Assignment (location, nm, value);
        */
    }
    
    pub fn parse_for_loop(&mut self) -> *mut Statement {
        
        todo!();
        /*
            std::unique_ptr<LoopStatement> s (new LoopStatement (location, false));
            match (TokenTypes::openParen);
            s->initialiser.reset (parseStatement());

            if (matchIf (TokenTypes::semicolon))
                s->condition.reset (new LiteralValue (location, true));
            else
            {
                s->condition.reset (parseExpression());
                match (TokenTypes::semicolon);
            }

            if (matchIf (TokenTypes::closeParen))
                s->iterator.reset (new Statement (location));
            else
            {
                s->iterator.reset (parseExpression());
                match (TokenTypes::closeParen);
            }

            s->body.reset (parseStatement());
            return s.release();
        */
    }
    
    pub fn parse_do_or_while_loop(&mut self, is_do_loop: bool) -> *mut Statement {
        
        todo!();
        /*
            std::unique_ptr<LoopStatement> s (new LoopStatement (location, isDoLoop));
            s->initialiser.reset (new Statement (location));
            s->iterator.reset (new Statement (location));

            if (isDoLoop)
            {
                s->body.reset (parseBlock());
                match (TokenTypes::while_);
            }

            match (TokenTypes::openParen);
            s->condition.reset (parseExpression());
            match (TokenTypes::closeParen);

            if (! isDoLoop)
                s->body.reset (parseStatement());

            return s.release();
        */
    }
    
    pub fn parse_identifier(&mut self) -> Identifier {
        
        todo!();
        /*
            Identifier i;
            if (currentType == TokenTypes::identifier)
                i = currentValue.toString();

            match (TokenTypes::identifier);
            return i;
        */
    }
    
    pub fn parse_function_definition(&mut self, function_name: &mut Identifier) -> Var {
        
        todo!();
        /*
            auto functionStart = location.location;

            if (currentType == TokenTypes::identifier)
                functionName = parseIdentifier();

            std::unique_ptr<FunctionObject> fo (new FunctionObject());
            parseFunctionParamsAndBody (*fo);
            fo->functionCode = String (functionStart, location.location);
            return Var (fo.release());
        */
    }
    
    pub fn parse_function_call(&mut self, 
        call:     *mut FunctionCall,
        function: &mut ExpPtr) -> *mut Expression {
        
        todo!();
        /*
            std::unique_ptr<FunctionCall> s (call);
            s->object.reset (function.release());
            match (TokenTypes::openParen);

            while (currentType != TokenTypes::closeParen)
            {
                s->arguments.add (parseExpression());
                if (currentType != TokenTypes::closeParen)
                    match (TokenTypes::comma);
            }

            return matchCloseParen (s.release());
        */
    }
    
    pub fn parse_suffixes(&mut self, e: *mut Expression) -> *mut Expression {
        
        todo!();
        /*
            ExpPtr input (e);

            if (matchIf (TokenTypes::dot))
                return parseSuffixes (new DotOperator (location, input, parseIdentifier()));

            if (currentType == TokenTypes::openParen)
                return parseSuffixes (parseFunctionCall (new FunctionCall (location), input));

            if (matchIf (TokenTypes::openBracket))
            {
                std::unique_ptr<ArraySubscript> s (new ArraySubscript (location));
                s->object.reset (input.release());
                s->index.reset (parseExpression());
                match (TokenTypes::closeBracket);
                return parseSuffixes (s.release());
            }

            if (matchIf (TokenTypes::plusplus))   return parsePostIncDec<AdditionOp> (input);
            if (matchIf (TokenTypes::minusminus)) return parsePostIncDec<SubtractionOp> (input);

            return input.release();
        */
    }
    
    pub fn parse_factor(&mut self) -> *mut Expression {
        
        todo!();
        /*
            if (currentType == TokenTypes::identifier)  return parseSuffixes (new UnqualifiedName (location, parseIdentifier()));
            if (matchIf (TokenTypes::openParen))        return parseSuffixes (matchCloseParen (parseExpression()));
            if (matchIf (TokenTypes::true_))            return parseSuffixes (new LiteralValue (location, (int) 1));
            if (matchIf (TokenTypes::false_))           return parseSuffixes (new LiteralValue (location, (int) 0));
            if (matchIf (TokenTypes::null_))            return parseSuffixes (new LiteralValue (location, Var()));
            if (matchIf (TokenTypes::undefined))        return parseSuffixes (new Expression (location));

            if (currentType == TokenTypes::literal)
            {
                Var v (currentValue); skip();
                return parseSuffixes (new LiteralValue (location, v));
            }

            if (matchIf (TokenTypes::openBrace))
            {
                std::unique_ptr<ObjectDeclaration> e (new ObjectDeclaration (location));

                while (currentType != TokenTypes::closeBrace)
                {
                    auto memberName = currentValue.toString();
                    match ((currentType == TokenTypes::literal && currentValue.isString())
                        ? TokenTypes::literal : TokenTypes::identifier);
                    match (TokenTypes::colon);

                    e->names.add (memberName);
                    e->initialisers.add (parseExpression());

                    if (currentType != TokenTypes::closeBrace)
                        match (TokenTypes::comma);
                }

                match (TokenTypes::closeBrace);
                return parseSuffixes (e.release());
            }

            if (matchIf (TokenTypes::openBracket))
            {
                std::unique_ptr<ArrayDeclaration> e (new ArrayDeclaration (location));

                while (currentType != TokenTypes::closeBracket)
                {
                    e->values.add (parseExpression());

                    if (currentType != TokenTypes::closeBracket)
                        match (TokenTypes::comma);
                }

                match (TokenTypes::closeBracket);
                return parseSuffixes (e.release());
            }

            if (matchIf (TokenTypes::function))
            {
                Identifier name;
                Var fn = parseFunctionDefinition (name);

                if (name.isValid())
                    throwError ("Inline functions definitions cannot have a name");

                return new LiteralValue (location, fn);
            }

            if (matchIf (TokenTypes::new_))
            {
                ExpPtr name (new UnqualifiedName (location, parseIdentifier()));

                while (matchIf (TokenTypes::dot))
                    name.reset (new DotOperator (location, name, parseIdentifier()));

                return parseFunctionCall (new NewOperator (location), name);
            }

            throwError ("Found " + getTokenName (currentType) + " when expecting an expression");
            return nullptr;
        */
    }
    
    pub fn parse_pre_inc_dec<OpType>(&mut self) -> *mut Expression {
    
        todo!();
        /*
            Expression* e = parseFactor(); // careful - bare pointer is deliberately aliased
                ExpPtr lhs (e), one (new LiteralValue (location, (int) 1));
                return new SelfAssignment (location, e, new OpType (location, lhs, one));
        */
    }
    
    pub fn parse_post_inc_dec<OpType>(&mut self, lhs: &mut ExpPtr) -> *mut Expression {
    
        todo!();
        /*
            Expression* e = lhs.release(); // careful - bare pointer is deliberately aliased
                ExpPtr lhs2 (e), one (new LiteralValue (location, (int) 1));
                return new PostAssignment (location, e, new OpType (location, lhs2, one));
        */
    }
    
    pub fn parse_typeof(&mut self) -> *mut Expression {
        
        todo!();
        /*
            std::unique_ptr<FunctionCall> f (new FunctionCall (location));
            f->object.reset (new UnqualifiedName (location, "typeof"));
            f->arguments.add (parseUnary());
            return f.release();
        */
    }
    
    pub fn parse_unary(&mut self) -> *mut Expression {
        
        todo!();
        /*
            if (matchIf (TokenTypes::minus))       { ExpPtr a (new LiteralValue (location, (int) 0)), b (parseUnary()); return new SubtractionOp   (location, a, b); }
            if (matchIf (TokenTypes::logicalNot))  { ExpPtr a (new LiteralValue (location, (int) 0)), b (parseUnary()); return new EqualsOp        (location, a, b); }
            if (matchIf (TokenTypes::plusplus))    return parsePreIncDec<AdditionOp>();
            if (matchIf (TokenTypes::minusminus))  return parsePreIncDec<SubtractionOp>();
            if (matchIf (TokenTypes::typeof_))     return parseTypeof();

            return parseFactor();
        */
    }
    
    pub fn parse_multiply_divide(&mut self) -> *mut Expression {
        
        todo!();
        /*
            ExpPtr a (parseUnary());

            for (;;)
            {
                if (matchIf (TokenTypes::times))        { ExpPtr b (parseUnary()); a.reset (new MultiplyOp (location, a, b)); }
                else if (matchIf (TokenTypes::divide))  { ExpPtr b (parseUnary()); a.reset (new DivideOp   (location, a, b)); }
                else if (matchIf (TokenTypes::modulo))  { ExpPtr b (parseUnary()); a.reset (new ModuloOp   (location, a, b)); }
                else break;
            }

            return a.release();
        */
    }
    
    pub fn parse_addition_subtraction(&mut self) -> *mut Expression {
        
        todo!();
        /*
            ExpPtr a (parseMultiplyDivide());

            for (;;)
            {
                if (matchIf (TokenTypes::plus))            { ExpPtr b (parseMultiplyDivide()); a.reset (new AdditionOp    (location, a, b)); }
                else if (matchIf (TokenTypes::minus))      { ExpPtr b (parseMultiplyDivide()); a.reset (new SubtractionOp (location, a, b)); }
                else break;
            }

            return a.release();
        */
    }
    
    pub fn parse_shift_operator(&mut self) -> *mut Expression {
        
        todo!();
        /*
            ExpPtr a (parseAdditionSubtraction());

            for (;;)
            {
                if (matchIf (TokenTypes::leftShift))                { ExpPtr b (parseExpression()); a.reset (new LeftShiftOp          (location, a, b)); }
                else if (matchIf (TokenTypes::rightShift))          { ExpPtr b (parseExpression()); a.reset (new RightShiftOp         (location, a, b)); }
                else if (matchIf (TokenTypes::rightShiftUnsigned))  { ExpPtr b (parseExpression()); a.reset (new RightShiftUnsignedOp (location, a, b)); }
                else break;
            }

            return a.release();
        */
    }
    
    pub fn parse_comparator(&mut self) -> *mut Expression {
        
        todo!();
        /*
            ExpPtr a (parseShiftOperator());

            for (;;)
            {
                if (matchIf (TokenTypes::equals))                  { ExpPtr b (parseShiftOperator()); a.reset (new EqualsOp             (location, a, b)); }
                else if (matchIf (TokenTypes::notEquals))          { ExpPtr b (parseShiftOperator()); a.reset (new NotEqualsOp          (location, a, b)); }
                else if (matchIf (TokenTypes::typeEquals))         { ExpPtr b (parseShiftOperator()); a.reset (new TypeEqualsOp         (location, a, b)); }
                else if (matchIf (TokenTypes::typeNotEquals))      { ExpPtr b (parseShiftOperator()); a.reset (new TypeNotEqualsOp      (location, a, b)); }
                else if (matchIf (TokenTypes::lessThan))           { ExpPtr b (parseShiftOperator()); a.reset (new LessThanOp           (location, a, b)); }
                else if (matchIf (TokenTypes::lessThanOrEqual))    { ExpPtr b (parseShiftOperator()); a.reset (new LessThanOrEqualOp    (location, a, b)); }
                else if (matchIf (TokenTypes::greaterThan))        { ExpPtr b (parseShiftOperator()); a.reset (new GreaterThanOp        (location, a, b)); }
                else if (matchIf (TokenTypes::greaterThanOrEqual)) { ExpPtr b (parseShiftOperator()); a.reset (new GreaterThanOrEqualOp (location, a, b)); }
                else break;
            }

            return a.release();
        */
    }
    
    pub fn parse_logic_operator(&mut self) -> *mut Expression {
        
        todo!();
        /*
            ExpPtr a (parseComparator());

            for (;;)
            {
                if (matchIf (TokenTypes::logicalAnd))       { ExpPtr b (parseComparator()); a.reset (new LogicalAndOp (location, a, b)); }
                else if (matchIf (TokenTypes::logicalOr))   { ExpPtr b (parseComparator()); a.reset (new LogicalOrOp  (location, a, b)); }
                else if (matchIf (TokenTypes::bitwiseAnd))  { ExpPtr b (parseComparator()); a.reset (new BitwiseAndOp (location, a, b)); }
                else if (matchIf (TokenTypes::bitwiseOr))   { ExpPtr b (parseComparator()); a.reset (new BitwiseOrOp  (location, a, b)); }
                else if (matchIf (TokenTypes::bitwiseXor))  { ExpPtr b (parseComparator()); a.reset (new BitwiseXorOp (location, a, b)); }
                else break;
            }

            return a.release();
        */
    }
    
    
    pub fn parse_ternary_operator(&mut self, condition: &mut ExpPtr) -> *mut Expression {
        
        todo!();
        /*
            std::unique_ptr<ConditionalOp> e (new ConditionalOp (location));
            e->condition.reset (condition.release());
            e->trueBranch.reset (parseExpression());
            match (TokenTypes::colon);
            e->falseBranch.reset (parseExpression());
            return e.release();
        */
    }
}
