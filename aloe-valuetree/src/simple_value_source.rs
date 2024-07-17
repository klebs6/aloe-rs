crate::ix!();

#[no_copy]
#[leak_detector]
pub struct SimpleValueSource<'a> {
    base:  ValueSource<'a>,
    value: Var,
}

impl<'a> SimpleValueSource<'a> {

    pub fn new(initial_value: &Var) -> Self {
    
        todo!();
        /*
        : value(initialValue),
        */
    }
    
    pub fn get_value(&self) -> Var {
        
        todo!();
        /*
            return value;
        */
    }
    
    pub fn set_value(&mut self, new_value: &Var)  {
        
        todo!();
        /*
            if (! newValue.equalsWithSameType (value))
            {
                value = newValue;
                sendChangeMessage (false);
            }
        */
    }
}
