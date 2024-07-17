crate::ix!();

pub struct JavascriptEngineRootObject {
    base:    DynamicObject,
    timeout: Time,
}

impl Default for JavascriptEngineRootObject {
    
    fn default() -> Self {
        todo!();
        /*
            setMethod ("exec",       exec);
                setMethod ("eval",       eval);
                setMethod ("trace",      trace);
                setMethod ("charToInt",  charToInt);
                setMethod ("parseInt",   IntegerClass::parseInt);
                setMethod ("typeof",     typeof_internal);
                setMethod ("parseFloat", parseFloat)
        */
    }
}

impl JavascriptEngineRootObject {
    
    pub fn execute(&mut self, code: &String)  {
        
        todo!();
        /*
            ExpressionTreeBuilder tb (code);
                std::unique_ptr<BlockStatement> (tb.parseStatementList())->perform (Scope ({}, *this, *this), nullptr);
        */
    }
    
    pub fn evaluate(&mut self, code: &String) -> Var {
        
        todo!();
        /*
            ExpressionTreeBuilder tb (code);
                return ExpPtr (tb.parseExpression())->getResult (Scope ({}, *this, *this));
        */
    }
    
    pub fn are_type_equal(a: &Var, b: &Var) -> bool {
        
        todo!();
        /*
            return a.hasSameTypeAs (b) && isFunction (a) == isFunction (b)
                    && (((a.isUndefined() || a.isVoid()) && (b.isUndefined() || b.isVoid())) || a == b);
        */
    }
    
    pub fn get_token_name(t: TokenType) -> String {
        
        todo!();
        /*
            return t[0] == '$' ? String (t + 1) : ("'" + String (t) + "'");
        */
    }
    
    pub fn is_function(v: &Var) -> bool {
        
        todo!();
        /*
            return dynamic_cast<FunctionObject*> (v.getObject()) != nullptr;
        */
    }
    
    pub fn is_numeric(v: &Var) -> bool {
        
        todo!();
        /*
            return v.isInt() || v.isDouble() || v.isInt64() || v.isBool();
        */
    }
    
    pub fn is_numeric_or_undefined(v: &Var) -> bool {
        
        todo!();
        /*
            return isNumeric (v) || v.isUndefined();
        */
    }
    
    pub fn get_octal_value(s: &String) -> i64 {
        
        todo!();
        /*
            BigInteger b; b.parseString (s.initialSectionContainingOnly ("01234567"), 8); return b.toInt64();
        */
    }
    
    pub fn get_prototype_identifier() -> Identifier {
        
        todo!();
        /*
            static const Identifier i ("prototype"); return i;
        */
    }
    
    pub fn get_property_pointer(
        o: &mut DynamicObject,
        i: &Identifier

    ) -> *mut Var {
        
        todo!();
        /*
            return o.getProperties().getVarPointer (i);
        */
    }
    
    pub fn get(
        a:     Args,
        index: i32) -> Var {
        
        todo!();
        /*
            return index < a.numArguments ? a.arguments[index] : Var();
        */
    }
    
    pub fn is_int(
        a:     Args,
        index: i32) -> bool {
        
        todo!();
        /*
            return get (a, index).isInt() || get (a, index).isInt64();
        */
    }
    
    pub fn get_int(
        a:     Args,
        index: i32) -> i32 {
        
        todo!();
        /*
            return get (a, index);
        */
    }
    
    pub fn get_double(
        a:     Args,
        index: i32) -> f64 {
        
        todo!();
        /*
            return get (a, index);
        */
    }
    
    pub fn get_string(
        a:     Args,
        index: i32) -> String {
        
        todo!();
        /*
            return get (a, index).toString();
        */
    }
    
    pub fn trace(a: Args) -> Var {
        
        todo!();
        /*
            Logger::outputDebugString (JSON::toString (a.thisObject)); return Var::undefined();
        */
    }
    
    pub fn char_to_int(a: Args) -> Var {
        
        todo!();
        /*
            return (int) (getString (a, 0)[0]);
        */
    }
    
    pub fn parse_float(a: Args) -> Var {
        
        todo!();
        /*
            return getDouble (a, 0);
        */
    }
    
    pub fn typeof_internal(a: Args) -> Var {
        
        todo!();
        /*
            Var v (get (a, 0));

                if (v.isVoid())                      return "void";
                if (v.isString())                    return "string";
                if (isNumeric (v))                   return "number";
                if (isFunction (v) || v.isMethod())  return "function";
                if (v.isObject())                    return "object";

                return "undefined";
        */
    }
    
    pub fn exec(a: Args) -> Var {
        
        todo!();
        /*
            if (auto* root = dynamic_cast<JavascriptEngineRootObject*> (a.thisObject.getObject()))
                    root->execute (getString (a, 0));

                return Var::undefined();
        */
    }
    
    pub fn eval(a: Args) -> Var {
        
        todo!();
        /*
            if (auto* root = dynamic_cast<JavascriptEngineRootObject*> (a.thisObject.getObject()))
                    return root->evaluate (getString (a, 0));

                return Var::undefined();
        */
    }
}
