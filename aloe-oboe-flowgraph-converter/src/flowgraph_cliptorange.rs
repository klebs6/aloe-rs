crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FowgraphClipToRange.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/oboe/src/flowgraph/FowgraphClipToRange.cpp]

/**
  | This is 3 dB, (10^(3/20)), to match the maximum
  | headroom in AudioTrack for float data.
  |
  | It is designed to allow occasional transient
  | peaks.
  */
pub const kDefaultMaxHeadroom: f32 = 1.41253754;
pub const kDefaultMinHeadroom: f32 = -kDefaultMaxHeadroom;

pub struct FowgraphClipToRange<'a> {
    base:    FlowGraphFilter<'a>,
    minimum: f32, // default = kDefaultMinHeadroom
    maximum: f32, // default = kDefaultMaxHeadroom
}

impl<'a> FowgraphClipToRange<'a> {
    
    pub fn set_minimum(&mut self, min: f32)  {
        
        todo!();
        /*
            mMinimum = min;
        */
    }
    
    pub fn get_minimum(&self) -> f32 {
        
        todo!();
        /*
            return mMinimum;
        */
    }
    
    pub fn set_maximum(&mut self, min: f32)  {
        
        todo!();
        /*
            mMaximum = min;
        */
    }
    
    pub fn get_maximum(&self) -> f32 {
        
        todo!();
        /*
            return mMaximum;
        */
    }
    
    pub fn get_name(&mut self) -> *const u8 {
        
        todo!();
        /*
            return "FowgraphClipToRange";
        */
    }
    
    pub fn new(channel_count: i32) -> Self {
    
        todo!();
        /*
        : flow_graph_filter(channelCount),

        
        */
    }
    
    pub fn on_process(&mut self, num_frames: i32) -> i32 {
        
        todo!();
        /*
            const float *inputBuffer = input.getBuffer();
        float *outputBuffer = output.getBuffer();

        int32_t numSamples = numFrames * output.getSamplesPerFrame();
        for (int32_t i = 0; i < numSamples; i++) {
            *outputBuffer++ = std::min(mMaximum, std::max(mMinimum, *inputBuffer++));
        }

        return numFrames;
        */
    }
}
