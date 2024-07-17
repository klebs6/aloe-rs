crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphMultiToMonoConverter.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FlowgraphMultiToMonoConverter.cpp]

/**
  | Convert a multi-channel interleaved
  | stream to a monophonic stream by extracting
  | channel[0].
  |
  */
pub struct FlowgraphMultiToMonoConverter<'a> {
    base:   FlowGraphNode<'a>,
    input:  FlowGraphPortFloatInput<'a>,
    output: FlowGraphPortFloatOutput<'a>,
}

impl<'a> FlowgraphMultiToMonoConverter<'a> {

    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FlowgraphMultiToMonoConverter";
        */
    }
    
    pub fn new(input_channel_count: i32) -> Self {
    
        todo!();
        /*


            : input(*this, inputChannelCount)
            , output(*this, 1)
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            const float *inputBuffer = input.getBuffer();
        float *outputBuffer = output.getBuffer();
        int32_t channelCount = input.getSamplesPerFrame();
        for (int i = 0; i < numFrames; i++) {
            // read first channel of multi stream, write many
            *outputBuffer++ = *inputBuffer;
            inputBuffer += channelCount;
        }
        return numFrames;
        */
    }
}

