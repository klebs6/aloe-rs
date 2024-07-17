crate::ix!();

/**
  | Used by AudioFormatWriter subclasses
  | to copy data to different formats.
  |
  */
pub struct AudioFormatWriteHelper<DestSampleType,SourceSampleType,DestEndianness> {
    _p0: PhantomData<DestSampleType>,
    _p1: PhantomData<SourceSampleType>,
    _p2: PhantomData<DestEndianness>,
}

pub trait HasWriteHelperTypes {

    type DestType;
    type SourceType;
}

//-----------------------------------
impl<
    DestSampleType,
    SourceSampleType,
    DestEndianness
>
HasWriteHelperTypes for 
AudioFormatWriteHelper<
    DestSampleType,
    SourceSampleType,
    DestEndianness
> {

    type DestType = AudioDataPointer<
        DestSampleType,   
        DestEndianness,          
        AudioDataInterleaved,    
        AudioDataNonConst
    >;

    type SourceType = AudioDataPointer<
        SourceSampleType, 
        AudioDataNativeEndian, 
        AudioDataNonInterleaved, 
        AudioDataConst
    >;
}

//-----------------------------------
impl<
    DestSampleType,
    SourceSampleType,
    DestEndianness
> 
AudioFormatWriteHelper<
    DestSampleType,
    SourceSampleType,
    DestEndianness
> {

    pub fn write(
        dest_data:         *mut c_void,
        num_dest_channels: i32,
        source:            *const *const i32,
        num_samples:       i32,
        source_offset:     Option<i32>

    ) {

        let source_offset: i32 = source_offset.unwrap_or(0);

        todo!();
        /*
            for (int i = 0; i < numDestChannels; ++i)
                {
                    const DestType dest (addBytesToPointer (destData, i * DestType::getBytesPerSample()), numDestChannels);

                    if (*source != nullptr)
                    {
                        dest.convertSamples (SourceType (*source + sourceOffset), numSamples);
                        ++source;
                    }
                    else
                    {
                        dest.clearSamples (numSamples);
                    }
                }
        */
    }
}
