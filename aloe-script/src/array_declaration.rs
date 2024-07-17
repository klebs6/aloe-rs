crate::ix!();

pub struct ArrayDeclaration {
    base:   Expression,
    values: Vec<Box<Expression>>,
}

impl ArrayDeclaration {
    
    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : expression(l),

        
        */
    }
    
    pub fn get_result(&self, s: &Scope) -> Var {
        
        todo!();
        /*
            Vec<Var> a;

            for (int i = 0; i < values.size(); ++i)
                a.add (values.getUnchecked(i)->getResult (s));

            // std::move() needed here for older compilers
            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wredundant-move")
                return std::move (a);
            ALOE_END_IGNORE_WARNINGS_GCC_LIKE
        */
    }
}
