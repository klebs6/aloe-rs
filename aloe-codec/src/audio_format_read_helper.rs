crate::ix!();

pub trait HasDestType   { type DestType; }
pub trait HasSourceType { type SourceType; }

/**
  | Used by AudioFormatReader subclasses
  | to copy data to different formats.
  |
  */
pub struct AudioFormatReadHelper<
    DestSampleType,
    SourceSampleType,
    SourceEndianness
> {
    _p0: PhantomData<DestSampleType>,
    _p1: PhantomData<SourceSampleType>,
    _p2: PhantomData<SourceEndianness>,
}

//------------------------------------
impl<
    DestSampleType,
    SourceSampleType,
    SourceEndianness
> 
HasDestType for AudioFormatReadHelper<
    DestSampleType,
    SourceSampleType,
    SourceEndianness
> {
    type DestType = AudioDataPointer<
        DestSampleType, 
        AudioDataNativeEndian, 
        AudioDataNonInterleaved, 
        AudioDataNonConst
    >;
}

//------------------------------------
impl<
    DestSampleType,
    SourceSampleType,
    SourceEndianness
> 
HasSourceType for AudioFormatReadHelper<
    DestSampleType,
    SourceSampleType,
    SourceEndianness
> {
    type SourceType = AudioDataPointer<
        SourceSampleType, 
        SourceEndianness, 
        AudioDataInterleaved, 
        AudioDataConst
    >;
}

//------------------------------------
impl<
    DestSampleType,
    SourceSampleType,
    SourceEndianness
> 
AudioFormatReadHelper<
    DestSampleType,
    SourceSampleType,
    SourceEndianness
> {

    pub fn read<TargetType>(
        dest_data:           *const *const TargetType,
        dest_offset:         i32,
        num_dest_channels:   i32,
        source_data:         *const c_void,
        num_source_channels: i32,
        num_samples:         i32)  {
    
        todo!();
        /*
            for (int i = 0; i < numDestChannels; ++i)
                {
                    if (void* targetChan = destData[i])
                    {
                        DestType dest (targetChan);
                        dest += destOffset;

                        if (i < numSourceChannels)
                            dest.convertSamples (SourceType (addBytesToPointer (sourceData, i * SourceType::getBytesPerSample()), numSourceChannels), numSamples);
                        else
                            dest.clearSamples (numSamples);
                    }
                }
        */
    }
}
