crate::ix!();

pub trait AudioIODeviceTypeInterface {

    /**
      | Refreshes the object's cached list
      | of known devices.
      | 
      | This must be called at least once before
      | calling getDeviceNames() or any of
      | the other device creation methods.
      |
      */
    fn scan_for_devices(&mut self);

    /**
      | Returns the list of available devices
      | of this type.
      | 
      | The scanForDevices() method must have
      | been called to create this list.
      | 
      | -----------
      | @param wantInputNames
      | 
      | for devices which have separate inputs
      | and outputs this determines which list
      | of names is returned
      |
      */
    fn get_device_names(&self, want_input_names: bool) -> Vec<String>;

    /**
      | Returns the name of the default device.
      | 
      | This will be one of the names from the
      | getDeviceNames() list.
      | 
      | -----------
      | @param forInput
      | 
      | if true, this means that a default input
      | device should be returned; if false,
      | it should return the default output
      |
      */
    fn get_default_device_index(&self, for_input: bool) -> i32;

    /**
      | Returns the index of a given device in
      | the list of device names. If asInput
      | is true, it shows the index in the inputs
      | list, otherwise it looks for it in the
      | outputs list.
      |
      */
    fn get_index_of_device(&self, 
        device:   *mut AudioIODevice,
        as_input: bool) -> i32;

    /**
      | Returns true if two different devices
      | can be used for the input and output.
      |
      */
    fn has_separate_inputs_and_outputs(&self) -> bool;

    /**
      | Creates one of the devices of this type.
      | 
      | The deviceName must be one of the strings
      | returned by getDeviceNames(), and
      | scanForDevices() must have been called
      | before this method is used.
      |
      */
    fn create_device(&mut self, 
        output_device_name: &String,
        input_device_name:  &String) -> *mut AudioIODevice;

}
