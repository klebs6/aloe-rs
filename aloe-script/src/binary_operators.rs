crate::ix!();

pub struct EqualsOp {
    base: BinaryOperator,
}

impl EqualsOp {

    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::equals),

        
        */
    }
    
    pub fn get_with_undefined_arg(&self) -> Var {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_with_doubles(&self, a: f64, b: f64) -> Var {
        
        todo!();
        /*
            return a == b;
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a == b;
        */
    }
    
    pub fn get_with_strings(&self, 
        a: &String,
        b: &String) -> Var {
        
        todo!();
        /*
            return a == b;
        */
    }
    
    pub fn get_with_array_or_object(&self, a: &Var, b: &Var) -> Var {
        
        todo!();
        /*
            return a == b;
        */
    }
}

pub struct NotEqualsOp {
    base: BinaryOperator,
}

impl NotEqualsOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::notEquals),

        
        */
    }
    
    pub fn get_with_undefined_arg(&self) -> Var {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_with_doubles(&self, a: f64, b: f64) -> Var {
        
        todo!();
        /*
            return a != b;
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a != b;
        */
    }
    
    pub fn get_with_strings(&self, 
        a: &String,
        b: &String) -> Var {
        
        todo!();
        /*
            return a != b;
        */
    }
    
    pub fn get_with_array_or_object(&self, a: &Var, b: &Var) -> Var {
        
        todo!();
        /*
            return a != b;
        */
    }
}

pub struct LessThanOp {
    base: BinaryOperator,
}

impl LessThanOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::lessThan),

        
        */
    }
    
    pub fn get_with_doubles(&self, a: f64, b: f64) -> Var {
        
        todo!();
        /*
            return a < b;
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a < b;
        */
    }
    
    pub fn get_with_strings(&self, 
        a: &String,
        b: &String) -> Var {
        
        todo!();
        /*
            return a < b;
        */
    }
}

pub struct LessThanOrEqualOp {
    base: BinaryOperator,
}

impl LessThanOrEqualOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::lessThanOrEqual),

        
        */
    }
    
    pub fn get_with_doubles(&self, a: f64, b: f64) -> Var {
        
        todo!();
        /*
            return a <= b;
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a <= b;
        */
    }
    
    pub fn get_with_strings(&self, 
        a: &String,
        b: &String) -> Var {
        
        todo!();
        /*
            return a <= b;
        */
    }
}

pub struct GreaterThanOp {
    base: BinaryOperator,
}

impl GreaterThanOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::greaterThan),

        
        */
    }
    
    pub fn get_with_doubles(&self, a: f64, b: f64) -> Var {
        
        todo!();
        /*
            return a > b;
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a > b;
        */
    }
    
    pub fn get_with_strings(&self, 
        a: &String,
        b: &String) -> Var {
        
        todo!();
        /*
            return a > b;
        */
    }
}

pub struct GreaterThanOrEqualOp {
    base: BinaryOperator,
}

impl GreaterThanOrEqualOp {

    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::greaterThanOrEqual),

        
        */
    }
    
    pub fn get_with_doubles(&self, a: f64, b: f64) -> Var {
        
        todo!();
        /*
            return a >= b;
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a >= b;
        */
    }
    
    pub fn get_with_strings(&self, 
        a: &String,
        b: &String) -> Var {
        
        todo!();
        /*
            return a >= b;
        */
    }
}

pub struct AdditionOp {
    base: BinaryOperator,
}

impl AdditionOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::plus),

        
        */
    }
    
    pub fn get_with_doubles(&self, a: f64, b: f64) -> Var {
        
        todo!();
        /*
            return a + b;
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a + b;
        */
    }
    
    pub fn get_with_strings(&self, 
        a: &String,
        b: &String) -> Var {
        
        todo!();
        /*
            return a + b;
        */
    }
}

pub struct SubtractionOp {
    base: BinaryOperator,
}

impl SubtractionOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::minus),

        
        */
    }
    
    pub fn get_with_doubles(&self, a: f64, b: f64) -> Var {
        
        todo!();
        /*
            return a - b;
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a - b;
        */
    }
}

pub struct MultiplyOp {
    base: BinaryOperator,
}

