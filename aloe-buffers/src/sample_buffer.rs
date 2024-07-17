crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/buffers/aloe_AudioSampleBuffer.h]

/**
  | A multi-channel buffer of 32-bit floating
  | point audio samples.
  |
  | This type is here for backwards compatibility
  | with the older AudioSampleBuffer class, which
  | was fixed for 32-bit data, but is otherwise
  | the same as the new templated AudioBuffer
  | class.
  |
  | @see AudioBuffer
  */
pub type AudioSampleBuffer = AudioBuffer<f32>;

/**
  | A multi-channel buffer containing floating
  | point audio samples.
  |
  | @tags{Audio}
  */
#[leak_detector]
pub struct AudioBuffer<Type> {
    num_channels:               i32, // default = 0
    size:                       i32, // default = 0
    allocated_bytes:            usize, // default = 0
    channels:                   *mut *mut Type,
    allocated_data:             HeapBlock<u8,true>,
    preallocated_channel_space: *mut [Type; 32],
    is_clear:                   bool, // default = false
}

pub trait HasType { 
    type SampleType; 
}

impl<Type> HasType for AudioBuffer<Type> {

    /**
      | This allows templated code that takes
      | an AudioBuffer to access its sample type.
      |
      */
    type SampleType = Type;
}

impl<Type> Default for AudioBuffer<Type> {
    
    /**
      | Creates an empty buffer with 0 channels
      | and 0 length.
      |
      */
    fn default() -> Self {
        todo!();
        /*


            : channels (static_cast<Type**> (preallocatedChannelSpace)
        */
    }
}

impl<Type> AudioBuffer<Type> {

    /** 
      | Creates a buffer with a specified number of
      | channels and samples.
      |
      | The contents of the buffer will initially
      | be undefined, so use clear() to set all
      | the samples to zero.
      |
      | The buffer will allocate its memory
      | internally, and this will be released when
      | the buffer is deleted. If the memory can't
      | be allocated, this will throw
      | a std::bad_alloc exception.
      */
    pub fn new_from_channels_and_samples(
        num_channels_to_allocate: i32,
        num_samples_to_allocate:  i32) -> Self {
    
        todo!();
        /*
        : num_channels(numChannelsToAllocate),
        : size(numSamplesToAllocate),

            jassert (size >= 0 && numChannels >= 0);
            allocateData();
        */
    }

    /**
      | Creates a buffer using a pre-allocated
      | block of memory. 
      |
      | Note that if the buffer
      | is resized or its number of channels
      | is changed, it will re-allocate memory
      | internally and copy the existing data
      | to this new area, so it will then stop
      | directly addressing this memory.
      | 
      | -----------
      | @param dataToReferTo
      | 
      | a pre-allocated array containing pointers
      | to the data for each channel that should
      | be used by this buffer. The buffer will
      | only refer to this memory, it won't try
      | to delete it when the buffer is deleted
      | or resized.
      | ----------
      | @param numChannelsToUse
      | 
      | the number of channels to use - this must
      | correspond to the number of elements
      | in the array passed in
      | ----------
      | @param numSamples
      | 
      | the number of samples to use - this must
      | correspond to the size of the arrays
      | passed in
      |
      */
    pub fn new_from_data_and_channels_and_samples(
        data_to_refer_to:    *mut *const Type,
        num_channels_to_use: i32,
        num_samples:         i32) -> Self {
    
        todo!();
        /*


            : numChannels (numChannelsToUse),
              size (numSamples)

            jassert (dataToReferTo != nullptr);
            jassert (numChannelsToUse >= 0 && numSamples >= 0);
            allocateChannels (dataToReferTo, 0);
        */
    }

    /**
      | Creates a buffer using a pre-allocated
      | block of memory. Note that if the buffer
      | is resized or its number of channels
      | is changed, it will re-allocate memory
      | internally and copy the existing data
      | to this new area, so it will then stop
      | directly addressing this memory.
      | 
      | -----------
      | @param dataToReferTo
      | 
      | a pre-allocated array containing pointers
      | to the data for each channel that should
      | be used by this buffer. The buffer will
      | only refer to this memory, it won't try
      | to delete it when the buffer is deleted
      | or resized.
      | ----------
      | @param numChannelsToUse
      | 
      | the number of channels to use - this must
      | correspond to the number of elements
      | in the array passed in
      | ----------
      | @param startSample
      | 
      | the offset within the arrays at which
      | the data begins
      | ----------
      | @param numSamples
      | 
      | the number of samples to use - this must
      | correspond to the size of the arrays
      | passed in
      |
      */
    pub fn new_from_start(
        data_to_refer_to:    *const *const Type,
        num_channels_to_use: i32,
        start_sample:        i32,
        num_samples:         i32) -> Self {
    
        todo!();
        /*


            : numChannels (numChannelsToUse),
              size (numSamples)

            jassert (dataToReferTo != nullptr);
            jassert (numChannelsToUse >= 0 && startSample >= 0 && numSamples >= 0);
            allocateChannels (dataToReferTo, startSample);
        */
    }

    /**
      | Copies another buffer. This buffer
      | will make its own copy of the other's
      | data, unless the buffer was created
      | using an external data buffer, in which
      | case both buffers will just point to
      | the same shared block of data.
      |
      */
    pub fn new_from_audio_buffer_ref(other: &AudioBuffer<Type>) -> Self {
    
        todo!();
        /*


            : numChannels (other.numChannels),
             size (other.size),
             allocatedBytes (other.allocatedBytes)

            if (allocatedBytes == 0)
            {
                allocateChannels (other.channels, 0);
            }
            else
            {
                allocateData();

                if (other.isClear)
                {
                    clear();
                }
                else
                {
                    for (int i = 0; i < numChannels; ++i)
                        FloatVectorOperations::copy (channels[i], other.channels[i], size);
                }
            }
        */
    }

