crate::ix!();

pub struct IOSAudioIODeviceImplIOChannelDataIOChannelConfig {
    hardware_channel_names:   Vec<String>,
    num_hardware_channels:    i32,
    are_channels_accessible:  bool,
    active_channels:          BigInteger,
    num_active_channels:      i32,
    active_channel_indices:   Vec<i32>,
    inactive_channel_indices: Vec<i32>,
}

impl IOSAudioIODeviceImplIOChannelDataIOChannelConfig {

    pub fn new(
        is_input:          bool,
        required_channels: BigInteger) -> Self {
    
        todo!();
        /*


            : hardwareChannelNames (getHardwareChannelNames (isInput)),
                    numHardwareChannels (hardwareChannelNames.size()),
                    areChannelsAccessible ((! isInput) || [AVAudioSession sharedInstance].isInputAvailable),
                    activeChannels (limitRequiredChannelsToHardware (numHardwareChannels, requiredChannels)),
                    numActiveChannels (activeChannels.countNumberOfSetBits()),
                    activeChannelIndices (getActiveChannelIndices (activeChannels)),
                    inactiveChannelIndices (getInactiveChannelIndices (activeChannelIndices, numHardwareChannels))

                    #if ALOE_IOS_AUDIO_LOGGING
                    {
                        String info;

                        info << "Number of hardware channels: " << numHardwareChannels
                            << ", Hardware channel names:";

                        for (auto& name : hardwareChannelNames)
                            info << " \"" << name << "\"";

                        info << ", Are channels available: " << (areChannelsAccessible ? "yes" : "no")
                            << ", Active channel indices:";

                        for (auto i : activeChannelIndices)
                            info << " " << i;

                        info << ", Inactive channel indices:";

                        for (auto i : inactiveChannelIndices)
                            info << " " << i;

                        ALOE_IOS_AUDIO_LOG ((isInput ? "Input" : "Output") << " channel configuration: {" << info << "}");
                    }
                    #endif
        */
    }
    
    pub fn get_hardware_channel_names(is_input: bool) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> result;

                    auto route = [AVAudioSession sharedInstance].currentRoute;

                    for (AVAudioSessionPortDescription* port in (isInput ? route.inputs : route.outputs))
                    {
                        for (AVAudioSessionChannelDescription* desc in port.channels)
                            result.add (nsStringToAloe (desc.channelName));
                    }

                    // A fallback for the iOS simulator and older iOS versions
                    if (result.isEmpty())
                        return { "Left", "Right" };

                    return result;
        */
    }
    
    pub fn limit_required_channels_to_hardware(
        num_hardware_channels_available: i32,
        required_channels:               BigInteger) -> BigInteger {
        
        todo!();
        /*
            requiredChannels.setRange (numHardwareChannelsAvailable,
                        requiredChannels.getHighestBit() + 1,
                        false);

                    return requiredChannels;
        */
    }
    
    pub fn get_active_channel_indices(active_channels_to_index: BigInteger) -> Vec<i32> {
        
        todo!();
        /*
            Vec<int> result;

                    auto index = activeChannelsToIndex.findNextSetBit (0);

                    while (index != -1)
                    {
                        result.add (index);
                        index = activeChannelsToIndex.findNextSetBit (++index);
                    }

                    return result;
        */
    }
    
    pub fn get_inactive_channel_indices(
        active_indices: &[i32],
        num_channels:   i32) -> Vec<i32> {
        
        todo!();
        /*
            Vec<int> result;

                    auto nextActiveChannel = activeIndices.begin();

                    for (int i = 0; i < numChannels; ++i)
                        if (nextActiveChannel != activeIndices.end() && i == *nextActiveChannel)
                            ++nextActiveChannel;
                        else
                            result.add (i);

                    return result;
        */
    }
}

