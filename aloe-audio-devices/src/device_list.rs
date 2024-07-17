crate::ix!();

pub fn add_if_not_null(
    list:   &mut Vec<Box<AudioIODeviceType>>,
    device: *mut AudioIODeviceType)  {

    todo!();
    /*
        if (device != nullptr)
            list.add (device);
    */
}

pub fn device_list_contains(
    ty:       *mut AudioIODeviceType,
    is_input: bool,
    name:     &String

) -> bool {

    todo!();
    /*
        for (auto& deviceName : type->getDeviceNames (isInput))
            if (deviceName.trim().equalsIgnoreCase (name.trim()))
                return true;

        return false;
    */
}

pub fn update_setup_channels(
    setup:            &mut AudioDeviceSetup,
    default_num_ins:  i32,
    default_num_outs: i32

) {

    todo!();
    /*
        auto updateChannels = [] (const String& deviceName, BigInteger& channels, int defaultNumChannels)
        {
            if (deviceName.isEmpty())
            {
                channels.clear();
            }
            else if (defaultNumChannels != -1)
            {
                channels.clear();
                channels.setRange (0, defaultNumChannels, true);
            }
        };

        updateChannels (setup.inputDeviceName,  setup.inputChannels,  setup.useDefaultInputChannels  ? defaultNumIns  : -1);
        updateChannels (setup.outputDeviceName, setup.outputChannels, setup.useDefaultOutputChannels ? defaultNumOuts : -1);
    */
}