    /**
      | Copies another buffer onto this one.
      | This buffer's size will be changed to
      | that of the other buffer.
      |
      */
    pub fn assign_from_audio_buffer_ref(&mut self, other: &AudioBuffer<Type>) -> &mut AudioBuffer<Type> {
        
        todo!();
        /*
            if (this != &other)
            {
                setSize (other.getNumChannels(), other.getNumSamples(), false, false, false);

                if (other.isClear)
                {
                    clear();
                }
                else
                {
                    isClear = false;

                    for (int i = 0; i < numChannels; ++i)
                        FloatVectorOperations::copy (channels[i], other.channels[i], size);
                }
            }

            return *this;
        */
    }
    
    pub fn new_from_audio_buffer(other: AudioBuffer<Type>) -> Self {
    
        todo!();
        /*


            : numChannels (other.numChannels),
              size (other.size),
              allocatedBytes (other.allocatedBytes),
              allocatedData (std::move (other.allocatedData)),
              isClear (other.isClear)

            if (numChannels < (int) numElementsInArray (preallocatedChannelSpace))
            {
                channels = preallocatedChannelSpace;

                for (int i = 0; i < numChannels; ++i)
                    preallocatedChannelSpace[i] = other.channels[i];
            }
            else
            {
                channels = other.channels;
            }

            other.numChannels = 0;
            other.size = 0;
            other.allocatedBytes = 0;
        */
    }
    
    pub fn assign_from_audio_buffer(&mut self, other: AudioBuffer<Type>) -> &mut AudioBuffer<Type> {
        
        todo!();
        /*
            numChannels = other.numChannels;
            size = other.size;
            allocatedBytes = other.allocatedBytes;
            allocatedData = std::move (other.allocatedData);
            isClear = other.isClear;

            if (numChannels < (int) numElementsInArray (preallocatedChannelSpace))
            {
                channels = preallocatedChannelSpace;

                for (int i = 0; i < numChannels; ++i)
                    preallocatedChannelSpace[i] = other.channels[i];
            }
            else
            {
                channels = other.channels;
            }

            other.numChannels = 0;
            other.size = 0;
            other.allocatedBytes = 0;
            return *this;
        */
    }

    /**
      | Returns the number of channels of audio
      | data that this buffer contains. 
      | @see getNumSamples, getReadPointer, getWritePointer
      |
      */
    pub fn get_num_channels(&self) -> i32 {
        
        todo!();
        /*
            return numChannels;
        */
    }

    /**
      | Returns the number of samples allocated
      | in each of the buffer's channels. @see
      | getNumChannels, getReadPointer,
      | getWritePointer
      |
      */
    pub fn get_num_samples(&self) -> i32 {
        
        todo!();
        /*
            return size;
        */
    }

