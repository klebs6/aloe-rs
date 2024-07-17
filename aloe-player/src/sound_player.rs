crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/players/aloe_SoundPlayer.h]

/**
  | A simple sound player that you can add
  | to the AudioDeviceManager to play simple
  | sounds.
  | 
  | @see AudioProcessor, AudioProcessorGraph
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct SoundPlayer {
    format_manager: AudioFormatManager,
    player:         AudioSourcePlayer,
    mixer:          MixerAudioSource,
    sources:        Vec<Box<dyn AudioSource>>,
    sample_rate:    f64,
    buffer_size:    i32,
}

impl AudioIODeviceCallback for SoundPlayer {

    fn audio_device_io_callback(
        &mut self, 
        input_channel_data:  *const *const f32,
        num_input_channels:  i32,
        output_channel_data: *mut *mut f32,
        num_output_channels: i32,
        num_samples:         i32

    ) {
        
        todo!();
        /*
            player.audioDeviceIOCallback (inputChannelData, numInputChannels,
                                      outputChannelData, numOutputChannels,
                                      numSamples);
        */
    }
    
    fn audio_device_about_to_start(&mut self, device: *mut dyn AudioIODeviceInterface)  {
        
        todo!();
        /*
            if (device != nullptr)
        {
            sampleRate = device->getCurrentSampleRate();
            bufferSize = device->getCurrentBufferSizeSamples();
        }

        player.audioDeviceAboutToStart (device);
        */
    }
    
    fn audio_device_stopped(&mut self)  {
        
        todo!();
        /*
            player.audioDeviceStopped();
        */
    }
}

impl Default for SoundPlayer {
    
    fn default() -> Self {
    
        todo!();
        /*
        : sample_rate(44100.0),
        : buffer_size(512),

            formatManager.registerBasicFormats();
        player.setSource (&mixer);
        */
    }
}

impl Drop for SoundPlayer {

