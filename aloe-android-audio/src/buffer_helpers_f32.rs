crate::ix!();

#[cfg(target_os="android")]
impl BufferHelpers<f32> {

    pub const isFloatingPoint: usize = 1;

    pub fn init_pcm_data_format(
        data_format:  &mut PCMDataFormatEx,
        num_channels: i32,
        sample_rate:  f64)  {
        
        todo!();
        /*
            dataFormat.formatType     = SL_ANDROID_DATAFORMAT_PCM_EX;
            dataFormat.numChannels    = (SLuint32) numChannels;
            dataFormat.samplesPerSec  = (SLuint32) (sampleRate * 1000);
            dataFormat.bitsPerSample  = 32;
            dataFormat.containerSize  = 32;
            dataFormat.channelMask    = (numChannels == 1) ? SL_SPEAKER_FRONT_CENTER :
                                                            (SL_SPEAKER_FRONT_LEFT | SL_SPEAKER_FRONT_RIGHT);
            dataFormat.endianness     = SL_BYTEORDER_LITTLEENDIAN;
            dataFormat.representation = SL_ANDROID_PCM_REPRESENTATION_FLOAT;
        */
    }
    
    pub fn prepare_callback_buffer(
        audio_buffer: &mut AudioBuffer<f32>,
        native:       *mut f32)  {
        
        todo!();
        /*
            if (audioBuffer.getNumChannels() == 1)
                audioBuffer.setDataToReferTo (&native, 1, audioBuffer.getNumSamples());
        */
    }
    
    pub fn convert_from_opensl(
        src_interleaved: *const f32,
        audio_buffer:    &mut AudioBuffer<f32>)  {
        
        todo!();
        /*
            if (audioBuffer.getNumChannels() == 1)
            {
                jassert (srcInterleaved == audioBuffer.getWritePointer (0));
                return;
            }

            for (int i = 0; i < audioBuffer.getNumChannels(); ++i)
            {
                using DstSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::NonConst>;
                using SrcSampleType = AudioData::Pointer<AudioData::Float32, AudioData::LittleEndian, AudioData::Interleaved,    AudioData::Const>;

                DstSampleType dstData (audioBuffer.getWritePointer (i));
                SrcSampleType srcData (srcInterleaved + i, audioBuffer.getNumChannels());
                dstData.convertSamples (srcData, audioBuffer.getNumSamples());
            }
        */
    }
    
    pub fn convert_to_opensl(
        audio_buffer:    &AudioBuffer<f32>,
        dst_interleaved: *mut f32)  {
        
        todo!();
        /*
            if (audioBuffer.getNumChannels() == 1)
            {
                jassert (dstInterleaved == audioBuffer.getReadPointer (0));
                return;
            }

            for (int i = 0; i < audioBuffer.getNumChannels(); ++i)
            {
                using DstSampleType = AudioData::Pointer<AudioData::Float32, AudioData::LittleEndian, AudioData::Interleaved,    AudioData::NonConst>;
                using SrcSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::Const>;

                DstSampleType dstData (dstInterleaved + i, audioBuffer.getNumChannels());
                SrcSampleType srcData (audioBuffer.getReadPointer (i));

                dstData.convertSamples (srcData, audioBuffer.getNumSamples());
            }
        */
    }
}
