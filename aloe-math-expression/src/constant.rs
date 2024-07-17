crate::ix!();

pub struct Constant {
    base:                 Term,
    value:                f64,
    is_resolution_target: bool,
}

impl Constant {
    
    pub fn new(
        val:               f64,
        resolution_target: bool) -> Self {
    
        todo!();
        /*
        : value(val),
        : is_resolution_target(resolutionTarget),

        
        */
    }
    
    pub fn get_type(&self) -> ExpressionType {
        
        todo!();
        /*
            return constantType;
        */
    }
    
    pub fn clone(&self) -> *mut Term {
        
        todo!();
        /*
            return new Constant (value, isResolutionTarget);
        */
    }
    
    pub fn resolve(
        &mut self, 
        _0: &dyn ExpressionScopeInterface,
        _1: i32

    ) -> TermPtr {
        
        todo!();
        /*
            return *this;
        */
    }
    
    pub fn to_double(&self) -> f64 {
        
        todo!();
        /*
            return value;
        */
    }
    
    pub fn negated(&mut self) -> TermPtr {
        
        todo!();
        /*
            return *new Constant (-value, isResolutionTarget);
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            String s (value);
            if (isResolutionTarget)
                s = "@" + s;

            return s;
        */
    }
}
