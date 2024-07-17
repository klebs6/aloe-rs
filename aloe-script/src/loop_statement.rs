crate::ix!();

pub struct LoopStatement {
    base:        Statement,
    initialiser: Box<Statement>,
    iterator:    Box<Statement>,
    body:        Box<Statement>,
    condition:   ExpPtr,
    is_do_loop:  bool,
}

impl StatementInterface for LoopStatement {

    fn perform(
        &self, 
        s:              &Scope,
        returned_value: *mut Var

    ) -> StatementResultCode {
        
        todo!();
        /*
            initialiser->perform (s, nullptr);

                        while (isDoLoop || condition->getResult (s))
                            {
                                s.checkTimeOut (location);
                                auto r = body->perform (s, returnedValue);

                                if (r == returnWasHit)   return r;
                                if (r == breakWasHit)    break;

                                iterator->perform (s, nullptr);

                                if (isDoLoop && r != continueWasHit && ! condition->getResult (s))
                                    break;
                            }

                        return ok;
        */
    }
}

impl LoopStatement {

    pub fn new(
        l:     &CodeLocation,
        is_do: bool) -> Self {
    
        todo!();
        /*
        : statement(l),
        : is_do_loop(isDo),

        
        */
    }
}
