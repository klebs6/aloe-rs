crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_AudioAppComponent.h]

/**
  | A base class for writing audio apps that
  | stream from the audio i/o devices. Conveniently
  | combines a Component with an AudioSource
  | to provide a starting point for your
  | audio applications.
  | 
  | A subclass can inherit from this and
  | implement just a few methods such as
  | getNextAudioBlock(). The base class
  | provides a basic AudioDeviceManager
  | object and runs audio through the default
  | output device.
  | 
  | An application should only create one
  | global instance of this object and multiple
  | classes should not inherit from this.
  | 
  | This class should not be inherited when
  | creating a plug-in as the host will handle
  | audio streams from hardware devices.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioAppComponent<'a> {
    base:                        Component<'a>,
    device_manager:              &'a mut AudioDeviceManager<'a>,
    default_device_manager:      AudioDeviceManager<'a>,
    audio_source_player:         AudioSourcePlayer,
    using_custom_device_manager: bool,
}

impl<'a> AudioSource for AudioAppComponent<'a> {}

impl<'a> PrepareToPlayAudioSource for AudioAppComponent<'a> {

    fn prepare_to_play(&mut self, _: i32, _: f64) { 
        todo!() 
    }
}

impl<'a> ReleaseResources for AudioAppComponent<'a> {

    fn release_resources(&mut self) { 
        todo!() 
    }
}

impl<'a> GetNextAudioBlock for AudioAppComponent<'a> {

    fn get_next_audio_block(&mut self, _: &aloe_audio_interface::AudioSourceChannelInfo) { 
        todo!() 
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_AudioAppComponent.cpp]
impl<'a> Drop for AudioAppComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        // If you hit this then your derived class must call shutdown audio in
        // destructor!
        jassert (audioSourcePlayer.getCurrentSource() == nullptr);
 */
    }
}

impl<'a> Default for AudioAppComponent<'a> {

    fn default() -> Self {
    
        todo!();
        /*


            : deviceManager (defaultDeviceManager),
          usingCustomDeviceManager (false)
        */
    }
}

impl<'a> AudioAppComponent<'a> {
    
    pub fn new(adm: &mut AudioDeviceManager) -> Self {
    
        todo!();
        /*


            : deviceManager (adm),
          usingCustomDeviceManager (true)
        */
    }
    
    /**
      | A subclass should call this from their
      | constructor, to set up the audio.
      |
      */
    pub fn set_audio_channels(
        &mut self, 
        num_input_channels:  i32,
        num_output_channels: i32,
        xml:                 *const XmlElement

    ) {
        
        todo!();
        /*
            String audioError;

        if (usingCustomDeviceManager && xml == nullptr)
        {
            auto setup = deviceManager.getAudioDeviceSetup();

            if (setup.inputChannels.countNumberOfSetBits() != numInputChannels
                 || setup.outputChannels.countNumberOfSetBits() != numOutputChannels)
            {
                setup.inputChannels.clear();
                setup.outputChannels.clear();

                setup.inputChannels.setRange (0, numInputChannels, true);
                setup.outputChannels.setRange (0, numOutputChannels, true);

                audioError = deviceManager.setAudioDeviceSetup (setup, false);
            }
        }
        else
        {
            audioError = deviceManager.initialise (numInputChannels, numOutputChannels, xml, true);
        }

        jassert (audioError.isEmpty());

        deviceManager.addAudioCallback (&audioSourcePlayer);
        audioSourcePlayer.setSource (this);
        */
    }
    
    /**
      | Shuts down the audio device and clears
      | the audio source.
      | 
      | This method should be called in the destructor
      | of the derived class otherwise an assertion
      | will be triggered.
      |
      */
    pub fn shutdown_audio(&mut self)  {
        
        todo!();
        /*
            audioSourcePlayer.setSource (nullptr);
        deviceManager.removeAudioCallback (&audioSourcePlayer);

        // other audio callbacks may still be using the device
        if (! usingCustomDeviceManager)
            deviceManager.closeAudioDevice();
        */
    }
}
