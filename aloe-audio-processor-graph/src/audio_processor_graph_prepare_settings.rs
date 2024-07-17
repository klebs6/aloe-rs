crate::ix!();

pub struct AudioProcessorGraphPrepareSettings {
    precision:   AudioProcessorProcessingPrecision, //= ProcessingPrecision::singlePrecision;
    sample_rate: f64, // default = 0.0
    block_size:  i32, // default = 0
    valid:       bool, // default = false
}

pub type AudioProcessorGraphPrepareSettingsTied<'a> 
= (&'a AudioProcessorProcessingPrecision,&'a f64,&'a i32,&'a bool);

impl AudioProcessorGraphPrepareSettings {

    pub fn tie(&self) -> AudioProcessorGraphPrepareSettingsTied {
        
        todo!();
        /*
            return std::tie (precision, sampleRate, blockSize, valid);
        */
    }
}

impl PartialEq<AudioProcessorGraphPrepareSettings> for AudioProcessorGraphPrepareSettings {
    
    #[inline] fn eq(&self, other: &AudioProcessorGraphPrepareSettings) -> bool {
        todo!();
        /*
            return tie() == other.tie();
        */
    }
}

impl Eq for AudioProcessorGraphPrepareSettings {}
