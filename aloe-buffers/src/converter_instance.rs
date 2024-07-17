crate::ix!();

/**
  | A class that converts between two
  | templated AudioData::AudioDataPointer types, and
  | which implements the AudioData::Converter
  | interface.
  |
  | This can be used as a concrete instance of
  | the AudioData::Converter abstract class.
  |
  | @see AudioData::Converter
  */
#[no_copy]
pub struct ConverterInstance<SourceSampleType,DestSampleType> {
    source_channels: i32,
    dest_channels:   i32,
    p1:              PhantomData<SourceSampleType>,
    p2:              PhantomData<DestSampleType>,
}

impl<SourceSampleType,DestSampleType> 
Converter for ConverterInstance<SourceSampleType,DestSampleType> {

    fn convert_samples(&self, 
        dest:        *mut c_void,
        source:      *const c_void,
        num_samples: i32)  {
        
        todo!();
        /*
            SourceSampleType s (source, sourceChannels);
                DestSampleType d (dest, destChannels);
                d.convertSamples (s, numSamples);
        */
    }
    
    fn convert_samples_with_sub_channel(&self, 
        dest:               *mut c_void,
        dest_sub_channel:   i32,
        source:             *const c_void,
        source_sub_channel: i32,
        num_samples:        i32)  {
        
        todo!();
        /*
            jassert (destSubChannel < destChannels && sourceSubChannel < sourceChannels);

                SourceSampleType s (addBytesToPointer (source, sourceSubChannel * SourceSampleType::getBytesPerSample()), sourceChannels);
                DestSampleType d (addBytesToPointer (dest, destSubChannel * DestSampleType::getBytesPerSample()), destChannels);
                d.convertSamples (s, numSamples);
        */
    }
}

impl<SourceSampleType,DestSampleType> 
ConverterInstance<SourceSampleType,DestSampleType> {

    pub fn new(
        num_source_channels: Option<i32>,
        num_dest_channels:   Option<i32>) -> Self {

        let num_source_channels: i32 = num_source_channels.unwrap_or(1);
        let num_dest_channels:   i32 = num_dest_channels.unwrap_or(1);

        todo!();
        /*
        : source_channels(numSourceChannels),
        : dest_channels(numDestChannels),

        
        */
    }
}
