crate::ix!();

pub trait BinaryOperatorInterface {

    fn get_with_undefined_arg(&self) -> Var {
        
        todo!();
        /*
            return Var::undefined();
        */
    }
    
    fn get_with_doubles(&self, _0: f64, _1: f64) -> Var {
        
        todo!();
        /*
            return throwError ("Double");
        */
    }
    
    fn get_with_ints(&self, _0: i64, _1: i64) -> Var {
        
        todo!();
        /*
            return throwError ("Integer");
        */
    }
    
    fn get_with_array_or_object(
        &self, 
        a:  &Var,
        _1: &Var

    ) -> Var {
        
        todo!();
        /*
            return throwError (a.isArray() ? "Vec" : "Object");
        */
    }
    
    fn get_with_strings(
        &self, 
        _0: &String,
        _1: &String

    ) -> Var {
        
        todo!();
        /*
            return throwError ("String");
        */
    }
}
