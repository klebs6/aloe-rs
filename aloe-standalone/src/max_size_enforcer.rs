crate::ix!();

pub struct StandalonePluginHolderCalbackMaxSizeEnforcerGetChannelWithOffset {
    offset: i32,
}

impl StandalonePluginHolderCalbackMaxSizeEnforcerGetChannelWithOffset {

    pub fn invoke<Ptr>(&mut self, ptr: Ptr) -> Ptr {
    
        todo!();
        /*
            return ptr + offset;
        */
    }
}

//-------------------------------
/**
  | This class can be used to ensure that
  | audio callbacks use buffers with a predictable
  | maximum size.
  | 
  | On some platforms (such as iOS 10), the
  | expected buffer size reported in audioDeviceAboutToStart
  | may be smaller than the blocks passed
  | to audioDeviceIOCallback. This can
  | lead to out-of-bounds reads if the render
  | callback depends on additional buffers
  | which were initialised using the smaller
  | size.
  | 
  | As a workaround, this class will ensure
  | that the render callback will only ever
  | be called with a block with a length less
  | than or equal to the expected block size.
  |
  */
pub struct StandalonePluginHolderCallbackMaxSizeEnforcer<'a> {
    inner:                  &'a mut dyn AudioIODeviceCallback,
    maximum_size:           i32, // default = 0
    stored_input_channels:  Vec<*const f32>,
    stored_output_channels: Vec<*mut f32>,
}

impl<'a> AudioIODeviceCallback for StandalonePluginHolderCallbackMaxSizeEnforcer<'a>
{
    fn audio_device_about_to_start(&mut self, device: *mut dyn AudioIODeviceInterface)  {
        
        todo!();
        /*
            maximumSize = device->getCurrentBufferSizeSamples();
                storedInputChannels .resize ((size_t) device->getActiveInputChannels() .countNumberOfSetBits());
                storedOutputChannels.resize ((size_t) device->getActiveOutputChannels().countNumberOfSetBits());

                inner.audioDeviceAboutToStart (device);
        */
    }
    
    fn audio_device_io_callback(
        &mut self, 
        input_channel_data:  *const *const f32,
        num_input_channels:  i32,
        output_channel_data: *mut *mut f32,
        num_output_channels: i32,
        num_samples:         i32

    )  {
        
        todo!();
        /*
            jassertquiet ((int) storedInputChannels.size()  == numInputChannels);
                jassertquiet ((int) storedOutputChannels.size() == numOutputChannels);

                int position = 0;

                while (position < numSamples)
                {
                    const auto blockLength = jmin (maximumSize, numSamples - position);

                    initChannelPointers (inputChannelData,  storedInputChannels,  position);
                    initChannelPointers (outputChannelData, storedOutputChannels, position);

                    inner.audioDeviceIOCallback (storedInputChannels.data(),
                                                 (int) storedInputChannels.size(),
                                                 storedOutputChannels.data(),
                                                 (int) storedOutputChannels.size(),
                                                 blockLength);

                    position += blockLength;
                }
        */
    }
    
    fn audio_device_stopped(&mut self)  {
        
        todo!();
        /*
            inner.audioDeviceStopped();
        */
    }
}

impl<'a> StandalonePluginHolderCallbackMaxSizeEnforcer<'a> {

    pub fn new(callback_in: &mut dyn AudioIODeviceCallback) -> Self {
    
        todo!();
        /*
        : inner(callbackIn),

        
        */
    }
    
    pub fn init_channel_pointers<Ptr, Vector>(&mut self, 
        source: Ptr,
        target: Vector,
        offset: i32)  {
    
        todo!();
        /*
            std::transform (source, source + target.size(), target.begin(), StandalonePluginHolderCalbackMaxSizeEnforcerGetChannelWithOffset { offset });
        */
    }
}

