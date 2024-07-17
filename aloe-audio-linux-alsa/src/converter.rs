crate::ix!();

pub struct MissingLink {}
pub type AudioDataConverter = MissingLink;
pub type SndPcm             = MissingLink;

pub struct ALSADeviceConverterHelper<SampleType> {
    _p0: PhantomData<SampleType>,
}

impl<SampleType> ALSADeviceConverterHelper<SampleType> {

    pub fn create_converter(
        for_input:                bool,
        is_little_endian:         bool,
        num_interleaved_channels: i32,
        interleaved:              bool) -> *mut AudioDataConverter {
        
        todo!();
        /*
            if (interleaved)
                    return create<AudioData::Interleaved> (forInput, isLittleEndian, numInterleavedChannels);

                return create<AudioData::NonInterleaved> (forInput, isLittleEndian, numInterleavedChannels);
        */
    }
    
    pub fn create<InterleavedType>(
        for_input:                bool,
        is_little_endian:         bool,
        num_interleaved_channels: i32) -> *mut AudioDataConverter {
    
        todo!();
        /*
            if (forInput)
                {
                    using DestType = AudioData::Pointer <AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::NonConst>;

                    if (isLittleEndian)
                        return new AudioData::ConverterInstance <AudioData::Pointer <SampleType, AudioData::LittleEndian, InterleavedType, AudioData::Const>, DestType> (numInterleavedChannels, 1);

                    return new AudioData::ConverterInstance <AudioData::Pointer <SampleType, AudioData::BigEndian, InterleavedType, AudioData::Const>, DestType> (numInterleavedChannels, 1);
                }

                using SourceType = AudioData::Pointer <AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::Const>;

                if (isLittleEndian)
                    return new AudioData::ConverterInstance <SourceType, AudioData::Pointer <SampleType, AudioData::LittleEndian, InterleavedType, AudioData::NonConst>> (1, numInterleavedChannels);

                return new AudioData::ConverterInstance <SourceType, AudioData::Pointer <SampleType, AudioData::BigEndian, InterleavedType, AudioData::NonConst>> (1, numInterleavedChannels);
        */
    }
}

