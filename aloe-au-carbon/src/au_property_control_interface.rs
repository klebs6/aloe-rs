crate::ix!();

pub trait AUPropertyControlInterface {

    fn handle_event(&mut self, 
        in_handler_ref: EventHandlerCallRef,
        event:          EventRef) -> bool;

    fn handle_property_change(&mut self, in_prop: &AudioUnitProperty) -> bool;

    fn add_interest(&mut self, 
        in_listener: AUEventListenerRef,
        in_object:   *mut c_void);

    fn remove_interest(&mut self, 
        in_listener: AUEventListenerRef,
        in_object:   *mut c_void);

    fn handle_control_change(&mut self);
}