    /**
      | Returns a pointer to an array of read-only
      | samples in one of the buffer's channels.
      | For speed, this doesn't check whether
      | the channel number is out of range, so
      | be careful when using it! If you need
      | to write to the data, do NOT call this
      | method and const_cast the result! Instead,
      | you must call getWritePointer so that
      | the buffer knows you're planning on
      | modifying the data.
      |
      */
    pub fn get_read_pointer_from_channel_number(&self, channel_number: i32) -> *const Type {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channelNumber, numChannels));
            return channels[channelNumber];
        */
    }

    /**
      | Returns a pointer to an array of read-only
      | samples in one of the buffer's channels.
      | For speed, this doesn't check whether
      | the channel number or index are out of
      | range, so be careful when using it! If
      | you need to write to the data, do NOT call
      | this method and const_cast the result!
      | Instead, you must call getWritePointer
      | so that the buffer knows you're planning
      | on modifying the data.
      |
      */
    pub fn get_read_pointer_from_channel_number_and_sample_index(&self, 
        channel_number: i32,
        sample_index:   i32) -> *const Type {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channelNumber, numChannels));
            jassert (isPositiveAndBelow (sampleIndex, size));
            return channels[channelNumber] + sampleIndex;
        */
    }

    /** 
      | Returns a writeable pointer to one of the
      | buffer's channels.
      |
      | For speed, this doesn't check whether the
      | channel number is out of range, so be
      | careful when using it!
      |
      | Note that if you're not planning on
      | writing to the data, you should always use
      | getReadPointer instead.
      |
      | This will mark the buffer as not cleared
      | and the hasBeenCleared method will return
      | false after this call. If you retain this
      | write pointer and write some data to the
      | buffer after calling its clear method,
      | subsequent clear calls will do nothing.
      | To avoid this either call this method each
      | time you need to write data, or use the
      | setNotClear method to force the internal
      | cleared flag to false.
      |
      | @see setNotClear
    */
    pub fn get_write_pointer_from_channel_number(&mut self, channel_number: i32) -> *mut Type {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channelNumber, numChannels));
            isClear = false;
            return channels[channelNumber];
        */
    }

    /** 
      | Returns a writeable pointer to one of the
      | buffer's channels.
      |
      | For speed, this doesn't check whether the
      | channel number or index are out of range, so
      | be careful when using it!
      |
      | Note that if you're not planning on writing
      | to the data, you should use getReadPointer
      | instead.
      |
      | This will mark the buffer as not cleared and
      | the hasBeenCleared method will return false
      | after this call. If you retain this write
      | pointer and write some data to the buffer
      | after calling its clear method, subsequent
      | clear calls will do nothing.  To avoid this
      | either call this method each time you need
      | to write data, or use the setNotClear method
      | to force the internal cleared flag to false.
      |
      | @see setNotClear
    */
    pub fn get_write_pointer_from_channel_number_and_sample_index(
        &mut self, 
        channel_number: i32,
        sample_index:   i32) -> *mut Type {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channelNumber, numChannels));
            jassert (isPositiveAndBelow (sampleIndex, size));
            isClear = false;
            return channels[channelNumber] + sampleIndex;
        */
    }

    /**
      | Returns an array of pointers to the channels
      | in the buffer. Don't modify any of the
      | pointers that are returned, and bear
      | in mind that these will become invalid
      | if the buffer is resized.
      |
      */
    pub fn get_array_of_read_pointers(&self) -> *const *const Type {
        
        todo!();
        /*
            return const_cast<const Type**> (channels);
        */
    }

    /** 
      | Returns an array of pointers to the channels
      | in the buffer.
      |
      | Don't modify any of the pointers that are
      | returned, and bear in mind that these will
      | become invalid if the buffer is resized.
      |
      | This will mark the buffer as not cleared and
      | the hasBeenCleared method will return false
      | after this call. If you retain this write
      | pointer and write some data to the buffer
      | after calling its clear method, subsequent
      | clear calls will do nothing.  To avoid this
      | either call this method each time you need
      | to write data, or use the setNotClear method
      | to force the internal cleared flag to false.
      |
      | @see setNotClear
      */
    pub fn get_array_of_write_pointers(&mut self) -> *mut *mut Type {
        
        todo!();
        /*
            isClear = false; return channels;
        */
    }

    /**
      | Changes the buffer's size or number
      | of channels. This can expand or contract
      | the buffer's length, and add or remove
      | channels. Note that if keepExistingContent
      | and avoidReallocating are both true,
      | then it will only avoid reallocating
      | if neither the channel count or length
      | in samples increase. If the required
      | memory can't be allocated, this will
      | throw a std::bad_alloc exception.
      | 
      | -----------
      | @param newNumChannels
      | 
      | the new number of channels.
      | ----------
      | @param newNumSamples
      | 
      | the new number of samples.
      | ----------
      | @param keepExistingContent
      | 
      | if this is true, it will try to preserve
      | as much of the old data as it can in the
      | new buffer.
      | ----------
      | @param clearExtraSpace
      | 
      | if this is true, then any extra channels
      | or space that is allocated will be also
      | be cleared. If false, then this space
      | is left uninitialised.
      | ----------
      | @param avoidReallocating
      | 
      | if this is true, then changing the buffer's
      | size won't reduce the amount of memory
      | that is currently allocated (but it
      | will still increase it if the new size
      | is bigger than the amount it currently
      | has). If this is false, then a new allocation
      | will be done so that the buffer uses takes
      | up the minimum amount of memory that
      | it needs.
      |
      */
    pub fn set_size(
        &mut self, 
        new_num_channels:      i32,
        new_num_samples:       i32,
        keep_existing_content: Option<bool>,
        clear_extra_space:     Option<bool>,
        avoid_reallocating:    Option<bool>)  {

        let keep_existing_content: bool = keep_existing_content.unwrap_or(false);
        let clear_extra_space:     bool = clear_extra_space.unwrap_or(false);
        let avoid_reallocating:    bool = avoid_reallocating.unwrap_or(false);

        todo!();
        /*
            jassert (newNumChannels >= 0);
            jassert (newNumSamples >= 0);

            if (newNumSamples != size || newNumChannels != numChannels)
            {
                auto allocatedSamplesPerChannel = ((size_t) newNumSamples + 3) & ~3u;
                auto channelListSize = ((static_cast<size_t> (1 + newNumChannels) * sizeof (Type*)) + 15) & ~15u;
                auto newTotalBytes = ((size_t) newNumChannels * (size_t) allocatedSamplesPerChannel * sizeof (Type))
                                        + channelListSize + 32;

                if (keepExistingContent)
                {
                    if (avoidReallocating && newNumChannels <= numChannels && newNumSamples <= size)
                    {
                        // no need to do any remapping in this case, as the channel pointers will remain correct!
                    }
                    else
                    {
                        HeapBlock<char, true> newData;
                        newData.allocate (newTotalBytes, clearExtraSpace || isClear);

                        auto numSamplesToCopy = (size_t) jmin (newNumSamples, size);

                        auto newChannels = unalignedPointerCast<Type**> (newData.get());
                        auto newChan     = unalignedPointerCast<Type*> (newData + channelListSize);

                        for (int j = 0; j < newNumChannels; ++j)
                        {
                            newChannels[j] = newChan;
                            newChan += allocatedSamplesPerChannel;
                        }

                        if (! isClear)
                        {
                            auto numChansToCopy = jmin (numChannels, newNumChannels);

                            for (int i = 0; i < numChansToCopy; ++i)
                                FloatVectorOperations::copy (newChannels[i], channels[i], (int) numSamplesToCopy);
                        }

                        allocatedData.swapWith (newData);
                        allocatedBytes = newTotalBytes;
                        channels = newChannels;
                    }
                }
                else
                {
                    if (avoidReallocating && allocatedBytes >= newTotalBytes)
                    {
                        if (clearExtraSpace || isClear)
                            allocatedData.clear (newTotalBytes);
                    }
                    else
                    {
                        allocatedBytes = newTotalBytes;
                        allocatedData.allocate (newTotalBytes, clearExtraSpace || isClear);
                        channels = unalignedPointerCast<Type**> (allocatedData.get());
                    }

                    auto* chan = unalignedPointerCast<Type*> (allocatedData + channelListSize);

                    for (int i = 0; i < newNumChannels; ++i)
                    {
                        channels[i] = chan;
                        chan += allocatedSamplesPerChannel;
                    }
                }

                channels[newNumChannels] = nullptr;
                size = newNumSamples;
                numChannels = newNumChannels;
            }
        */
    }

    /**
      | Makes this buffer point to a pre-allocated
      | set of channel data arrays. There's
      | also a constructor that lets you specify
      | arrays like this, but this lets you change
      | the channels dynamically. Note that
      | if the buffer is resized or its number
      | of channels is changed, it will re-allocate
      | memory internally and copy the existing
      | data to this new area, so it will then
      | stop directly addressing this memory.
      | The hasBeenCleared method will return
      | false after this call.
      | 
      | -----------
      | @param dataToReferTo
      | 
      | a pre-allocated array containing pointers
      | to the data for each channel that should
      | be used by this buffer. The buffer will
      | only refer to this memory, it won't try
      | to delete it when the buffer is deleted
      | or resized.
      | ----------
      | @param newNumChannels
      | 
      | the number of channels to use - this must
      | correspond to the number of elements
      | in the array passed in
      | ----------
      | @param newStartSample
      | 
      | the offset within the arrays at which
      | the data begins
      | ----------
      | @param newNumSamples
      | 
      | the number of samples to use - this must
      | correspond to the size of the arrays
      | passed in
      |
      */
    pub fn set_data_to_refer_to_with_new_start_sample(
        &mut self, 
        data_to_refer_to: *mut *mut Type,
        new_num_channels: i32,
        new_start_sample: i32,
        new_num_samples:  i32
    )  {
        
        todo!();
        /*
            jassert (dataToReferTo != nullptr);
            jassert (newNumChannels >= 0 && newNumSamples >= 0);

            if (allocatedBytes != 0)
            {
                allocatedBytes = 0;
                allocatedData.free();
            }

            numChannels = newNumChannels;
            size = newNumSamples;

            allocateChannels (dataToReferTo, newStartSample);
            jassert (! isClear);
        */
    }

    /**
      | Makes this buffer point to a pre-allocated
      | set of channel data arrays. There's
      | also a constructor that lets you specify
      | arrays like this, but this lets you change
      | the channels dynamically. Note that
      | if the buffer is resized or its number
      | of channels is changed, it will re-allocate
      | memory internally and copy the existing
      | data to this new area, so it will then
      | stop directly addressing this memory.
      | The hasBeenCleared method will return
      | false after this call.
      | 
      | -----------
      | @param dataToReferTo
      | 
      | a pre-allocated array containing pointers
      | to the data for each channel that should
      | be used by this buffer. The buffer will
      | only refer to this memory, it won't try
      | to delete it when the buffer is deleted
      | or resized.
      | ----------
      | @param newNumChannels
      | 
      | the number of channels to use - this must
      | correspond to the number of elements
      | in the array passed in
      | ----------
      | @param newNumSamples
      | 
      | the number of samples to use - this must
      | correspond to the size of the arrays
      | passed in
      |
      */
    pub fn set_data_to_refer_to(&mut self, 
        data_to_refer_to: *mut *mut Type,
        new_num_channels: i32,
        new_num_samples:  i32)  {
        
        todo!();
        /*
            setDataToReferTo (dataToReferTo, newNumChannels, 0, newNumSamples);
        */
    }

    /** 
      | Resizes this buffer to match the given one,
      | and copies all of its content across.
      |
      | The source buffer can contain a different
      | floating point type, so this can be used
      | to convert between 32 and 64 bit float
      | buffer types.
      |
      | The hasBeenCleared method will return
      | false after this call if the other buffer
      | contains data.
    */
    pub fn make_copy_of<OtherType>(
        &mut self, 
        other:              &AudioBuffer<OtherType>,
        avoid_reallocating: Option<bool>)
    {
        let avoid_reallocating: bool =
            avoid_reallocating.unwrap_or(false);

        todo!();
        /*
            setSize (other.getNumChannels(), other.getNumSamples(), false, false, avoidReallocating);

            if (other.hasBeenCleared())
            {
                clear();
            }
            else
            {
                isClear = false;

                for (int chan = 0; chan < numChannels; ++chan)
                {
                    auto* dest = channels[chan];
                    auto* src = other.getReadPointer (chan);

                    for (int i = 0; i < size; ++i)
                        dest[i] = static_cast<Type> (src[i]);
                }
            }
        */
    }
    
    /** 
      | Clears all the samples in all channels and
      | marks the buffer as cleared.
      |
      | This method will do nothing if the buffer
      | has been marked as cleared (i.e. the
      | hasBeenCleared method returns true.)
      |
      | @see hasBeenCleared, setNotClear
      */
    pub fn clear(&mut self)  {

        todo!();
        /*
            if (! isClear)
            {
                for (int i = 0; i < numChannels; ++i)
                    FloatVectorOperations::clear (channels[i], size);

                isClear = true;
            }
        */
    }

    /** 
      | Clears a specified region of all the
      | channels.
      |
      | This will mark the buffer as cleared if
      | the entire buffer contents are cleared.
      |
      | For speed, this doesn't check whether the
      | channel and sample number are in-range, so
      | be careful!
      |
      | This method will do nothing if the buffer
      | has been marked as cleared (i.e. the
      | hasBeenCleared method returns true.)
      |
      | @see hasBeenCleared, setNotClear
      */
    pub fn clear_specified_region_of_all_channels(
        &mut self, 
        start_sample: i32,
        num_samples:  i32)  {
        
        todo!();
        /*
            jassert (startSample >= 0 && numSamples >= 0 && startSample + numSamples <= size);

            if (! isClear)
            {
                for (int i = 0; i < numChannels; ++i)
                    FloatVectorOperations::clear (channels[i] + startSample, numSamples);

                isClear = (startSample == 0 && numSamples == size);
            }
        */
    }

    /** 
      | Clears a specified region of just one
      | channel.
      |
      | For speed, this doesn't check whether the
      | channel and sample number are in-range, so
      | be careful!
      |
      | This method will do nothing if the buffer
      | has been marked as cleared (i.e. the
      | hasBeenCleared method returns true.)
      |
      | @see hasBeenCleared, setNotClear
      */
    pub fn clear_specified_region_of_just_one_channel(
        &mut self, 
        channel:      i32,
        start_sample: i32,
        num_samples:  i32)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channel, numChannels));
            jassert (startSample >= 0 && numSamples >= 0 && startSample + numSamples <= size);

            if (! isClear)
                FloatVectorOperations::clear (channels[channel] + startSample, numSamples);
        */
    }

    /** 
      | Returns true if the buffer has been entirely
      | cleared.
      |
      | Note that this does not actually measure
      | the contents of the buffer - it simply
      | returns a flag that is set when the buffer
      | is cleared, and which is reset whenever
      | functions like getWritePointer are
      | invoked. That means the method is quick,
      | but it may return false negatives when in
      | fact the buffer is still empty.
      */
    pub fn has_been_cleared(&self) -> bool {
        
        todo!();
        /*
            return isClear;
        */
    }

    /** 
      | Forces the internal cleared flag of the
      | buffer to false.
      |
      | This may be useful in the case where you
      | are holding on to a write pointer and call
      | the clear method before writing some
      | data. You can then use this method to mark
      | the buffer as containing data so that
      | subsequent clear calls will
      | succeed. However a better solution is to
      | call getWritePointer each time you need to
      | write data.
      */
    pub fn set_not_clear(&mut self)  {
        
        todo!();
        /*
            isClear = false;
        */
    }

    /** 
      | Returns a sample from the buffer.
      |
      | The channel and index are not checked
      | - they are expected to be in-range. If
      | not, an assertion will be thrown, but in
      | a release build, you're into 'undefined
      | behaviour' territory.
      */
    pub fn get_sample(&self, 
        channel:      i32,
        sample_index: i32) -> Type {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channel, numChannels));
            jassert (isPositiveAndBelow (sampleIndex, size));
            return *(channels[channel] + sampleIndex);
        */
    }

    /** 
      | Sets a sample in the buffer.
      |
      | The channel and index are not checked
      | - they are expected to be in-range. If
      | not, an assertion will be thrown, but in
      | a release build, you're into 'undefined
      | behaviour' territory.
      |
      | The hasBeenCleared method will return
      | false after this call.
      */
    pub fn set_sample(&mut self, 
        dest_channel: i32,
        dest_sample:  i32,
        new_value:    Type)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (destChannel, numChannels));
            jassert (isPositiveAndBelow (destSample, size));
            *(channels[destChannel] + destSample) = newValue;
            isClear = false;
        */
    }

    /** 
      | Adds a value to a sample in the buffer.
      |
      | The channel and index are not checked
      | - they are expected to be in-range. If
      | not, an assertion will be thrown, but in
      | a release build, you're into 'undefined
      | behaviour' territory.
      |
      | The hasBeenCleared method will return
      | false after this call.
      */
    pub fn add_sample(&mut self, 
        dest_channel: i32,
        dest_sample:  i32,
        value_to_add: Type)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (destChannel, numChannels));
            jassert (isPositiveAndBelow (destSample, size));
            *(channels[destChannel] + destSample) += valueToAdd;
            isClear = false;
        */
    }

    /** 
      | Applies a gain multiple to a region of one
      | channel.
      |
      | For speed, this doesn't check whether the
      | channel and sample number are in-range, so
      | be careful!
      */
    pub fn apply_gain_multiple_to_region_of_one_channel(
        &mut self, 
        channel:      i32,
        start_sample: i32,
        num_samples:  i32,
        gain:         Type)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channel, numChannels));
            jassert (startSample >= 0 && numSamples >= 0 && startSample + numSamples <= size);

            if (gain != Type (1) && ! isClear)
            {
                auto* d = channels[channel] + startSample;

                if (gain == Type())
                    FloatVectorOperations::clear (d, numSamples);
                else
                    FloatVectorOperations::multiply (d, gain, numSamples);
            }
        */
    }

    /** 
      | Applies a gain multiple to a region of all
      | the channels.
      |
      | For speed, this doesn't check whether the
      | sample numbers are in-range, so be
      | careful!
      */
    pub fn apply_gain_to_region(&mut self, 
        start_sample: i32,
        num_samples:  i32,
        gain:         Type)  {
        
        todo!();
        /*
            for (int i = 0; i < numChannels; ++i)
                applyGain (i, startSample, numSamples, gain);
        */
    }

    /**
      | Applies a gain multiple to all the audio
      | data.
      |
      */
    pub fn apply_gain(&mut self, gain: Type)  {
        
        todo!();
        /*
            applyGain (0, size, gain);
        */
    }

    /** 
      | Applies a range of gains to a region of
      | a channel.
      |
      | The gain that is applied to each sample
      | will vary from startGain on the first
      | sample to endGain on the last Sample, so
      | it can be used to do basic fades.
      |
      | For speed, this doesn't check whether the
      | sample numbers are in-range, so be
      | careful!
      */
    pub fn apply_gain_ramp_channel(
        &mut self, 
        channel:      i32,
        start_sample: i32,
        num_samples:  i32,
        start_gain:   Type,
        end_gain:     Type)  {
        
        todo!();
        /*
            if (! isClear)
            {
                if (startGain == endGain)
                {
                    applyGain (channel, startSample, numSamples, startGain);
                }
                else
                {
                    jassert (isPositiveAndBelow (channel, numChannels));
                    jassert (startSample >= 0 && numSamples >= 0 && startSample + numSamples <= size);

                    const auto increment = (endGain - startGain) / (float) numSamples;
                    auto* d = channels[channel] + startSample;

                    while (--numSamples >= 0)
                    {
                        *d++ *= startGain;
                        startGain += increment;
                    }
                }
            }
        */
    }

    /** 
      | Applies a range of gains to a region of all
      | channels.
      |
      | The gain that is applied to each sample
      | will vary from startGain on the first
      | sample to endGain on the last Sample, so
      | it can be used to do basic fades.
      |
      | For speed, this doesn't check whether the
      | sample numbers are in-range, so be
      | careful!
      */
    pub fn apply_gain_ramp(
        &mut self, 
        start_sample: i32,
        num_samples:  i32,
        start_gain:   Type,
        end_gain:     Type)  {
        
        todo!();
        /*
            for (int i = 0; i < numChannels; ++i)
                applyGainRamp (i, startSample, numSamples, startGain, endGain);
        */
    }

    /**
      | Adds samples from another buffer to
      | this one. The hasBeenCleared method
      | will return false after this call if
      | samples have been added.
      | 
      | -----------
      | @param destChannel
      | 
      | the channel within this buffer to add
      | the samples to
      | ----------
      | @param destStartSample
      | 
      | the start sample within this buffer's
      | channel
      | ----------
      | @param source
      | 
      | the source buffer to add from
      | ----------
      | @param sourceChannel
      | 
      | the channel within the source buffer
      | to read from
      | ----------
      | @param sourceStartSample
      | 
      | the offset within the source buffer's
      | channel to start reading samples from
      | ----------
      | @param numSamples
      | 
      | the number of samples to process
      | ----------
      | @param gainToApplyToSource
      | 
      | an optional gain to apply to the source
      | samples before they are
      | 
      | added to this buffer's samples
      | 
      | @see copyFrom
      |
      */
    pub fn add_from_source_channel(&mut self, 
        dest_channel:            i32,
        dest_start_sample:       i32,
        source:                  &AudioBuffer<Type>,
        source_channel:          i32,
        source_start_sample:     i32,
        num_samples:             i32,
        gain_to_apply_to_source: Option<Type>)  {

        todo!();
        /*
           let gain_to_apply_to_source: Type = gain_to_apply_to_source.unwrap_or(Type::new(1));

            jassert (&source != this
                     || sourceChannel != destChannel
                     || sourceStartSample + numSamples <= destStartSample
                     || destStartSample + numSamples <= sourceStartSample);
            jassert (isPositiveAndBelow (destChannel, numChannels));
            jassert (destStartSample >= 0 && numSamples >= 0 && destStartSample + numSamples <= size);
            jassert (isPositiveAndBelow (sourceChannel, source.numChannels));
            jassert (sourceStartSample >= 0 && sourceStartSample + numSamples <= source.size);

            if (gainToApplyToSource != 0 && numSamples > 0 && ! source.isClear)
            {
                auto* d = channels[destChannel] + destStartSample;
                auto* s = source.channels[sourceChannel] + sourceStartSample;

                if (isClear)
                {
                    isClear = false;

                    if (gainToApplyToSource != Type (1))
                        FloatVectorOperations::copyWithMultiply (d, s, gainToApplyToSource, numSamples);
                    else
                        FloatVectorOperations::copy (d, s, numSamples);
                }
                else
                {
                    if (gainToApplyToSource != Type (1))
                        FloatVectorOperations::addWithMultiply (d, s, gainToApplyToSource, numSamples);
                    else
                        FloatVectorOperations::add (d, s, numSamples);
                }
            }
        */
    }

    /**
      | Adds samples from an array of floats
      | to one of the channels. The hasBeenCleared
      | method will return false after this
      | call if samples have been added.
      | 
      | -----------
      | @param destChannel
      | 
      | the channel within this buffer to add
      | the samples to
      | ----------
      | @param destStartSample
      | 
      | the start sample within this buffer's
      | channel
      | ----------
      | @param source
      | 
      | the source data to use
      | ----------
      | @param numSamples
      | 
      | the number of samples to process
      | ----------
      | @param gainToApplyToSource
      | 
      | an optional gain to apply to the source
      | samples before they are
      | 
      | added to this buffer's samples
      | 
      | @see copyFrom
      |
      */
    pub fn add_from(&mut self, 
        dest_channel:            i32,
        dest_start_sample:       i32,
        source:                  *const Type,
        num_samples:             i32,
        gain_to_apply_to_source: Option<Type>)  {


        todo!();
        /*
           let gain_to_apply_to_source: Type = gain_to_apply_to_source.unwrap_or(Type::new(1));

            jassert (isPositiveAndBelow (destChannel, numChannels));
            jassert (destStartSample >= 0 && numSamples >= 0 && destStartSample + numSamples <= size);
            jassert (source != nullptr);

            if (gainToApplyToSource != 0 && numSamples > 0)
            {
                auto* d = channels[destChannel] + destStartSample;

                if (isClear)
                {
                    isClear = false;

                    if (gainToApplyToSource != Type (1))
                        FloatVectorOperations::copyWithMultiply (d, source, gainToApplyToSource, numSamples);
                    else
                        FloatVectorOperations::copy (d, source, numSamples);
                }
                else
                {
                    if (gainToApplyToSource != Type (1))
                        FloatVectorOperations::addWithMultiply (d, source, gainToApplyToSource, numSamples);
                    else
                        FloatVectorOperations::add (d, source, numSamples);
                }
            }
        */
    }

    /**
      | Adds samples from an array of floats,
      | applying a gain ramp to them. The hasBeenCleared
      | method will return false after this
      | call if samples have been added.
      | 
      | -----------
      | @param destChannel
      | 
      | the channel within this buffer to add
      | the samples to
      | ----------
      | @param destStartSample
      | 
      | the start sample within this buffer's
      | channel
      | ----------
      | @param source
      | 
      | the source data to use
      | ----------
      | @param numSamples
      | 
      | the number of samples to process
      | ----------
      | @param startGain
      | 
      | the gain to apply to the first sample
      | (this is multiplied with
      | 
      | the source samples before they are added
      | to this buffer)
      | ----------
      | @param endGain
      | 
      | the gain to apply to the final sample.
      | The gain is linearly
      | 
      | interpolated between the first and
      | last samples.
      |
      */
    pub fn add_from_with_ramp(&mut self, 
        dest_channel:      i32,
        dest_start_sample: i32,
        source:            *const Type,
        num_samples:       i32,
        start_gain:        Type,
        end_gain:          Type)  {
        
        todo!();
        /*
            if (startGain == endGain)
            {
                addFrom (destChannel, destStartSample, source, numSamples, startGain);
            }
            else
            {
                jassert (isPositiveAndBelow (destChannel, numChannels));
                jassert (destStartSample >= 0 && numSamples >= 0 && destStartSample + numSamples <= size);
                jassert (source != nullptr);

                if (numSamples > 0)
                {
                    isClear = false;
                    const auto increment = (endGain - startGain) / numSamples;
                    auto* d = channels[destChannel] + destStartSample;

                    while (--numSamples >= 0)
                    {
                        *d++ += startGain * *source++;
                        startGain += increment;
                    }
                }
            }
        */
    }

    /**
      | Copies samples from another buffer
      | to this one.
      | 
      | -----------
      | @param destChannel
      | 
      | the channel within this buffer to copy
      | the samples to
      | ----------
      | @param destStartSample
      | 
      | the start sample within this buffer's
      | channel
      | ----------
      | @param source
      | 
      | the source buffer to read from
      | ----------
      | @param sourceChannel
      | 
      | the channel within the source buffer
      | to read from
      | ----------
      | @param sourceStartSample
      | 
      | the offset within the source buffer's
      | channel to start reading samples from
      | ----------
      | @param numSamples
      | 
      | the number of samples to process
      | 
      | @see addFrom
      |
      */
    pub fn copy_from_source_channel(&mut self, 
        dest_channel:        i32,
        dest_start_sample:   i32,
        source:              &AudioBuffer<Type>,
        source_channel:      i32,
        source_start_sample: i32,
        num_samples:         i32)  {
        
        todo!();
        /*
            jassert (&source != this
                     || sourceChannel != destChannel
                     || sourceStartSample + numSamples <= destStartSample
                     || destStartSample + numSamples <= sourceStartSample);
            jassert (isPositiveAndBelow (destChannel, numChannels));
            jassert (destStartSample >= 0 && destStartSample + numSamples <= size);
            jassert (isPositiveAndBelow (sourceChannel, source.numChannels));
            jassert (sourceStartSample >= 0 && numSamples >= 0 && sourceStartSample + numSamples <= source.size);

            if (numSamples > 0)
            {
                if (source.isClear)
                {
                    if (! isClear)
                        FloatVectorOperations::clear (channels[destChannel] + destStartSample, numSamples);
                }
                else
                {
                    isClear = false;
                    FloatVectorOperations::copy (channels[destChannel] + destStartSample,
                                                 source.channels[sourceChannel] + sourceStartSample,
                                                 numSamples);
                }
            }
        */
    }

    /**
      | Copies samples from an array of floats
      | into one of the channels. The hasBeenCleared
      | method will return false after this
      | call if samples have been copied.
      | 
      | -----------
      | @param destChannel
      | 
      | the channel within this buffer to copy
      | the samples to
      | ----------
      | @param destStartSample
      | 
      | the start sample within this buffer's
      | channel
      | ----------
      | @param source
      | 
      | the source buffer to read from
      | ----------
      | @param numSamples
      | 
      | the number of samples to process
      | 
      | @see addFrom
      |
      */
    pub fn copy_from(&mut self, 
        dest_channel:      i32,
        dest_start_sample: i32,
        source:            *const Type,
        num_samples:       i32)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (destChannel, numChannels));
            jassert (destStartSample >= 0 && numSamples >= 0 && destStartSample + numSamples <= size);
            jassert (source != nullptr);

            if (numSamples > 0)
            {
                isClear = false;
                FloatVectorOperations::copy (channels[destChannel] + destStartSample, source, numSamples);
            }
        */
    }

    /**
      | Copies samples from an array of floats
      | into one of the channels, applying a
      | gain to it. The hasBeenCleared method
      | will return false after this call if
      | samples have been copied.
      | 
      | -----------
      | @param destChannel
      | 
      | the channel within this buffer to copy
      | the samples to
      | ----------
      | @param destStartSample
      | 
      | the start sample within this buffer's
      | channel
      | ----------
      | @param source
      | 
      | the source buffer to read from
      | ----------
      | @param numSamples
      | 
      | the number of samples to process
      | ----------
      | @param gain
      | 
      | the gain to apply
      | 
      | @see addFrom
      |
      */
    pub fn copy_from_with_gain(&mut self, 
        dest_channel:      i32,
        dest_start_sample: i32,
        source:            *const Type,
        num_samples:       i32,
        gain:              Type)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (destChannel, numChannels));
            jassert (destStartSample >= 0 && numSamples >= 0 && destStartSample + numSamples <= size);
            jassert (source != nullptr);

            if (numSamples > 0)
            {
                auto* d = channels[destChannel] + destStartSample;

                if (gain != Type (1))
                {
                    if (gain == Type())
                    {
                        if (! isClear)
                            FloatVectorOperations::clear (d, numSamples);
                    }
                    else
                    {
                        isClear = false;
                        FloatVectorOperations::copyWithMultiply (d, source, gain, numSamples);
                    }
                }
                else
                {
                    isClear = false;
                    FloatVectorOperations::copy (d, source, numSamples);
                }
            }
        */
    }

    /**
      | Copies samples from an array of floats
      | into one of the channels, applying a
      | gain ramp. The hasBeenCleared method
      | will return false after this call if
      | samples have been copied.
      | 
      | -----------
      | @param destChannel
      | 
      | the channel within this buffer to copy
      | the samples to
      | ----------
      | @param destStartSample
      | 
      | the start sample within this buffer's
      | channel
      | ----------
      | @param source
      | 
      | the source buffer to read from
      | ----------
      | @param numSamples
      | 
      | the number of samples to process
      | ----------
      | @param startGain
      | 
      | the gain to apply to the first sample
      | (this is multiplied with
      | 
      | the source samples before they are copied
      | to this buffer)
      | ----------
      | @param endGain
      | 
      | the gain to apply to the final sample.
      | The gain is linearly
      | 
      | interpolated between the first and
      | last samples.
      | 
      | @see addFrom
      |
      */
    pub fn copy_from_with_ramp(&mut self, 
        dest_channel:      i32,
        dest_start_sample: i32,
        source:            *const Type,
        num_samples:       i32,
        start_gain:        Type,
        end_gain:          Type)  {
        
        todo!();
        /*
            if (startGain == endGain)
            {
                copyFrom (destChannel, destStartSample, source, numSamples, startGain);
            }
            else
            {
                jassert (isPositiveAndBelow (destChannel, numChannels));
                jassert (destStartSample >= 0 && numSamples >= 0 && destStartSample + numSamples <= size);
                jassert (source != nullptr);

                if (numSamples > 0)
                {
                    isClear = false;
                    const auto increment = (endGain - startGain) / numSamples;
                    auto* d = channels[destChannel] + destStartSample;

                    while (--numSamples >= 0)
                    {
                        *d++ = startGain * *source++;
                        startGain += increment;
                    }
                }
            }
        */
    }

    /**
      | Returns a Range indicating the lowest
      | and highest sample values in a given
      | section.
      | 
      | -----------
      | @param channel
      | 
      | the channel to read from
      | ----------
      | @param startSample
      | 
      | the start sample within the channel
      | ----------
      | @param numSamples
      | 
      | the number of samples to check
      |
      */
    pub fn find_min_max(&self, 
        channel:      i32,
        start_sample: i32,
        num_samples:  i32) -> Range<Type> {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channel, numChannels));
            jassert (startSample >= 0 && numSamples >= 0 && startSample + numSamples <= size);

            if (isClear)
                return { Type (0), Type (0) };

            return FloatVectorOperations::findMinAndMax (channels[channel] + startSample, numSamples);
        */
    }

    /**
      | Finds the highest absolute sample value
      | within a region of a channel.
      |
      */
    pub fn get_magnitude_within_a_region_of_a_channel(
        &self, 
        channel:      i32,
        start_sample: i32,
        num_samples:  i32) -> Type {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channel, numChannels));
            jassert (startSample >= 0 && numSamples >= 0 && startSample + numSamples <= size);

            if (isClear)
                return Type (0);

            auto r = findMinMax (channel, startSample, numSamples);

            return jmax (r.getStart(), -r.getStart(), r.getEnd(), -r.getEnd());
        */
    }

    /**
      | Finds the highest absolute sample value
      | within a region on all channels.
      |
      */
    pub fn get_magnitude(
        &self, 
        start_sample: i32,
        num_samples:  i32) -> Type {
        
        todo!();
        /*
            Type mag (0);

            if (! isClear)
                for (int i = 0; i < numChannels; ++i)
                    mag = jmax (mag, getMagnitude (i, startSample, numSamples));

            return mag;
        */
    }

    /**
      | Returns the root mean squared level
      | for a region of a channel.
      |
      */
    pub fn get_rms_level(&self, 
        channel:      i32,
        start_sample: i32,
        num_samples:  i32) -> Type {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channel, numChannels));
            jassert (startSample >= 0 && numSamples >= 0 && startSample + numSamples <= size);

            if (numSamples <= 0 || channel < 0 || channel >= numChannels || isClear)
                return Type (0);

            auto* data = channels[channel] + startSample;
            double sum = 0.0;

            for (int i = 0; i < numSamples; ++i)
            {
                auto sample = data[i];
                sum += sample * sample;
            }

            return static_cast<Type> (std::sqrt (sum / numSamples));
        */
    }

    /**
      | Reverses a part of a channel.
      |
      */
    pub fn reverse_part_of_a_channel(
        &self, 
        channel:      i32,
        start_sample: i32,
        num_samples:  i32)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channel, numChannels));
            jassert (startSample >= 0 && numSamples >= 0 && startSample + numSamples <= size);

            if (! isClear)
                std::reverse (channels[channel] + startSample,
                              channels[channel] + startSample + numSamples);
        */
    }

    /**
      | Reverses a part of the buffer.
      |
      */
    pub fn reverse(
        &self, 
        start_sample: i32,
        num_samples:  i32)  {
        
        todo!();
        /*
            for (int i = 0; i < numChannels; ++i)
                reverse (i, startSample, numSamples);
        */
    }
    
    pub fn allocate_data(&mut self)  {
        
        todo!();
        /*
            #if (! ALOE_GCC || (__GNUC__ * 100 + __GNUC_MINOR__) >= 409)
            static_assert (alignof (Type) <= detail::maxAlignment,
                           "AudioBuffer cannot hold types with alignment requirements larger than that guaranteed by malloc");
           #endif
            jassert (size >= 0);

            auto channelListSize = (size_t) (numChannels + 1) * sizeof (Type*);
            auto requiredSampleAlignment = std::alignment_of<Type>::value;
            size_t alignmentOverflow = channelListSize % requiredSampleAlignment;

            if (alignmentOverflow != 0)
                channelListSize += requiredSampleAlignment - alignmentOverflow;

            allocatedBytes = (size_t) numChannels * (size_t) size * sizeof (Type) + channelListSize + 32;
            allocatedData.malloc (allocatedBytes);
            channels = unalignedPointerCast<Type**> (allocatedData.get());
            auto chan = unalignedPointerCast<Type*> (allocatedData + channelListSize);

            for (int i = 0; i < numChannels; ++i)
            {
                channels[i] = chan;
                chan += size;
            }

            channels[numChannels] = nullptr;
            isClear = false;
        */
    }
    
    pub fn allocate_channels(&mut self, 
        data_to_refer_to: *const *const Type,
        offset:           i32)  {
        
        todo!();
        /*
            jassert (offset >= 0);

            // (try to avoid doing a malloc here, as that'll blow up things like Pro-Tools)
            if (numChannels < (int) numElementsInArray (preallocatedChannelSpace))
            {
                channels = static_cast<Type**> (preallocatedChannelSpace);
            }
            else
            {
                allocatedData.malloc (numChannels + 1, sizeof (Type*));
                channels = unalignedPointerCast<Type**> (allocatedData.get());
            }

            for (int i = 0; i < numChannels; ++i)
            {
                // you have to pass in the same number of valid pointers as numChannels
                jassert (dataToReferTo[i] != nullptr);
                channels[i] = dataToReferTo[i] + offset;
            }

            channels[numChannels] = nullptr;
            isClear = false;
        */
    }
}
