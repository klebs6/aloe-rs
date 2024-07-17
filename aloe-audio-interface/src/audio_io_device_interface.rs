crate::ix!();

pub trait AudioIODeviceInterface
: GetOutputChannelNames
+ GetInputChannelNames
+ GetAvailableSampleRates
+ GetAvailableBufferSizes
+ GetDefaultBufferSize
+ OpenDevice
+ Close
+ DeviceIsOpen
+ DeviceStart
+ Stop
+ CheckIsPlaying
+ GetLastError
+ GetCurrentBufferSizeSamples
+ GetCurrentSampleRate
+ GetCurrentBitDepth
+ GetActiveOutputChannels
+ GetActiveInputChannels
+ GetOutputLatencyInSamples
+ GetInputLatencyInSamples
+ HasControlPanel
+ ShowControlPanel
+ SetAudioProcessingEnabled
+ GetxRunCount
{ }
