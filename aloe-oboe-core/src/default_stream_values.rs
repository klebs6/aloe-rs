/*!
  | On API 16 to 26 OpenSL ES will be used.
  | When using OpenSL ES the optimal values
  | for sampleRate and framesPerBurst
  | are not known by the native code.
  | 
  | On API 17+ these values should be obtained
  | from the AudioManager using this code:
  | 
  | It can then be passed down to Oboe through
  | JNI.
  | 
  | <pre><code>
  |  // Note that this technique only works for built-in speakers and headphones.
  |  AudioManager myAudioMgr   = (AudioManager) getSystemService(Context.AUDIO_SERVICE);
  |  String sampleRateStr      = myAudioMgr.getProperty(AudioManager.PROPERTY_OUTPUT_SAMPLE_RATE);
  |  int defaultSampleRate     = Integer.parseInt(sampleRateStr);
  |  String framesPerBurstStr  = myAudioMgr.getProperty(AudioManager.PROPERTY_OUTPUT_FRAMES_PER_BUFFER);
  |  int defaultFramesPerBurst = Integer.parseInt(framesPerBurstStr);
  | </code></pre>
  |
  | AAudio will get the optimal framesPerBurst
  | from the HAL and will ignore this value.
  |
  */

crate::ix!();

/**
  | The default sample rate to use when opening
  | new audio streams
  |
  */
lazy_static!{
    /*
    static int32_t SampleRate;
    */
}

/**
  | The default frames per burst to use when
  | opening new audio streams
  |
  */
lazy_static!{
    /*
    static int32_t FramesPerBurst;
    */
}

/**
  | The default channel count to use when
  | opening new audio streams
  |
  */
lazy_static!{
    /*
    static int32_t ChannelCount;
    */
}
