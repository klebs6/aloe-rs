crate::ix!();

/**
  | An exception that can be thrown by Expression::evaluate().
  |
  */
#[derive(Debug,Error)]
pub enum EvaluationError {

    #[error(non_std,no_from)] 
    Default {
        description: String,
    }
}

impl EvaluationError {
    
    pub fn new(desc: &String) -> Self {
    
        todo!();
        /*
        : description(desc),

            DBG ("Expression::EvaluationError: " + description);
        */
    }
}
