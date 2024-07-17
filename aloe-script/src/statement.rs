crate::ix!();

pub trait StatementInterface {

    fn perform(
        &self, 
        _0: &Scope,
        _1: *mut Var
    ) -> StatementResultCode 
    {
        StatementResultCode::ok
    }
}

pub enum StatementResultCode  { 
    ok = 0, 
    returnWasHit, 
    breakWasHit, 
    continueWasHit 
}

#[no_copy]
#[leak_detector]
pub struct Statement {
    location: CodeLocation,
}

impl Statement {

    pub fn new(l: &CodeLocation) -> Self {
    
        todo!();
        /*
        : location(l),

        
        */
    }
}
