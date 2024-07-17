crate::ix!();

pub struct AudioDeviceSetupDetails<'a>
{
    manager:                 *mut AudioDeviceManager<'a>,
    min_num_input_channels:  i32,
    max_num_input_channels:  i32,
    min_num_output_channels: i32,
    max_num_output_channels: i32,
    use_stereo_pairs:        bool,
}

pub fn get_no_device_string() -> String {
    
    todo!();
    /*
        return "<< " + TRANS("none") + " >>";
    */
}
