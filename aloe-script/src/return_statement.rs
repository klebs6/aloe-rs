crate::ix!();

pub struct ReturnStatement {
    base:         Statement,
    return_value: ExpPtr,
}

impl StatementInterface for ReturnStatement {

    fn perform(&self, 
        s:   &Scope,
        ret: *mut Var) -> StatementResultCode {
        
        todo!();
        /*
            if (ret != nullptr)  *ret = returnValue->getResult (s);
                        return returnWasHit;
        */
    }
}

impl ReturnStatement {

    pub fn new(
        l: &CodeLocation,
        v: *mut Expression) -> Self {
    
        todo!();
        /*
        : statement(l),
        : return_value(v),

        
        */
    }
}

