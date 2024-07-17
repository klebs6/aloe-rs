crate::ix!();

/**
  | A pointer to a block of audio data with
  | a particular encoding.
  |
  | This object can be used to read and write
  | from blocks of encoded audio samples. To
  | create one, you specify the audio format
  | as a series of template parameters, e.g.
  |
  | @code
  | // this creates a pointer for reading from
  | // a const array of 16-bit little-endian
  | // packed samples.
  | AudioData::AudioDataPointer <AudioData::AudioDataInt16,
  |                     AudioData::AudioDataLittleEndian,
  |                     AudioData::AudioDataNonInterleaved,
  |                     AudioData::AudioDataConst> pointer (someRawAudioData);
  |
  | // These methods read the sample that is being pointed to
  | float firstSampleAsFloat = pointer.getAsFloat();
  | int32 firstSampleAsInt = pointer.getAsInt32();
  | ++pointer; // moves the pointer to the next sample.
  | pointer += 3; // skips the next 3 samples.
  | @endcode
  |
  | The convertSamples() method lets you copy
  | a range of samples from one format to
  | another, automatically converting its
  | format.
  |
  | @see AudioData::Converter
  */
pub struct AudioDataPointer<SampleFormat,Endianness,InterleavingType,Constness> {
    base: InterleavingType,
    data: SampleFormat,

    p1: PhantomData<Endianness>,
    p2: PhantomData<Constness>,
}

impl<SampleFormat,Endianness,InterleavingType,Constness> 

AddAssign<&i32> for AudioDataPointer<SampleFormat,Endianness,InterleavingType,Constness> {
    
    /**
      Adds a number of samples to the pointer's
      position.
      */
    #[inline] fn add_assign(&mut self, other: &i32) {
        todo!();
        /*
            this->advanceDataBy (data, samplesToJump); return *this;
        */
    }
}

