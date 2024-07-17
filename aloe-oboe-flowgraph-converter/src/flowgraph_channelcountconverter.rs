crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphChannelCountConverter.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphChannelCountConverter.cpp]

/**
  | Change the number of number of channels
  | without mixing.
  | 
  | When increasing the channel count,
  | duplicate input channels.
  | 
  | When decreasing the channel count,
  | drop input channels.
  |
  */
pub struct FlowgraphChannelCountConverter<'a> {
    base:   FlowGraphNode<'a>,
    input:  FlowGraphPortFloatInput<'a>,
    output: FlowGraphPortFloatOutput<'a>,
}

impl<'a> FlowgraphChannelCountConverter<'a> {
    
    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowgraphChannelCountConverter";
        */
    }
    
    pub fn new(
        input_channel_count:  i32,
        output_channel_count: i32) -> Self {
    
        todo!();
        /*


            : input(*this, inputChannelCount)
            , output(*this, outputChannelCount)
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            const float *inputBuffer = input.getBuffer();
        float *outputBuffer = output.getBuffer();
        int32_t inputChannelCount = input.getSamplesPerFrame();
        int32_t outputChannelCount = output.getSamplesPerFrame();
        for (int i = 0; i < numFrames; i++) {
            int inputChannel = 0;
            for (int outputChannel = 0; outputChannel < outputChannelCount; outputChannel++) {
                // Copy input channels to output channels.
                // Wrap if we run out of inputs.
                // Discard if we run out of outputs.
                outputBuffer[outputChannel] = inputBuffer[inputChannel];
                inputChannel = (inputChannel == inputChannelCount)
                        ? 0 : inputChannel + 1;
            }
            inputBuffer += inputChannelCount;
            outputBuffer += outputChannelCount;
        }
        return numFrames;
        */
    }
}

