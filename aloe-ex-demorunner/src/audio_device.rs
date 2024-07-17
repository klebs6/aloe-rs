crate::ix!();

pub fn get_current_default_audio_device_name(
        device_manager: &mut AudioDeviceManager,
        is_input:       bool) -> String {
    
    todo!();
    /*
        auto* deviceType = deviceManager.getCurrentDeviceTypeObject();
        jassert (deviceType != nullptr);

        if (deviceType != nullptr)
        {
            auto deviceNames = deviceType->getDeviceNames();
            return deviceNames [deviceType->getDefaultDeviceIndex (isInput)];
        }

        return {};
    */
}

/**
  | (returns a shared AudioDeviceManager
  | object that all the demos can use)
  |
  */
pub fn get_shared_audio_device_manager<'a>(
    num_input_channels:  Option<i32>,
    num_output_channels: Option<i32>

) -> &'a mut AudioDeviceManager<'a> {
    
    let num_input_channels:  i32 = num_input_channels.unwrap_or(-1);
    let num_output_channels: i32 = num_output_channels.unwrap_or(-1);

    todo!();
    /*
        if (sharedAudioDeviceManager == nullptr)
            sharedAudioDeviceManager.reset (new AudioDeviceManager());

        auto* currentDevice = sharedAudioDeviceManager->getCurrentAudioDevice();

        if (numInputChannels < 0)
            numInputChannels = (currentDevice != nullptr ? currentDevice->getActiveInputChannels().countNumberOfSetBits() : 1);

        if (numOutputChannels < 0)
            numOutputChannels = (currentDevice != nullptr ? currentDevice->getActiveOutputChannels().countNumberOfSetBits() : 2);

        if (numInputChannels > 0 && ! RuntimePermissions::isGranted (RuntimePermissions::recordAudio))
        {
            RuntimePermissions::request (RuntimePermissions::recordAudio,
                                         [numInputChannels, numOutputChannels] (bool granted)
                                         {
                                             if (granted)
                                                 getSharedAudioDeviceManager (numInputChannels, numOutputChannels);
                                         });

            numInputChannels = 0;
        }

        if (sharedAudioDeviceManager->getCurrentAudioDevice() != nullptr)
        {
            auto setup = sharedAudioDeviceManager->getAudioDeviceSetup();

            auto numInputs  = jmax (numInputChannels,  setup.inputChannels.countNumberOfSetBits());
            auto numOutputs = jmax (numOutputChannels, setup.outputChannels.countNumberOfSetBits());

            auto oldInputs  = setup.inputChannels.countNumberOfSetBits();
            auto oldOutputs = setup.outputChannels.countNumberOfSetBits();

            if (oldInputs != numInputs || oldOutputs != numOutputs)
            {
                if (oldInputs == 0 && oldOutputs == 0)
                {
                    sharedAudioDeviceManager->initialise (numInputChannels, numOutputChannels, nullptr, true, {}, nullptr);
                }
                else
                {
                    setup.useDefaultInputChannels = setup.useDefaultOutputChannels = false;

                    setup.inputChannels.clear();
                    setup.outputChannels.clear();

                    setup.inputChannels.setRange (0, numInputs, true);
                    setup.outputChannels.setRange (0, numOutputs, true);

                    if (oldInputs == 0 && numInputs > 0 && setup.inputDeviceName.isEmpty())
                        setup.inputDeviceName = getCurrentDefaultAudioDeviceName (*sharedAudioDeviceManager, true);

                    if (oldOutputs == 0 && numOutputs > 0 && setup.outputDeviceName.isEmpty())
                        setup.outputDeviceName = getCurrentDefaultAudioDeviceName (*sharedAudioDeviceManager, false);

                    sharedAudioDeviceManager->setAudioDeviceSetup (setup, false);
                }
            }
        }
        else
        {
            sharedAudioDeviceManager->initialise (numInputChannels, numOutputChannels, nullptr, true, {}, nullptr);
        }

        return *sharedAudioDeviceManager;
    */
}
