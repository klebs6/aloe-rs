crate::ix!();

lazy_static!{
    /*
    static AUCarbonViewControl* mLastControl;
    */
}

pub trait AUCarbonViewControlInterface {

    fn bind(&mut self);

    fn control_to_parameter(&mut self);

    fn parameter_to_control(&mut self, new_value: f32);

    fn set_value_fract(&mut self, value: f64);

    fn get_value_fract(&mut self) -> f64;

    fn set_text_value(&mut self, str_: CFStringRef);

    fn get_text_value(&mut self) -> CFStringRef;

    fn set_value(&mut self, value: i64);

    fn get_value(&mut self) -> i64;

    fn handle_event(
        &mut self, 
        in_handler_ref: EventHandlerCallRef,
        event:          EventRef
    ) -> bool;
}

impl AUCarbonViewControlInterface for AUCarbonViewControl {

    /**
      | second-stage construction
      |
      */
    fn bind(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    fn control_to_parameter(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    fn parameter_to_control(&mut self, new_value: f32)  {
        
        todo!();
        /*
        
        */
    }
    
    fn set_value_fract(&mut self, value: f64)  {
        
        todo!();
        /*
        
        */
    }
    
    fn get_value_fract(&mut self) -> f64 {
        
        todo!();
        /*
        
        */
    }
    
    fn set_text_value(&mut self, str_: CFStringRef)  {
        
        todo!();
        /*
        
        */
    }
    
    fn get_text_value(&mut self) -> CFStringRef {
        
        todo!();
        /*
        
        */
    }
    
    fn set_value(&mut self, value: i64)  {
        
        todo!();
        /*
        
        */
    }
    
    fn get_value(&mut self) -> i64 {
        
        todo!();
        /*
        
        */
    }

    /**
      | CarbonEventHandler overrides
      |
      */
    fn handle_event(&mut self, 
        in_handler_ref: EventHandlerCallRef,
        event:          EventRef) -> bool {
        
        todo!();
        /*
        
        */
    }
}