impl<SampleFormat,Endianness,InterleavingType,Constness> 
AudioDataPointer<SampleFormat,Endianness,InterleavingType,Constness> {

    /** 
      | Creates a non-interleaved pointer from
      | some raw data in the appropriate format.
      |
      | This constructor is only used if
      | you've specified the
      | AudioData::AudioDataNonInterleaved option - for
      | interleaved formats, use the
      | constructor that also takes a number
      | of channels.
      */
    pub fn new_from_source_data(source_data: *const c_void) -> Self {
    
        todo!();
        /*


            : data (Constness::toVoidPtr (sourceData))
                // If you're using interleaved data, call the other constructor! If you're using non-interleaved data,
                // you should pass AudioDataNonInterleaved as the template parameter for the interleaving type!
                static_assert (InterleavingType::isInterleavedType == 0, "Incorrect constructor for interleaved data");
        */
    }

    /** 
      | Creates a pointer from some raw data in
      | the appropriate format with the specified
      | number of interleaved channels.  For
      | non-interleaved data, use the other
      | constructor.
      */
    pub fn new_interleaved_from_source_data(
        source_data:     *const c_void,
        num_interleaved: i32) -> Self {
    
        todo!();
        /*


            : InterleavingType (numInterleaved), data (Constness::toVoidPtr (sourceData))
        */
    }

    /**
      | Creates a copy of another pointer.
      |
      */
    pub fn new_from_pointer(other: &AudioDataPointer<SampleFormat,Endianness,InterleavingType,Constness>) -> Self {
    
        todo!();
        /*
        : interleaving_type(other),
        : data(other.data),

        
        */
    }
    
    pub fn assign_from(&mut self, other: &AudioDataPointer<SampleFormat,Endianness,InterleavingType,Constness>) -> &mut AudioDataPointer<SampleFormat,Endianness,InterleavingType,Constness> {
        
        todo!();
        /*
            InterleavingType::operator= (other);
                data = other.data;
                return *this;
        */
    }
        
    /** 
      | Returns the value of the first sample as
      | a floating point value.
      |
      | The value will be in the range -1.0 to
      | 1.0 for integer formats. For floating
      | point formats, the value could be
      | outside that range, although -1 to
      | 1 is the standard range.
      */
    #[inline] pub fn get_as_float(&self) -> f32 {
        
        todo!();
        /*
            return Endianness::getAsFloat (data);
        */
    }

    /** 
      | Sets the value of the first sample as
      | a floating point value.
      |
      | (This method can only be used if the
      | AudioData::AudioDataNonConst option was used).
      | The value should be in the range -1.0
      | to 1.0 - for integer formats, values
      | outside that range will be
      | clipped. For floating point formats,
      | any value passed in here will be
      | written directly, although -1 to 1 is
      | the standard range.
      */
    #[inline] pub fn set_as_float(&mut self, new_value: f32)  {
        
        todo!();
        /*
            // trying to write to a const pointer! For a writeable one, use AudioData::AudioDataNonConst instead!
                static_assert (Constness::isConst == 0, "Attempt to write to a const pointer");
                Endianness::setAsFloat (data, newValue);
        */
    }

    /** 
      | Returns the value of the first sample as
      | a 32-bit integer.
      |
      | The value returned will be in the
      | range 0x80000000 to 0x7fffffff, and
      | shorter values will be shifted to fill
      | this range (e.g. if you're reading
      | from 24-bit data, the values will be
      | shifted up by 8 bits when returned
      | here). If the source data is floating
      | point, values beyond -1.0 to 1.0 will
      | be clipped so that -1.0 maps onto
      | -0x7fffffff and 1.0 maps to
      | 0x7fffffff.
      */
    #[inline] pub fn get_as_int32(&self) -> i32 {
        
        todo!();
        /*
            return Endianness::getAsInt32 (data);
        */
    }

    /** 
      | Sets the value of the first sample as
      | a 32-bit integer.
      |
      | This will be mapped to the range of
      | the format that is being written - see
      | getAsInt32().
      */
    #[inline] pub fn set_as_int32(&mut self, new_value: i32)  {
        
        todo!();
        /*
            // trying to write to a const pointer! For a writeable one, use AudioData::AudioDataNonConst instead!
                static_assert (Constness::isConst == 0, "Attempt to write to a const pointer");
                Endianness::setAsInt32 (data, newValue);
        */
    }

    /**
      | Moves the pointer along to the next sample.
      |
      */
    #[inline] pub fn increment(&mut self) -> &mut AudioDataPointer<SampleFormat,Endianness,InterleavingType,Constness> {
        
        todo!();
        /*
            advance(); return *this;
        */
    }

    /**
      | Moves the pointer back to the previous
      | sample.
      |
      */
    #[inline] pub fn decrement(&mut self) -> &mut AudioDataPointer<SampleFormat,Endianness,InterleavingType,Constness> {
        
        todo!();
        /*
            this->advanceDataBy (data, -1); return *this;
        */
    }

    /** 
      | Writes a stream of samples into this
      | pointer from another pointer.  This will
      | copy the specified number of samples,
      | converting between formats appropriately.
      */
    pub fn convert_samples_with_pointer(
        &self, 
        source:      AudioDataPointer<SampleFormat,Endianness,InterleavingType,Constness>,
        num_samples: i32)  {
        
        todo!();
        /*
            // trying to write to a const pointer! For a writeable one, use AudioData::AudioDataNonConst instead!
                static_assert (Constness::isConst == 0, "Attempt to write to a const pointer");

                for (AudioDataPointer dest (*this); --numSamples >= 0;)
                {
                    dest.data.copyFromSameType (source.data);
                    dest.advance();
                    source.advance();
                }
        */
    }

    /** 
      | Writes a stream of samples into this
      | pointer from another pointer.
      |
      | This will copy the specified number of
      | samples, converting between formats
      | appropriately.
      */
    pub fn convert_samples_into_pointer_from_another<OtherPointerType>(
        &self, 
        source:      OtherPointerType,
        num_samples: i32)  {
    
        todo!();
        /*
            // trying to write to a const pointer! For a writeable one, use AudioData::AudioDataNonConst instead!
                static_assert (Constness::isConst == 0, "Attempt to write to a const pointer");

                AudioDataPointer dest (*this);

                if (source.getRawData() != getRawData() || source.getNumBytesBetweenSamples() >= getNumBytesBetweenSamples())
                {
                    while (--numSamples >= 0)
                    {
                        Endianness::copyFrom (dest.data, source);
                        dest.advance();
                        ++source;
                    }
                }
                else // copy backwards if we're increasing the sample width..
                {
                    dest += numSamples;
                    source += numSamples;

                    while (--numSamples >= 0)
                        Endianness::copyFrom ((--dest).data, --source);
                }
        */
    }

    /**
      | Sets a number of samples to zero.
      |
      */
    pub fn clear_samples(&self, num_samples: i32)  {
        
        todo!();
        /*
            AudioDataPointer dest (*this);
                dest.clear (dest.data, numSamples);
        */
    }

    /**
      | Scans a block of data, returning the
      | lowest and highest levels as floats
      |
      */
    pub fn find_min_and_max_with_num_samples(&self, num_samples: usize) -> Range<f32> {
        
        todo!();
        /*
            if (numSamples == 0)
                    return Range<float>();

                AudioDataPointer dest (*this);

                if (isFloatingPoint())
                {
                    float mn = dest.getAsFloat();
                    dest.advance();
                    float mx = mn;

                    while (--numSamples > 0)
                    {
                        const float v = dest.getAsFloat();
                        dest.advance();

                        if (mx < v)  mx = v;
                        if (v < mn)  mn = v;
                    }

                    return Range<float> (mn, mx);
                }

                int32 mn = dest.getAsInt32();
                dest.advance();
                int32 mx = mn;

                while (--numSamples > 0)
                {
                    const int v = dest.getAsInt32();
                    dest.advance();

                    if (mx < v)  mx = v;
                    if (v < mn)  mn = v;
                }

                return Range<float> ((float) mn * (float) (1.0 / (1.0 + (double) AudioDataInt32::maxValue)),
                                     (float) mx * (float) (1.0 / (1.0 + (double) AudioDataInt32::maxValue)));
        */
    }

    /**
      | Scans a block of data, returning the
      | lowest and highest levels as floats
      |
      */
    pub fn find_min_and_max(
        &self, 
        num_samples: usize,
        min_value:   &mut f32,
        max_value:   &mut f32
    )
    {
        todo!();

        /*
            Range<float> r (findMinAndMax (numSamples));
                minValue = r.getStart();
                maxValue = r.getEnd();
        */
    }

    /**
      | Returns true if the pointer is using
      | a floating-point format.
      |
      */
    pub fn is_floating_point() -> bool {
        
        todo!();
        /*
            return (bool) SampleFormat::isFloat;
        */
    }

    /**
      | Returns true if the format is big-endian.
      |
      */
    pub fn is_big_endian() -> bool {
        
        todo!();
        /*
            return (bool) Endianness::isBigEndian;
        */
    }

    /**
      | Returns the number of bytes in each sample
      | (ignoring the number of interleaved
      | channels).
      |
      */
    pub fn get_bytes_per_sample() -> i32 {
        
        todo!();
        /*
            return (int) SampleFormat::bytesPerSample;
        */
    }

    /**
      | Returns the number of interleaved channels
      | in the format.
      |
      */
    pub fn get_num_interleaved_channels(&self) -> i32 {
        
        todo!();
        /*
            return (int) this->numInterleavedChannels;
        */
    }

    /**
      | Returns the number of bytes between
      | the start address of each sample.
      |
      */
    pub fn get_num_bytes_between_samples(&self) -> i32 {
        
        todo!();
        /*
            return InterleavingType::getNumBytesBetweenSamples (data);
        */
    }

    /** 
      | Returns the accuracy of this format when
      | represented as a 32-bit integer.
      |
      | This is the smallest number above
      | 0 that can be represented in the
      | sample format, converted to a 32-bit
      | range. E,g. if the format is 8-bit,
      | its resolution is 0x01000000; if the
      | format is 24-bit, its resolution is
      | 0x100.
      */
    pub fn get_32bit_resolution() -> i32 {
        
        todo!();
        /*
            return (int) SampleFormat::resolution;
        */
    }

    /**
      | Returns a pointer to the underlying
      | data.
      |
      */
    pub fn get_raw_data(&self)  {
        
        todo!();
        /*
            return data.data;
        */
    }
    
    #[inline] pub fn advance(&mut self)  {
        
        todo!();
        /*
            this->advanceData (data);
        */
    }
}