    fn drop(&mut self) {
        todo!();
        /* 
        mixer.removeAllInputs();
        player.setSource (nullptr);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/players/aloe_SoundPlayer.cpp]
impl SoundPlayer {
    
    /**
      | Plays a sound from a file.
      |
      */
    pub fn play_from_file(&mut self, file: &File)  {
        
        todo!();
        /*
            if (file.existsAsFile())
            play (formatManager.createReaderFor (file), true);
        */
    }
    
    /**
      | Convenient method to play sound from
      | a Aloe resource.
      |
      */
    pub fn play_resource(
        &mut self, 
        resource_data: *const c_void,
        resource_size: usize

    ) {
        
        todo!();
        /*
            if (resourceData != nullptr && resourceSize > 0)
        {
            auto mem = std::make_unique<MemoryInputStream> (resourceData, resourceSize, false);
            play (formatManager.createReaderFor (std::move (mem)), true);
        }
        */
    }
    
    /**
      | Plays the sound from an audio format
      | reader.
      | 
      | If deleteWhenFinished is true then
      | the format reader will be automatically
      | deleted once the sound has finished
      | playing.
      |
      */
    pub fn play_from_audio_format_reader(
        &mut self, 
        reader:               *mut AudioFormatReader,
        delete_when_finished: Option<bool>
    ) {

        let delete_when_finished: bool = delete_when_finished.unwrap_or(false);
        
        todo!();
        /*
            if (reader != nullptr)
            play (new AudioFormatReaderSource (reader, deleteWhenFinished), true, reader->sampleRate);
        */
    }
    
    /**
      | Plays the sound from an audio sample
      | buffer.
      | 
      | This will output the sound contained
      | in an audio sample buffer. If deleteWhenFinished
      | is true then the audio sample buffer
      | will be automatically deleted once
      | the sound has finished playing.
      | 
      | If playOnAllOutputChannels is true,
      | then if there are more output channels
      | than buffer channels, then the ones
      | that are available will be re-used on
      | multiple outputs so that something
      | is sent to all output channels. If it
      | is false, then the buffer will just be
      | played on the first output channels.
      |
      */
    pub fn play_from_audio_sample_buffer(
        &mut self, 
        buffer:                      *mut AudioBuffer<f32>,
        delete_when_finished:        Option<bool>,
        play_on_all_output_channels: Option<bool>

    )  {

        let delete_when_finished        = delete_when_finished.unwrap_or(false);
        let play_on_all_output_channels = play_on_all_output_channels.unwrap_or(false);
        
        todo!();
        /*
            if (buffer != nullptr)
            play (new AudioBufferSource (buffer, deleteWhenFinished, playOnAllOutputChannels), true);
        */
    }
    
    /**
      | Plays the sound from a positionable
      | audio source.
      | 
      | This will output the sound coming from
      | a positionable audio source. This gives
      | you slightly more control over the sound
      | playback compared to the other playSound
      | methods. For example, if you would like
      | to stop the sound prematurely you can
      | call this method with a TransportAudioSource
      | and then call audioSource->stop. Note
      | that, you must call audioSource->start
      | to start the playback, if your audioSource
      | is a TransportAudioSource.
      | 
      | The audio device manager will not hold
      | any references to this audio source
      | once the audio source has stopped playing
      | for any reason, for example when the
      | sound has finished playing or when you
      | have called audioSource->stop. Therefore,
      | calling audioSource->start() on a
      | finished audioSource will not restart
      | the sound again. If this is desired simply
      | call playSound with the same audioSource
      | again.
      | 
      | -----------
      | @param audioSource
      | 
      | the audio source to play
      | ----------
      | @param deleteWhenFinished
      | 
      | If this is true then the audio source
      | will be deleted once the device manager
      | has finished playing.
      | ----------
      | @param sampleRateOfSource
      | 
      | The sample rate of the source. If this
      | is zero, Aloe will assume that the sample
      | rate is the same as the audio output device.
      |
      */
    pub fn play_from_positionable_audio_source(
        &mut self, 
        audio_source:         *mut dyn PositionableAudioSource,
        delete_when_finished: Option<bool>,
        file_sample_rate:     Option<f64>

    ) {

        let delete_when_finished = delete_when_finished.unwrap_or(false);
        let file_sample_rate     = file_sample_rate.unwrap_or(0.0);
        
        todo!();
        /*
            if (audioSource != nullptr)
        {
            AudioTransportSource* transport = dynamic_cast<AudioTransportSource*> (audioSource);

            if (transport == nullptr)
            {
                if (deleteWhenFinished)
                {
                    transport = new AudioSourceOwningTransportSource (audioSource, fileSampleRate);
                }
                else
                {
                    transport = new AudioTransportSource();
                    transport->setSource (audioSource, 0, nullptr, fileSampleRate);
                    deleteWhenFinished = true;
                }
            }

            transport->start();
            transport->prepareToPlay (bufferSize, sampleRate);

            new AutoRemovingTransportSource (mixer, transport, deleteWhenFinished, bufferSize, sampleRate);
        }
        else
        {
            if (deleteWhenFinished)
                delete audioSource;
        }
        */
    }
    
    /**
      | Plays a beep through the current audio
      | device.
      | 
      | This is here to allow the audio setup
      | UI panels to easily include a "test"
      | button so that the user can check where
      | the audio is coming from.
      |
      */
    pub fn play_test_sound(&mut self)  {
        
        todo!();
        /*
            auto soundLength = (int) sampleRate;
        double frequency = 440.0;
        float amplitude = 0.5f;

        auto phasePerSample = MathConstants<double>::twoPi / (sampleRate / frequency);

        auto* newSound = new AudioBuffer<float> (1, soundLength);

        for (int i = 0; i < soundLength; ++i)
            newSound->setSample (0, i, amplitude * (float) std::sin (i * phasePerSample));

        newSound->applyGainRamp (0, 0, soundLength / 10, 0.0f, 1.0f);
        newSound->applyGainRamp (0, soundLength - soundLength / 4, soundLength / 4, 1.0f, 0.0f);

        play (newSound, true, true);
        */
    }
    
    pub fn audio_device_error(&mut self, error_message: &String)  {
        
        todo!();
        /*
            player.audioDeviceError (errorMessage);
        */
    }
}
