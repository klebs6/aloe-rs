crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphManyToMultiConverter.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphManyToMultiConverter.cpp]

/**
  | Combine multiple mono inputs into one
  | interleaved multi-channel output.
  |
  */
pub struct FlowgraphManyToMultiConverter<'a> {
    base:   FlowGraphNode<'a>,
    inputs: Vec<Box<FlowGraphPortFloatInput<'a>>>,
    output: FlowGraphPortFloatOutput<'a>,
}

impl<'a> FlowgraphManyToMultiConverter<'a> {

    pub fn set_enabled(&mut self, enabled: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowgraphManyToMultiConverter";
        */
    }
    
    pub fn new(channel_count: i32) -> Self {
    
        todo!();
        /*


            : inputs(channelCount)
            , output(*this, channelCount) 

        for (int i = 0; i < channelCount; i++) {
            inputs[i] = std::make_unique<FlowGraphPortFloatInput>(*this, 1);
        }
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            int32_t channelCount = output.getSamplesPerFrame();

        for (int ch = 0; ch < channelCount; ch++) {
            const float *inputBuffer = inputs[ch]->getBuffer();
            float *outputBuffer = output.getBuffer() + ch;

            for (int i = 0; i < numFrames; i++) {
                // read one, write into the proper interleaved output channel
                float sample = *inputBuffer++;
                *outputBuffer = sample;
                outputBuffer += channelCount; // advance to next multichannel frame
            }
        }
        return numFrames;
        */
    }
}
