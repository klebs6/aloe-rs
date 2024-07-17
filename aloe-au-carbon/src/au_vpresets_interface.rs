crate::ix!();

pub trait AUVPresetsInterface {

    fn handle_property_change(&mut self, in_prop: &AudioUnitProperty) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    fn add_interest(&mut self, 
        in_listener: AUEventListenerRef,
        in_object:   *mut c_void)  {
        
        todo!();
        /*
        
        */
    }
    
    fn remove_interest(&mut self, 
        in_listener: AUEventListenerRef,
        in_object:   *mut c_void)  {
        
        todo!();
        /*
        
        */
    }
    
    fn handle_control_change(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