impl MultiplyOp {

    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::times),

        
        */
    }
    
    pub fn get_with_doubles(&self, a: f64, b: f64) -> Var {
        
        todo!();
        /*
            return a * b;
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a * b;
        */
    }
}

pub struct DivideOp {
    base: BinaryOperator,
}

impl DivideOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::divide),

        
        */
    }
    
    pub fn get_with_doubles(&self, a: f64, b: f64) -> Var {
        
        todo!();
        /*
            return b != 0 ? a / b : std::numeric_limits<double>::infinity();
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return b != 0 ? Var ((double) a / (double) b) : Var (std::numeric_limits<double>::infinity());
        */
    }
}

pub struct ModuloOp {
    base: BinaryOperator,
}

impl ModuloOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::modulo),

        
        */
    }
    
    pub fn get_with_doubles(&self, a: f64, b: f64) -> Var {
        
        todo!();
        /*
            return b != 0 ? fmod (a, b) : std::numeric_limits<double>::infinity();
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return b != 0 ? Var (a % b) : Var (std::numeric_limits<double>::infinity());
        */
    }
}

pub struct BitwiseOrOp {
    base: BinaryOperator,
}

impl BitwiseOrOp {

    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::bitwiseOr),

        
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a | b;
        */
    }
}

pub struct BitwiseAndOp {
    base: BinaryOperator,
}

impl BitwiseAndOp {

    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::bitwiseAnd),

        
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a & b;
        */
    }
}

pub struct BitwiseXorOp {
    base: BinaryOperator,
}

impl BitwiseXorOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::bitwiseXor),

        
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return a ^ b;
        */
    }
}

pub struct LeftShiftOp {
    base: BinaryOperator,
}

impl LeftShiftOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::leftShift),

        
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return ((int) a) << (int) b;
        */
    }
}

pub struct RightShiftOp {
    base: BinaryOperator,
}

impl RightShiftOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::rightShift),

        
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return ((int) a) >> (int) b;
        */
    }
}

pub struct RightShiftUnsignedOp {
    base: BinaryOperator,
}

impl RightShiftUnsignedOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator(l, a, b, TokenTypes::rightShiftUnsigned),

        
        */
    }
    
    pub fn get_with_ints(&self, a: i64, b: i64) -> Var {
        
        todo!();
        /*
            return (int) (((uint32) a) >> (int) b);
        */
    }
}

pub struct LogicalAndOp {
    base: BinaryOperatorBase,
}

impl LogicalAndOp {

    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator_base(l, a, b, TokenTypes::logicalAnd),

        
        */
    }
    
    pub fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            return lhs->getResult (s) && rhs->getResult (s);
        */
    }
}

pub struct LogicalOrOp {
    base: BinaryOperatorBase,
}

impl LogicalOrOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator_base(l, a, b, TokenTypes::logicalOr),

        
        */
    }
    
    pub fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            return lhs->getResult (s) || rhs->getResult (s);
        */
    }
}

pub struct TypeEqualsOp {
    base: BinaryOperatorBase,
}

impl TypeEqualsOp {

    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator_base(l, a, b, TokenTypes::typeEquals),

        
        */
    }
    
    pub fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            return areTypeEqual (lhs->getResult (s), rhs->getResult (s));
        */
    }
}

pub struct TypeNotEqualsOp {
    base: BinaryOperatorBase,
}

impl TypeNotEqualsOp {
    
    pub fn new(
        l: &CodeLocation,
        a: &mut ExpPtr,
        b: &mut ExpPtr) -> Self {
    
        todo!();
        /*
        : binary_operator_base(l, a, b, TokenTypes::typeNotEquals),

        
        */
    }
    
    pub fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            return ! areTypeEqual (lhs->getResult (s), rhs->getResult (s));
        */
    }
}

pub struct ConditionalOp {
    base:         Expression,
    condition:    ExpPtr,
    true_branch:  ExpPtr,
    false_branch: ExpPtr,
}

impl ConditionalOp {
    
    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : expression(l),

        
        */
    }
    
    pub fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            return (condition->getResult (s) ? trueBranch : falseBranch)->getResult (s);
        */
    }
    
    pub fn assign(&self, 
        s: &Scope,
        v: &Var)  {
        
        todo!();
        /*
            (condition->getResult (s) ? trueBranch : falseBranch)->assign (s, v);
        */
    }
}
