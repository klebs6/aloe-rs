crate::ix!();

#[cfg(target_os="android")]
pub struct BufferHelpers<T> {
    _0: PhantomData<T>,
}

#[cfg(target_os="android")]
impl BufferHelpers<i16> {

    pub const isFloatingPoint: usize = 0;

    pub fn init_pcm_data_format(
        data_format:  &mut PCMDataFormatEx,
        num_channels: i32,
        sample_rate:  f64)  {
        
        todo!();
        /*
            dataFormat.formatType     = SL_DATAFORMAT_PCM;
            dataFormat.numChannels    = (SLuint32) numChannels;
            dataFormat.samplesPerSec  = (SLuint32) (sampleRate * 1000);
            dataFormat.bitsPerSample  = SL_PCMSAMPLEFORMAT_FIXED_16;
            dataFormat.containerSize  = SL_PCMSAMPLEFORMAT_FIXED_16;
            dataFormat.channelMask    = (numChannels == 1) ? SL_SPEAKER_FRONT_CENTER :
                                                            (SL_SPEAKER_FRONT_LEFT | SL_SPEAKER_FRONT_RIGHT);
            dataFormat.endianness     = SL_BYTEORDER_LITTLEENDIAN;
            dataFormat.representation = 0;
        */
    }
    
    pub fn prepare_callback_buffer(
        _0: &mut AudioBuffer<f32>,
        _1: *mut i16)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn convert_from_opensl(
        src_interleaved: *const i16,
        audio_buffer:    &mut AudioBuffer<f32>)  {
        
        todo!();
        /*
            for (int i = 0; i < audioBuffer.getNumChannels(); ++i)
            {
                using DstSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::NonConst>;
                using SrcSampleType = AudioData::Pointer<AudioData::Int16,   AudioData::LittleEndian, AudioData::Interleaved,    AudioData::Const>;

                DstSampleType dstData (audioBuffer.getWritePointer (i));
                SrcSampleType srcData (srcInterleaved + i, audioBuffer.getNumChannels());
                dstData.convertSamples (srcData, audioBuffer.getNumSamples());
            }
        */
    }
    
    pub fn convert_to_opensl(
        audio_buffer:    &AudioBuffer<f32>,
        dst_interleaved: *mut i16)  {
        
        todo!();
        /*
            for (int i = 0; i < audioBuffer.getNumChannels(); ++i)
            {
                using DstSampleType = AudioData::Pointer<AudioData::Int16,   AudioData::LittleEndian, AudioData::Interleaved, AudioData::NonConst>;
                using SrcSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::Const>;

                DstSampleType dstData (dstInterleaved + i, audioBuffer.getNumChannels());
                SrcSampleType srcData (audioBuffer.getReadPointer (i));

                dstData.convertSamples (srcData, audioBuffer.getNumSamples());
            }
        */
    }
}
