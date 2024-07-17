crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/unit_tests/aloe_UnitTest.cpp]

/**
  | Checks that the result of an expression
  | does not throw an exception.
  |
  */
macro_rules! expectdoesnotthrow {
    ($expr:ident) => {
        /*
        
                try                                  
                {                                    
                    (expr);                          
                    expect (true);                   
                }                                    
                catch (...)                          
                {                                    
                    expect (false, "Expected: does not throw an exception, Actual: throws."); 
                }
        */
    }
}

/**
  | Checks that the result of an expression
  | throws an exception.
  |
  */
macro_rules! expectthrows {
    ($expr:ident) => {
        /*
        
            try                                  
            {                                    
                (expr);                          
                expect (false, "Expected: throws an exception, Actual: does not throw."); 
            }                                    
            catch (...)                          
            {                                    
                expect (true);                   
            }
        */
    }
}

/**
  | Checks that the result of an expression
  | throws an exception of a certain type.
  |
  */
macro_rules! expectthrowstype {
    ($expr:ident, $type:ident) => {
        /*
        
            try                                  
            {                                    
                (expr);                          
                expect (false, "Expected: throws an exception of type " #type ", Actual: does not throw."); 
            }                                    
            catch (type&)                        
            {                                    
                expect (true);                   
            }                                    
            catch (...)                          
            {                                    
                expect (false, "Expected: throws an exception of type " #type ", Actual: throws another type."); 
            }
        */
    }
}
