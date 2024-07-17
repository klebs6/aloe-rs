crate::ix!();

pub struct OboeAudioIODeviceBufferHelpers<OboeDataFormat> {
    _p0: PhantomData<OboeDataFormat>,
}

impl OboeAudioIODeviceBufferHelpers<i16> {

    pub fn oboe_audio_format() -> OboeAudioFormat {
        
        todo!();
        /*
            return OboeAudioFormat::I16;
        */
    }
    
    pub fn bit_depth() -> i32 {
        
        todo!();
        /*
            return 16;
        */
    }
    
    pub fn refer_audio_buffer_directly_to_oboe_if_possible(
        _0: *mut i16,
        _1: &mut AudioBuffer<f32>,
        _2: i32) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn convert_from_oboe(
        src_interleaved: *const i16,
        audio_buffer:    &mut AudioBuffer<f32>,
        num_samples:     i32)  {
        
        todo!();
        /*
            for (int i = 0; i < audioBuffer.getNumChannels(); ++i)
            {
                using DstSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::NonConst>;
                using SrcSampleType = AudioData::Pointer<AudioData::Int16,   AudioData::NativeEndian, AudioData::Interleaved,    AudioData::Const>;

                DstSampleType dstData (audioBuffer.getWritePointer (i));
                SrcSampleType srcData (srcInterleaved + i, audioBuffer.getNumChannels());
                dstData.convertSamples (srcData, numSamples);
            }
        */
    }
    
    pub fn convert_to_oboe(
        audio_buffer:    &AudioBuffer<f32>,
        dst_interleaved: *mut i16,
        num_samples:     i32)  {
        
        todo!();
        /*
            for (int i = 0; i < audioBuffer.getNumChannels(); ++i)
            {
                using DstSampleType = AudioData::Pointer<AudioData::Int16,   AudioData::NativeEndian, AudioData::Interleaved,    AudioData::NonConst>;
                using SrcSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::Const>;

                DstSampleType dstData (dstInterleaved + i, audioBuffer.getNumChannels());
                SrcSampleType srcData (audioBuffer.getReadPointer (i));
                dstData.convertSamples (srcData, numSamples);
            }
        */
    }
}

impl OboeAudioIODeviceBufferHelpers<f32> {
    
    pub fn oboe_audio_format() -> OboeAudioFormat {
        
        todo!();
        /*
            return OboeAudioFormat::Float;
        */
    }
    
    pub fn bit_depth() -> i32 {
        
        todo!();
        /*
            return 32;
        */
    }
    
    pub fn refer_audio_buffer_directly_to_oboe_if_possible(
        native_buffer: *mut f32,
        audio_buffer:  &mut AudioBuffer<f32>,
        num_samples:   i32) -> bool {
        
        todo!();
        /*
            if (audioBuffer.getNumChannels() == 1)
            {
                audioBuffer.setDataToReferTo (&nativeBuffer, 1, numSamples);
                return true;
            }

            return false;
        */
    }
    
    pub fn convert_from_oboe(
        src_interleaved: *const f32,
        audio_buffer:    &mut AudioBuffer<f32>,
        num_samples:     i32)  {
        
        todo!();
        /*
            auto numChannels = audioBuffer.getNumChannels();

            if (numChannels > 0)
            {
                // No need to convert, we instructed the buffer to point to the src data directly already
                jassert (audioBuffer.getWritePointer (0) != srcInterleaved);

                for (int i = 0; i < numChannels; ++i)
                {
                    using DstSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::NonConst>;
                    using SrcSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::Interleaved,    AudioData::Const>;

                    DstSampleType dstData (audioBuffer.getWritePointer (i));
                    SrcSampleType srcData (srcInterleaved + i, audioBuffer.getNumChannels());
                    dstData.convertSamples (srcData, numSamples);
                }
            }
        */
    }
    
    pub fn convert_to_oboe(
        audio_buffer:    &AudioBuffer<f32>,
        dst_interleaved: *mut f32,
        num_samples:     i32)  {
        
        todo!();
        /*
            auto numChannels = audioBuffer.getNumChannels();

            if (numChannels > 0)
            {
                // No need to convert, we instructed the buffer to point to the src data directly already
                jassert (audioBuffer.getReadPointer (0) != dstInterleaved);

                for (int i = 0; i < numChannels; ++i)
                {
                    using DstSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::Interleaved,    AudioData::NonConst>;
                    using SrcSampleType = AudioData::Pointer<AudioData::Float32, AudioData::NativeEndian, AudioData::NonInterleaved, AudioData::Const>;

                    DstSampleType dstData (dstInterleaved + i, audioBuffer.getNumChannels());
                    SrcSampleType srcData (audioBuffer.getReadPointer (i));
                    dstData.convertSamples (srcData, numSamples);
                }
            }
        */
    }
}
