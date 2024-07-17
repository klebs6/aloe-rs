crate::ix!();

pub trait AUCarbonViewBaseInterface {

    /* ------------- AUViewBase overrides  ------------- */
    fn create_carbon_view(&mut self, 
        in_audio_unit:      AudioUnit,
        in_window:          WindowRef,
        in_parent_control:  ControlRef,
        in_location:        &Float32Point,
        in_size:            &Float32Point,
        out_parent_control: &mut ControlRef) -> OSStatus;

    /* ------------ our own virtual methods  ------------ */
    fn createui(&mut self, 
        in_xoffset: Float32,
        in_yoffset: Float32) -> OSStatus;
    
    fn handle_event(&mut self, 
        in_handler_ref: EventHandlerCallRef,
        event:          EventRef) -> bool;
    
    fn respond_to_event_timer(&mut self, in_timer: EventLoopTimerRef);
}
