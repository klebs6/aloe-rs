crate::ix!();

pub trait SendParamChangeMessageToListeners {

    fn send_param_change_message_to_listeners(
        &mut self, 
        parameter_index: i32,
        new_value:       f32
    );
}
