crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphMonoToMultiConverter.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphMonoToMultiConverter.cpp]

/**
  | Convert a monophonic stream to a multi-channel
  | interleaved stream with the same signal
  | on each channel.
  |
  */
pub struct FlowgraphMonoToMultiConverter<'a> {
    base:   FlowGraphNode<'a>,
    input:  FlowGraphPortFloatInput<'a>,
    output: FlowGraphPortFloatOutput<'a>,
}

impl<'a> FlowgraphMonoToMultiConverter<'a> {

    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowgraphMonoToMultiConverter";
        */
    }
    
    pub fn new(output_channel_count: i32) -> Self {
    
        todo!();
        /*


            : input(*this, 1)
            , output(*this, outputChannelCount)
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            const float *inputBuffer = input.getBuffer();
        float *outputBuffer = output.getBuffer();
        int32_t channelCount = output.getSamplesPerFrame();
        for (int i = 0; i < numFrames; i++) {
            // read one, write many
            float sample = *inputBuffer++;
            for (int channel = 0; channel < channelCount; channel++) {
                *outputBuffer++ = sample;
            }
        }
        return numFrames;
        */
    }
}
