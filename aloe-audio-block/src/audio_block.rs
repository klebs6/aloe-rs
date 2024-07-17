crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/containers/aloe_AudioBlock.h]

/**
  | Minimal and lightweight data-structure
  | which contains a list of pointers to
  | channels containing some kind of sample
  | data.
  | 
  | This class doesn't own any of the data
  | which it points to, it's simply a view
  | into data that is owned elsewhere. You
  | can construct one from some raw data
  | that you've allocated yourself, or
  | give it a HeapBlock to use, or give it
  | an AudioBuffer which it can refer to,
  | but in all cases the user is responsible
  | for making sure that the data doesn't
  | get deleted while there's still an
  | AudioBlock using it.
  | 
  | @tags{DSP}
  |
  | AudioBlock only supports single or double precision
  | floating point types
  |
  */
pub struct AudioBlock<SampleType> 
where SampleType: SampleTypeInterface
{
    channels:     *mut *mut SampleType,
    num_channels: usize, // default = 0
    start_sample: usize, // default = 0
    num_samples:  usize, // default = 0
}

pub trait HasChannelCountType {
    type ChannelCountType;
}

impl<T> HasChannelCountType for AudioBlock<T> 
where T: SampleTypeInterface
{
    type ChannelCountType = usize;
}

impl<T> HasNumericType for AudioBlock<T> 
where T: SampleTypeInterface
{
    type Type = <T as HasElementType>::Type;
}

impl<T,U> From<&AudioBlock<U>> for AudioBlock<T> 

where T: SampleTypeInterface + From<U>,
      U: SampleTypeInterface,
{
    fn from(other: &AudioBlock<U>) -> Self {
        todo!();
    }
}

impl<SampleType: SampleTypeInterface> Default for AudioBlock<SampleType> 
where SampleType: SampleTypeInterface
{
    /**
      | Create a zero-sized AudioBlock.
      |
      */
    fn default() -> Self {
        todo!();
    }
}

impl<SampleType: SampleTypeInterface> AddAssign<&<Self as HasNumericType>::Type> for AudioBlock<SampleType> 
    where SampleType: SampleTypeInterface
{
    
    #[ALOE_VECTOR_CALLTYPE]
    #[inline]fn add_assign(&mut self, other: &<Self as HasNumericType>::Type) {
        todo!();
        /*
            return add (value);
        */
    }
}

/*
impl<SampleType: SampleTypeInterface> 
    AddAssign<AudioBlock<SampleType>> for AudioBlock<SampleType> 
    where SampleType: SampleTypeInterface
{
    
    #[inline]fn add_assign(&mut self, other: &AudioBlock<SampleType>) {
        todo!();
        /*
            return add (src);
        */
    }
}
*/

impl<SampleType: SampleTypeInterface> SubAssign<&AudioBlock<SampleType>> for AudioBlock<SampleType> 
    where SampleType: SampleTypeInterface
{
    #[inline]fn sub_assign(&mut self, other: &AudioBlock<SampleType>) {
        todo!();
        /*
            return subtract (src);
        */
    }
}

impl<SampleType: SampleTypeInterface> MulAssign<AudioBlock<SampleType>> for AudioBlock<SampleType> 
    where SampleType: SampleTypeInterface
{
    #[inline] fn mul_assign(&mut self, src: AudioBlock<SampleType>) {
        todo!();
        /*
            return multiplyBy (src);
        */
    }
}

impl<SampleType: SampleTypeInterface,OtherSampleType: FloatSample,SmoothingType> 
    MulAssign<&mut SmoothedValue<OtherSampleType,SmoothingType>> 
    for AudioBlock<SampleType> 
    where SampleType: SampleTypeInterface,
          OtherSampleType: SampleTypeInterface
{
    #[inline] fn mul_assign(&mut self, value: &mut SmoothedValue<OtherSampleType,SmoothingType>) {
        todo!();
        /*
            return multiplyBy (value);
        */
    }
}

pub fn audio_block_size_factor<SampleType: SampleTypeInterface>()  -> usize {

    size_of::<SampleType>() / size_of::<<AudioBlock<SampleType> as HasNumericType>::Type>() 
}

pub fn audio_block_element_mask<SampleType: SampleTypeInterface>() -> usize {

    audio_block_size_factor::<SampleType>() - 1
}

pub fn audio_block_byte_mask<SampleType: SampleTypeInterface>() -> usize { 

    let sample_type_size_factor = audio_block_size_factor::<SampleType>();
    let numeric_type_size       = size_of::<<AudioBlock<SampleType> as HasNumericType>::Type>();

    ( sample_type_size_factor * numeric_type_size ) - 1
}

pub fn audio_block_default_alignment<SampleType>() -> usize { 

    size_of::<SampleType>()

    /*
   #if ALOE_USE_SIMD
    static constexpr size_t defaultAlignment = sizeof (SIMDRegister<NumericType>);
   #else
    static constexpr size_t defaultAlignment = sizeof (NumericType);
   #endif
    */
}

impl<SampleType: SampleTypeInterface> AudioBlock<SampleType> 
where SampleType: SampleTypeInterface
{

    /**
      | Creates an AudioBlock from a pointer
      | to an array of channels.
      | 
      | AudioBlock does not copy nor own the
      | memory pointed to by dataToUse.
      | 
      | Therefore it is the user's responsibility
      | to ensure that the memory is retained
      | throughout the life-time of the AudioBlock
      | and released when no longer needed.
      |
      */
    pub fn new_from_channel_data(
        channel_data:       *mut *const SampleType,
        number_of_channels: usize,
        number_of_samples:  usize) -> Self {
    
        todo!();
        /*
            : channels (channelData),
              numChannels (static_cast<ChannelCountType> (numberOfChannels)),
              numSamples (numberOfSamples)
        */
    }
    
    /**
      | Creates an AudioBlock from a pointer
      | to an array of channels.
      | 
      | AudioBlock does not copy nor own the
      | memory pointed to by dataToUse.
      | 
      | Therefore it is the user's responsibility
      | to ensure that the memory is retained
      | throughout the life-time of the AudioBlock
      | and released when no longer needed.
      |
      */
    pub fn new_from_pointer_to_array_of_channels(
        channel_data:       *mut *const SampleType,
        number_of_channels: usize,
        start_sample_index: usize,
        number_of_samples:  usize) -> Self {
    
        todo!();
        /*


            : channels (channelData),
              numChannels (static_cast<ChannelCountType> (numberOfChannels)),
              startSample (startSampleIndex),
              numSamples (numberOfSamples)
        */
    }
    
    /**
      | Allocates a suitable amount of space
      | in a HeapBlock, and initialises this
      | object to point into it.
      | 
      | The HeapBlock must of course not be freed
      | or re-allocated while this object is
      | still in use, because it will be referencing
      | its data.
      |
      */
    pub fn new_from_heap_block_allocation(
        heap_block_to_use_for_allocation: &mut HeapBlock<u8>,
        number_of_channels:               usize,
        number_of_samples:                usize,
        alignment_in_bytes:               Option<usize>

    ) -> Self {

        let alignment_in_bytes: usize =
                 alignment_in_bytes.unwrap_or(audio_block_default_alignment::<SampleType>());

        todo!();

        /*


            : numChannels (static_cast<ChannelCountType> (numberOfChannels)),
              numSamples (numberOfSamples)

            auto roundedUpNumSamples = (numberOfSamples + elementMask) & ~elementMask;
            auto channelSize = sizeof (SampleType) * roundedUpNumSamples;
            auto channelListBytes = sizeof (SampleType*) * numberOfChannels;
            auto extraBytes = alignmentInBytes - 1;

            heapBlockToUseForAllocation.malloc (channelListBytes + extraBytes + channelSize * numberOfChannels);

            auto* chanArray = unalignedPointerCast<SampleType**> (heapBlockToUseForAllocation.getData());
            channels = chanArray;

            auto* data = unalignedPointerCast<SampleType*> (addBytesToPointer (chanArray, channelListBytes));
            data = snapPointerToAlignment (data, alignmentInBytes);

            for (ChannelCountType i = 0; i < numChannels; ++i)
            {
                chanArray[i] = data;
                data += roundedUpNumSamples;
            }
        */
    }

    /**
      | Creates an AudioBlock that points to
      | the data in an AudioBuffer.
      | 
      | AudioBlock does not copy nor own the
      | memory pointed to by dataToUse.
      | 
      | Therefore it is the user's responsibility
      | to ensure that the buffer is retained
      | throughout the life-time of the AudioBlock
      | without being modified.
      |
      */
    pub fn new_from_other_sample_buffer_mut<OtherSampleType: SampleTypeInterface>(buffer: &mut AudioBuffer<OtherSampleType>) -> Self {
    
        todo!();
        /*


            : channels (buffer.getArrayOfWritePointers()),
              numChannels (static_cast<ChannelCountType> (buffer.getNumChannels())),
              numSamples (static_cast<size_t> (buffer.getNumSamples()))
        */
    }

    /**
      | Creates an AudioBlock that points to
      | the data in an AudioBuffer.
      | 
      | AudioBlock does not copy nor own the
      | memory pointed to by dataToUse.
      | 
      | Therefore it is the user's responsibility
      | to ensure that the buffer is retained
      | throughout the life-time of the AudioBlock
      | without being modified.
      |
      */
    pub fn new_from_other_sample_buffer<OtherSampleType: SampleTypeInterface>(buffer: &AudioBuffer<OtherSampleType>) -> Self {
    
        todo!();
        /*


            : channels (buffer.getArrayOfReadPointers()),
              numChannels (static_cast<ChannelCountType> (buffer.getNumChannels())),
              numSamples (static_cast<size_t> (buffer.getNumSamples()))
        */
    }

    /**
      | Creates an AudioBlock that points to
      | the data in an AudioBuffer.
      | 
      | AudioBlock does not copy nor own the
      | memory pointed to by dataToUse.
      | 
      | Therefore it is the user's responsibility
      | to ensure that the buffer is retained
      | throughout the life-time of the AudioBlock
      | without being modified.
      |
      */
    pub fn new_from_other_sample_buffer_mut_and_start_index<OtherSampleType: SampleTypeInterface>(
        buffer:             &mut AudioBuffer<OtherSampleType>,
        start_sample_index: usize) -> Self {
    
        todo!();
        /*


            : channels (buffer.getArrayOfWritePointers()),
              numChannels (static_cast<ChannelCountType> (buffer.getNumChannels())),
              startSample (startSampleIndex),
              numSamples (static_cast<size_t> (buffer.getNumSamples()) - startSampleIndex)
            jassert (startSample < static_cast<size_t> (buffer.getNumSamples()));
        */
    }
    
    pub fn new_from_other_audio_block<OtherSampleType: SampleTypeInterface>(other: &AudioBlock<OtherSampleType>) -> Self {
    
        todo!();
        /*


            : channels { other.channels },
              numChannels { other.numChannels },
              startSample { other.startSample },
              numSamples { other.numSamples }
        */
    }
    
    pub fn assign_from<OtherSampleType: SampleTypeInterface>(&mut self, other: &AudioBlock<OtherSampleType>) -> &mut AudioBlock<SampleType> {
        
        todo!();
        /*
            AudioBlock blockCopy { other };
            swap (blockCopy);
            return *this;
        */
    }
    
    pub fn swap(&mut self, other: &mut AudioBlock<SampleType>)  {
        
        todo!();
        /*
            std::swap (other.channels, channels);
            std::swap (other.numChannels, numChannels);
            std::swap (other.startSample, startSample);
            std::swap (other.numSamples, numSamples);
        */
    }
    
    pub fn operator_equals<OtherSampleType: SampleTypeInterface>(&self, other: &AudioBlock<OtherSampleType>) -> bool {
        
        todo!();
        /*
            return std::equal (channels,
                               channels + numChannels,
                               other.channels,
                               other.channels + other.numChannels)
                   && startSample == other.startSample
                   && numSamples == other.numSamples;
        */
    }
    
    /**
      | Returns the number of channels referenced
      | by this block.
      |
      */
    pub fn get_num_channels(&self) -> usize {
        
        todo!();
        /*
            return static_cast<size_t> (numChannels);
        */
    }

    /**
      | Returns the number of samples referenced
      | by this block.
      |
      */
    pub fn get_num_samples(&self) -> usize {
        
        todo!();
        /*
            return numSamples;
        */
    }

    /**
      | Returns a raw pointer into one of the
      | channels in this block.
      |
      */
    pub fn get_channel_pointer(&self, channel: usize) -> *mut SampleType {
        
        todo!();
        /*
            jassert (channel < numChannels);
            jassert (numSamples > 0);
            return channels[channel] + startSample;
        */
    }

    /**
      | Returns an AudioBlock that represents
      | one of the channels in this block.
      |
      */
    pub fn get_single_channel_block(&self, channel: usize) -> AudioBlock<SampleType> {
        
        todo!();
        /*
            jassert (channel < numChannels);
            return AudioBlock (channels + channel, 1, startSample, numSamples);
        */
    }

    /**
      | Returns a subset of contiguous channels
      | 
      | -----------
      | @param channelStart
      | 
      | First channel of the subset
      | ----------
      | @param numChannelsToUse
      | 
      | Count of channels in the subset
      |
      */
    pub fn get_subset_channel_block(&self, 
        channel_start:       usize,
        num_channels_to_use: usize) -> AudioBlock<SampleType> {
        
        todo!();
        /*
            jassert (channelStart < numChannels);
            jassert ((channelStart + numChannelsToUse) <= numChannels);

            return AudioBlock (channels + channelStart, numChannelsToUse, startSample, numSamples);
        */
    }

    /**
      | Returns a sample from the buffer.
      | 
      | The channel and index are not checked
      | - they are expected to be in-range. If
      | not, an assertion will be thrown, but
      | in a release build, you're into 'undefined
      | behaviour' territory.
      |
      */
    pub fn get_sample(&self, 
        channel:      i32,
        sample_index: i32) -> SampleType {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (channel, numChannels));
            jassert (isPositiveAndBelow (sampleIndex, numSamples));
            return channels[channel][(size_t) startSample + (size_t) sampleIndex];
        */
    }

    /**
      | Modifies a sample in the buffer.
      | 
      | The channel and index are not checked
      | - they are expected to be in-range. If
      | not, an assertion will be thrown, but
      | in a release build, you're into 'undefined
      | behaviour' territory.
      |
      */
    pub fn set_sample(&self, 
        dest_channel: i32,
        dest_sample:  i32,
        new_value:    SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (destChannel, numChannels));
            jassert (isPositiveAndBelow (destSample, numSamples));
            channels[destChannel][(size_t) startSample + (size_t) destSample] = newValue;
        */
    }

    /**
      | Adds a value to a sample in the buffer.
      | 
      | The channel and index are not checked
      | - they are expected to be in-range. If
      | not, an assertion will be thrown, but
      | in a release build, you're into 'undefined
      | behaviour' territory.
      |
      */
    pub fn add_sample(&self, 
        dest_channel: i32,
        dest_sample:  i32,
        value_to_add: SampleType)  {
        
        todo!();
        /*
            jassert (isPositiveAndBelow (destChannel, numChannels));
            jassert (isPositiveAndBelow (destSample, numSamples));
            channels[destChannel][(size_t) startSample + (size_t) destSample] += valueToAdd;
        */
    }
    
    /**
      | Clears the memory referenced by this
      | AudioBlock.
      |
      */
    pub fn clear_mut(&mut self) -> &mut AudioBlock<SampleType> {
        
        todo!();
        /*
            clearInternal(); return *this;
        */
    }
    
    pub fn clear(&self) -> &AudioBlock<SampleType> {
        
        todo!();
        /*
            clearInternal(); return *this;
        */
    }

    /**
      | Fills the memory referenced by this
      | AudioBlock with value.
      |
      */
    #[ALOE_VECTOR_CALLTYPE]
    pub fn fill_mut(&mut self, value: <Self as HasNumericType>::Type) -> &mut AudioBlock<SampleType> {
        
        todo!();
        /*
            fillInternal (value); return *this;
        */
    }
    
    #[ALOE_VECTOR_CALLTYPE]
    pub fn fill(&self, value: <Self as HasNumericType>::Type) -> &AudioBlock<SampleType> {
        
        todo!();
        /*
            fillInternal (value); return *this;
        */
    }

    /**
      | Copies the values in src to this block.
      |
      */
    pub fn copy_from_mut<OtherSampleType: SampleTypeInterface>(&mut self, src: &AudioBlock<OtherSampleType>) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            copyFromInternal (src); return *this;
        */
    }
    
    pub fn copy_from<OtherSampleType: SampleTypeInterface>(&self, src: &AudioBlock<OtherSampleType>) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            copyFromInternal (src); return *this;
        */
    }

    /**
      | Copy the values from an AudioBuffer
      | to this block.
      | 
      | All indices and sizes are in this AudioBlock's
      | units, i.e. if SampleType is a
      | SIMDRegister then incrementing srcPos
      | by one will increase the sample position
      | in the AudioBuffer's units by a factor
      | of SIMDRegister<SampleType>::SIMDNumElements.
      |
      */
    pub fn copy_from_audio_buffer_mut<OtherNumericType>(
        &mut self, 
        src:          &AudioBuffer<OtherNumericType>,
        src_pos:      Option<usize>,
        dst_pos:      Option<usize>,
        num_elements: Option<usize>

    ) -> &mut AudioBlock<SampleType> {

        let src_pos:      usize = src_pos.unwrap_or(0);
        let dst_pos:      usize = dst_pos.unwrap_or(0);
        let num_elements: usize = num_elements.unwrap_or(usize::MAX);

        todo!();
        /*
            copyFromInternal (src, srcPos, dstPos, numElements); return *this;
        */
    }
    
    pub fn copy_from_audio_buffer<OtherNumericType>(
        &self, 
        src:          &AudioBuffer<OtherNumericType>,
        src_pos:      Option<usize>,
        dst_pos:      Option<usize>,
        num_elements: Option<usize>

    ) -> &AudioBlock<SampleType> {

        let src_pos:      usize = src_pos.unwrap_or(0);
        let dst_pos:      usize = dst_pos.unwrap_or(0);
        let num_elements: usize = num_elements.unwrap_or(usize::MAX);
        todo!();
        /*
            copyFromInternal (src, srcPos, dstPos, numElements); return *this;
        */
    }

    /**
      | Copies the values from this block to
      | an AudioBuffer.
      | 
      | All indices and sizes are in this AudioBlock's
      | units, i.e. if SampleType is a
      | 
      | SIMDRegister then incrementing dstPos
      | by one will increase the sample position
      | in the AudioBuffer's units by a factor
      | of SIMDRegister<SampleType>::SIMDNumElements.
      |
      */
    pub fn copy_to(
        &self, 
        dst:          &mut AudioBuffer<<Self as HasNumericType>::Type>,
        src_pos:      Option<usize>,
        dst_pos:      Option<usize>,
        num_elements: Option<usize>

    )  {

        let src_pos:      usize = src_pos.unwrap_or(0);
        let dst_pos:      usize = dst_pos.unwrap_or(0);
        let num_elements: usize = num_elements.unwrap_or(usize::MAX);

        todo!();
        /*
            auto dstlen = static_cast<size_t> (dst.getNumSamples()) / sizeFactor;
            auto n = static_cast<int> (jmin (numSamples - srcPos, dstlen - dstPos, numElements) * sizeFactor);
            auto maxChannels = jmin (static_cast<size_t> (dst.getNumChannels()), static_cast<size_t> (numChannels));

            for (size_t ch = 0; ch < maxChannels; ++ch)
                FloatVectorOperations::copy (dst.getWritePointer (static_cast<int> (ch),
                                                                  static_cast<int> (dstPos * sizeFactor)),
                                             getDataPointer (ch) + (srcPos * sizeFactor),
                                             n);
        */
    }

    /**
      | Move memory within this block from the
      | position srcPos to the position dstPos.
      | 
      | If numElements is not specified then
      | move will move the maximum amount of
      | memory.
      |
      */
    pub fn move_mut(
        &mut self, 
        src_pos:      usize,
        dst_pos:      usize,
        num_elements: Option<usize>

    ) -> &mut AudioBlock<SampleType> {

        let num_elements: usize = num_elements.unwrap_or(usize::MAX);

        todo!();
        /*
            moveInternal (srcPos, dstPos, numElements); return *this;
        */
    }
    
    pub fn move_(
        &self, 
        src_pos:      usize,
        dst_pos:      usize,
        num_elements: Option<usize>

    ) -> &AudioBlock<SampleType> {

        let num_elements: usize = num_elements.unwrap_or(usize::MAX);

        todo!();
        /*
            moveInternal (srcPos, dstPos, numElements); return *this;
        */
    }
    
    /**
      | Return a new AudioBlock pointing to
      | a sub-block inside this block. This
      | function does not copy the memory and
      | you must ensure that the original memory
      | pointed to by the receiver remains valid
      | through-out the life-time of the returned
      | sub-block.
      | 
      | -----------
      | @param newOffset
      | 
      | The index of an element inside the receiver
      | which will will become the first element
      | of the return value.
      | ----------
      | @param newLength
      | 
      | The number of elements of the newly created
      | sub-block.
      |
      */
    pub fn get_sub_block_with_new_offset_and_length(
        &self, 
        new_offset: usize,
        new_length: usize

    ) -> AudioBlock<SampleType> {
        
        todo!();
        /*
            jassert (newOffset < numSamples);
            jassert (newOffset + newLength <= numSamples);

            return AudioBlock (channels, numChannels, startSample + newOffset, newLength);
        */
    }

    /**
      | Return a new AudioBlock pointing to
      | a sub-block inside this block. This
      | function does not copy the memory and
      | you must ensure that the original memory
      | pointed to by the receiver remains valid
      | through-out the life-time of the returned
      | sub-block.
      | 
      | -----------
      | @param newOffset
      | 
      | The index of an element inside the block
      | which will will become the first element
      | of the return value.
      | 
      | The return value will include all subsequent
      | elements of the receiver.
      |
      */
    pub fn get_sub_block(&self, new_offset: usize) -> AudioBlock<SampleType> {
        
        todo!();
        /*
            return getSubBlock (newOffset, getNumSamples() - newOffset);
        */
    }
    
    /**
      | Adds a fixed value to the elements in
      | this block.
      |
      */
    #[ALOE_VECTOR_CALLTYPE]
    pub fn add_mut(&mut self, value: <Self as HasNumericType>::Type) -> &mut AudioBlock<SampleType> {
        
        todo!();
        /*
            addInternal (value); return *this;
        */
    }

    #[ALOE_VECTOR_CALLTYPE]
    pub fn add(&self, value: <Self as HasNumericType>::Type) -> &AudioBlock<SampleType> {
        
        todo!();
        /*
            addInternal (value); return *this;
        */
    }

    /**
      | Adds the elements in the src block to
      | the elements in this block.
      |
      */
    pub fn add_elements_in_the_source_block_mut<OtherSampleType: SampleTypeInterface>(
        &mut self, 
        src: AudioBlock<OtherSampleType>

    ) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            addInternal (src); return *this;
        */
    }
    
    pub fn add_elements_in_the_source_block<OtherSampleType: SampleTypeInterface>(
        &self, 
        src: AudioBlock<OtherSampleType>

    ) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            addInternal (src); return *this;
        */
    }

    /**
      | Adds a fixed value to each source value
      | and replaces the contents of this block.
      |
      */
    #[ALOE_VECTOR_CALLTYPE]
    pub fn replace_with_sum_of_fixed_value_and_source_value_mut<OtherSampleType: SampleTypeInterface>(
        &mut self, 
        src:   AudioBlock<OtherSampleType>,
        value: <Self as HasNumericType>::Type) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithSumOfInternal (src, value); return *this;
        */
    }

    #[ALOE_VECTOR_CALLTYPE]
    pub fn replace_with_sum_of_fixed_value_and_source_value<OtherSampleType: SampleTypeInterface>(
        &self, 
        src:   AudioBlock<OtherSampleType>,
        value: <Self as HasNumericType>::Type) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithSumOfInternal (src, value); return *this;
        */
    }

    /**
      | Adds each source1 value to the corresponding
      | source2 value and replaces the contents
      | of this block.
      |
      */
    pub fn replace_with_sum_of_source_values_mut<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(
        &mut self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>

    ) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithSumOfInternal (src1, src2); return *this;
        */
    }
    
    pub fn replace_with_sum_of_source_values<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(
        &self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>
    ) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithSumOfInternal (src1, src2); return *this;
        */
    }
    
    /**
      | Subtracts a fixed value from the elements
      | in this block.
      |
      */
    #[ALOE_VECTOR_CALLTYPE]
    pub fn subtract_fixed_value_from_elements_in_this_block_mut(&mut self, value: <Self as HasNumericType>::Type) -> &mut AudioBlock<SampleType> {
        
        todo!();
        /*
            subtractInternal (value); return *this;
        */
    }

    #[ALOE_VECTOR_CALLTYPE]
    pub fn subtract_fixed_value_from_elements_in_this_block(&self, value: <Self as HasNumericType>::Type) -> &AudioBlock<SampleType> {
        
        todo!();
        /*
            subtractInternal (value); return *this;
        */
    }

    /**
      | Subtracts the source values from the
      | elements in this block.
      |
      */
    pub fn subtract_source_values_from_elements_in_this_block_mut<OtherSampleType: SampleTypeInterface>(
        &mut self, 
        src: AudioBlock<OtherSampleType>

    ) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            subtractInternal (src); return *this;
        */
    }
    
    pub fn subtract_source_values_from_elements_in_this_block<OtherSampleType: SampleTypeInterface>(
        &self, 
        src: AudioBlock<OtherSampleType>
    ) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            subtractInternal (src); return *this;
        */
    }

    /**
      | Subtracts a fixed value from each source
      | value and replaces the contents of this
      | block.
      |
      */
    #[ALOE_VECTOR_CALLTYPE]
    pub fn replace_with_difference_of_fixed_value_from_each_source_value_mut<OtherSampleType: SampleTypeInterface>(
        &mut self, 
        src:   AudioBlock<OtherSampleType>,
        value: <Self as HasNumericType>::Type

    ) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithDifferenceOfInternal (src, value); return *this;
        */
    }

    #[ALOE_VECTOR_CALLTYPE]
    pub fn replace_with_difference_of_fixed_value_from_each_source_value<OtherSampleType: SampleTypeInterface>(
        &self, 
        src:   AudioBlock<OtherSampleType>,
        value: <Self as HasNumericType>::Type

    ) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithDifferenceOfInternal (src, value); return *this;
        */
    }

    /**
      | Subtracts each source2 value from the
      | corresponding source1 value and replaces
      | the contents of this block.
      |
      */
    pub fn replace_with_difference_of_source_value_pairs_mut<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(
        &mut self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>

    ) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithDifferenceOfInternal (src1, src2); return *this;
        */
    }
    
    pub fn replace_with_difference_of_source_value_pairs<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(
        &self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>
    ) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithDifferenceOfInternal (src1, src2); return *this;
        */
    }
    
    /**
      | Multiplies the elements in this block
      | by a fixed value.
      |
      */
    #[ALOE_VECTOR_CALLTYPE]
    pub fn multiply_by_value_mut(&mut self, value: <Self as HasNumericType>::Type) -> &mut AudioBlock<SampleType> {
        
        todo!();
        /*
            multiplyByInternal (value); return *this;
        */
    }

    #[ALOE_VECTOR_CALLTYPE]
    pub fn multiply_by_value(&self, value: <Self as HasNumericType>::Type) -> &AudioBlock<SampleType> {
        
        todo!();
        /*
            multiplyByInternal (value); return *this;
        */
    }

    /**
      | Multiplies the elements in this block
      | by the elements in the src block
      |
      */
    pub fn multiply_by_audio_block_mut<OtherSampleType: SampleTypeInterface>(&mut self, src: AudioBlock<OtherSampleType>) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            multiplyByInternal (src); return *this;
        */
    }
    
    pub fn multiply_by_audio_block<OtherSampleType: SampleTypeInterface>(&self, src: AudioBlock<OtherSampleType>) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            multiplyByInternal (src); return *this;
        */
    }

    /**
      | Replaces the elements in this block
      | with the product of the elements in the
      | source src block and a fixed value.
      |
      */
    #[ALOE_VECTOR_CALLTYPE]
    pub fn replace_with_product_of_elements_in_source_block_and_fixed_value_mut<OtherSampleType: SampleTypeInterface>(
        &mut self, 
        src:   AudioBlock<OtherSampleType>,
        value: <Self as HasNumericType>::Type

    ) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithProductOfInternal (src, value); return *this;
        */
    }

    #[ALOE_VECTOR_CALLTYPE]
    pub fn replace_with_product_of_elements_in_source_block_and_fixed_value<OtherSampleType: SampleTypeInterface>(
        &self, 
        src:   AudioBlock<OtherSampleType>,
        value: <Self as HasNumericType>::Type

    ) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithProductOfInternal (src, value); return *this;
        */
    }

    /**
      | Replaces the elements in this block
      | with the product of the elements in the
      | src1 and scr2 blocks.
      |
      */
    pub fn replace_with_product_of_elements_in_two_source_blocks_mut<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(
        &mut self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>
    ) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithProductOfInternal (src1, src2); return *this;
        */
    }
    
    pub fn replace_with_product_of_elements_in_two_source_blocks<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(
        &self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>

    ) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithProductOfInternal (src1, src2); return *this;
        */
    }
    
    /**
      | Multiplies each channels of this block
      | by a smoothly changing value.
      |
      */
    pub fn multiply_by_smoothly_changing_value_mut<OtherSampleType: FloatSample, SmoothingType>(&mut self, value: &mut SmoothedValue<OtherSampleType,SmoothingType>) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            multiplyByInternal (value); return *this;
        */
    }
    
    pub fn multiply_by_smoothly_changing_value<OtherSampleType: FloatSample, SmoothingType>(&self, value: &mut SmoothedValue<OtherSampleType,SmoothingType>) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            multiplyByInternal (value); return *this;
        */
    }

    /**
      | Replaces each channel of this block
      | with the product of the src block and
      | a smoothed value.
      |
      */
    pub fn replace_with_product_of_source_block_and_smoothed_value_mut<
        BlockSampleType: SampleTypeInterface, 
        SmootherSampleType: FloatSample, 
        SmoothingType
    >(
        &mut self, 
        src:   AudioBlock<BlockSampleType>,
        value: &mut SmoothedValue<SmootherSampleType,SmoothingType>

    ) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithProductOfInternal (src, value); return *this;
        */
    }
    
    pub fn replace_with_product_of_source_block_and_smoothed_value<
        BlockSampleType:    SampleTypeInterface, 
        SmootherSampleType: FloatSample, 
        SmoothingType
    >(
        &self, 
        src:   AudioBlock<BlockSampleType>,
        value: &mut SmoothedValue<SmootherSampleType,SmoothingType>

    ) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithProductOfInternal (src, value); return *this;
        */
    }
    
    /**
      | Multiplies each value in src by a fixed
      | value and adds the result to this block.
      |
      */
    #[ALOE_VECTOR_CALLTYPE]
    pub fn add_product_of_mut<OtherSampleType: SampleTypeInterface>(
        &mut self, 
        src:    AudioBlock<OtherSampleType>,
        factor: <Self as HasNumericType>::Type

    ) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            addProductOfInternal (src, factor); return *this;
        */
    }

    #[ALOE_VECTOR_CALLTYPE]
    pub fn add_product_of<OtherSampleType: SampleTypeInterface>(&self, 
        src:    AudioBlock<OtherSampleType>,
        factor: <Self as HasNumericType>::Type) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            addProductOfInternal (src, factor); return *this;
        */
    }

    /**
      | Multiplies each value in srcA with the
      | corresponding value in srcB and adds
      | the result to this block.
      |
      */
    pub fn add_product_of_two_audio_blocks_mut<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(
        &mut self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>

    ) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            addProductOfInternal (src1, src2); return *this;
        */
    }
    
    pub fn add_product_of_two_audio_blocks<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(
        &self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>

    ) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            addProductOfInternal (src1, src2); return *this;
        */
    }
    
    /**
      | Negates each value of this block.
      |
      */
    pub fn negate_mut(&mut self) -> &mut AudioBlock<SampleType> {
        
        todo!();
        /*
            negateInternal(); return *this;
        */
    }
    
    pub fn negate(&self) -> &AudioBlock<SampleType> {
        
        todo!();
        /*
            negateInternal(); return *this;
        */
    }

    /**
      | Replaces the contents of this block
      | with the negative of the values in the
      | src block.
      |
      */
    pub fn replace_with_negative_of_mut<OtherSampleType: SampleTypeInterface>(&mut self, src: AudioBlock<OtherSampleType>) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithNegativeOfInternal (src); return *this;
        */
    }
    
    pub fn replace_with_negative_of<OtherSampleType: SampleTypeInterface>(&self, src: AudioBlock<OtherSampleType>) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithNegativeOfInternal (src); return *this;
        */
    }

    /**
      | Replaces the contents of this block
      | with the absolute values of the src block.
      |
      */
    pub fn replace_with_absolute_value_of_mut<OtherSampleType: SampleTypeInterface>(&mut self, src: AudioBlock<OtherSampleType>) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithAbsoluteValueOfInternal (src); return *this;
        */
    }
    
    pub fn replace_with_absolute_value_of<OtherSampleType: SampleTypeInterface>(&self, src: AudioBlock<OtherSampleType>) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithAbsoluteValueOfInternal (src); return *this;
        */
    }
    
    /**
      | Replaces each element of this block
      | with the minimum of the corresponding
      | element of the source arrays.
      |
      */
    pub fn replace_with_min_of_mut<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(&mut self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithMinOfInternal (src1, src2); return *this;
        */
    }
    
    pub fn replace_with_min_of<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(&self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithMinOfInternal (src1, src2); return *this;
        */
    }

    /**
      | Replaces each element of this block
      | with the maximum of the corresponding
      | element of the source arrays.
      |
      */
    pub fn replace_with_max_of_mut<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(&mut self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>) -> &mut AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithMaxOfInternal (src1, src2); return *this;
        */
    }
    
    pub fn replace_with_max_of<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(&self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>) -> &AudioBlock<SampleType> {
    
        todo!();
        /*
            replaceWithMaxOfInternal (src1, src2); return *this;
        */
    }

    /**
      | Finds the minimum and maximum value
      | of the buffer.
      |
      */
    pub fn find_min_and_max(&self) -> Range<<Self as HasNumericType>::Type> {
        
        todo!();
        /*
            if (numChannels == 0)
                return {};

            auto n = static_cast<int> (numSamples * sizeFactor);
            auto minmax = FloatVectorOperations::findMinAndMax (getDataPointer (0), n);

            for (size_t ch = 1; ch < numChannels; ++ch)
                minmax = minmax.getUnionWith (FloatVectorOperations::findMinAndMax (getDataPointer (ch), n));

            return minmax;
        */
    }

    /**
      | Applies a function to each value in an
      | input block, putting the result into
      | an output block.
      | 
      | The function supplied must take a SampleType
      | as its parameter, and return a SampleType.
      | 
      | The two blocks must have the same number
      | of channels and samples.
      |
      */
    pub fn process<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface, FunctionType>(
        in_block:  AudioBlock<Src1SampleType>,
        out_block: AudioBlock<Src2SampleType>,
        function:  FunctionType)  {
    
        todo!();
        /*
            auto len = inBlock.getNumSamples();
            auto numChans = inBlock.getNumChannels();

            jassert (len == outBlock.getNumSamples());
            jassert (numChans == outBlock.getNumChannels());

            for (ChannelCountType c = 0; c < numChans; ++c)
            {
                auto* src = inBlock.getChannelPointer (c);
                auto* dst = outBlock.getChannelPointer (c);

                for (size_t i = 0; i < len; ++i)
                    dst[i] = function (src[i]);
            }
        */
    }
    
    pub fn get_data_pointer(&self, channel: usize) -> *mut <Self as HasNumericType>::Type {
        
        todo!();
        /*
            return reinterpret_cast<NumericType*> (getChannelPointer (channel));
        */
    }
    
    #[ALOE_VECTOR_CALLTYPE]
    pub fn clear_internal(&self)  {
        
        todo!();
        /*
            auto n = static_cast<int> (numSamples * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::clear (getDataPointer (ch), n);
        */
    }

    #[ALOE_VECTOR_CALLTYPE]
    pub fn fill_internal(&self, value: <Self as HasNumericType>::Type)  {
        
        todo!();
        /*
            auto n = static_cast<int> (numSamples * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::fill (getDataPointer (ch), value, n);
        */
    }
    
    pub fn copy_from_internal<OtherSampleType: SampleTypeInterface>(&self, src: &AudioBlock<OtherSampleType>)  {
    
        todo!();
        /*
            auto maxChannels = jmin (src.numChannels, numChannels);
            auto n = static_cast<int> (jmin (src.numSamples * src.sizeFactor,
                                             numSamples * sizeFactor));

            for (size_t ch = 0; ch < maxChannels; ++ch)
                FloatVectorOperations::copy (getDataPointer (ch), src.getDataPointer (ch), n);
        */
    }
    
    pub fn copy_from_internal_audio_buffer<OtherNumericType>(&self, 
        src:          &AudioBuffer<OtherNumericType>,
        src_pos:      usize,
        dst_pos:      usize,
        num_elements: usize)  {
    
        todo!();
        /*
            auto srclen = static_cast<size_t> (src.getNumSamples()) / sizeFactor;
            auto n = static_cast<int> (jmin (srclen - srcPos, numSamples - dstPos, numElements) * sizeFactor);
            auto maxChannels = jmin (static_cast<size_t> (src.getNumChannels()), static_cast<size_t> (numChannels));

            for (size_t ch = 0; ch < maxChannels; ++ch)
                FloatVectorOperations::copy (getDataPointer (ch) + (dstPos * sizeFactor),
                                             src.getReadPointer (static_cast<int> (ch),
                                                                 static_cast<int> (srcPos * sizeFactor)),
                                             n);
        */
    }
    
    pub fn move_internal(
        &self, 
        src_pos:      usize,
        dst_pos:      usize,
        num_elements: Option<usize>

    ) {

        let num_elements: usize = num_elements.unwrap_or(usize::MAX);

        todo!();
        /*
            jassert (srcPos <= numSamples && dstPos <= numSamples);
            auto len = jmin (numSamples - srcPos, numSamples - dstPos, numElements) * sizeof (SampleType);

            if (len != 0)
                for (size_t ch = 0; ch < numChannels; ++ch)
                    ::memmove (getChannelPointer (ch) + dstPos,
                               getChannelPointer (ch) + srcPos, len);
        */
    }
    
    #[ALOE_VECTOR_CALLTYPE]
    pub fn add_internal(&self, value: <Self as HasNumericType>::Type)  {
        
        todo!();
        /*
            auto n = static_cast<int> (numSamples * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::add (getDataPointer (ch), value, n);
        */
    }
    
    pub fn add_internal_with_audio_block<OtherSampleType: SampleTypeInterface>(&self, src: AudioBlock<OtherSampleType>)  {
    
        todo!();
        /*
            jassert (numChannels == src.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::add (getDataPointer (ch), src.getDataPointer (ch), n);
        */
    }

    #[ALOE_VECTOR_CALLTYPE]
    pub fn replace_with_sum_of_internal<OtherSampleType: SampleTypeInterface>(&self, 
        src:   AudioBlock<OtherSampleType>,
        value: <Self as HasNumericType>::Type)  {
    
        todo!();
        /*
            jassert (numChannels == src.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::add (getDataPointer (ch), src.getDataPointer (ch), value, n);
        */
    }
    
    pub fn replace_with_sum_of_internal_with_two_sources<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(
        &self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>

    ) {
    
        todo!();
        /*
            jassert (numChannels == src1.numChannels && src1.numChannels == src2.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src1.numSamples, src2.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::add (getDataPointer (ch), src1.getDataPointer (ch), src2.getDataPointer (ch), n);
        */
    }
    
    #[ALOE_VECTOR_CALLTYPE]
    pub fn subtract_internal(&self, value: <Self as HasNumericType>::Type)  {
        
        todo!();
        /*
            addInternal (value * static_cast<NumericType> (-1.0));
        */
    }
    
    pub fn subtract_internal_with_audio_block<OtherSampleType: SampleTypeInterface>(
        &self, 
        src: AudioBlock<OtherSampleType>

    ) {
    
        todo!();
        /*
            jassert (numChannels == src.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::subtract (getDataPointer (ch), src.getDataPointer (ch), n);
        */
    }

    #[ALOE_VECTOR_CALLTYPE]
    pub fn replace_with_difference_of_internal<OtherSampleType: SampleTypeInterface>(
        &self, 
        src:   AudioBlock<OtherSampleType>,
        value: <Self as HasNumericType>::Type

    ) {
    
        todo!();
        /*
            replaceWithSumOfInternal (src, static_cast<NumericType> (-1.0) * value);
        */
    }
    
    pub fn replace_with_difference_of_internal_with_two_audio_blocks<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(
        &self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>

    ) {
    
        todo!();
        /*
            jassert (numChannels == src1.numChannels && src1.numChannels == src2.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src1.numSamples, src2.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::subtract (getDataPointer (ch), src1.getDataPointer (ch), src2.getDataPointer (ch), n);
        */
    }
    
    #[ALOE_VECTOR_CALLTYPE]
    pub fn multiply_by_internal_with_numeric_type(&self, value: <Self as HasNumericType>::Type)  {
        
        todo!();
        /*
            auto n = static_cast<int> (numSamples * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::multiply (getDataPointer (ch), value, n);
        */
    }
    
    pub fn multiply_by_internal_with_audio_block<OtherSampleType: SampleTypeInterface>(&self, src: AudioBlock<OtherSampleType>)  {
    
        todo!();
        /*
            jassert (numChannels == src.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::multiply (getDataPointer (ch), src.getDataPointer (ch), n);
        */
    }

    #[ALOE_VECTOR_CALLTYPE]
    pub fn replace_with_product_of_internal_vector_with_src_and_value<OtherSampleType: SampleTypeInterface>(
        &self, 
        src:   AudioBlock<OtherSampleType>,
        value: <Self as HasNumericType>::Type

    ) {
    
        todo!();
        /*
            jassert (numChannels == src.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::multiply (getDataPointer (ch), src.getDataPointer (ch), value, n);
        */
    }
    
    pub fn replace_with_product_of_internal<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(
        &self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>

    ) {
    
        todo!();
        /*
            jassert (numChannels == src1.numChannels && src1.numChannels == src2.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src1.numSamples, src2.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::multiply (getDataPointer (ch), src1.getDataPointer (ch), src2.getDataPointer (ch), n);
        */
    }
    
    pub fn multiply_by_internal<OtherSampleType: FloatSample, SmoothingType>(&self, value: &mut SmoothedValue<OtherSampleType,SmoothingType>)  {
    
        todo!();
        /*
            if (! value.isSmoothing())
            {
                multiplyByInternal ((NumericType) value.getTargetValue());
            }
            else
            {
                for (size_t i = 0; i < numSamples; ++i)
                {
                    const auto scaler = (NumericType) value.getNextValue();

                    for (size_t ch = 0; ch < numChannels; ++ch)
                        getDataPointer (ch)[i] *= scaler;
                }
            }
        */
    }
    
    pub fn replace_with_product_of_internal_with_src_and_value<
        BlockSampleType: SampleTypeInterface, 
        SmootherSampleType: FloatSample, 
        SmoothingType
    >(
        &self, 
        src:   AudioBlock<BlockSampleType>,
        value: &mut SmoothedValue<SmootherSampleType,SmoothingType>

    ) {
    
        todo!();
        /*
            jassert (numChannels == src.numChannels);

            if (! value.isSmoothing())
            {
                replaceWithProductOfInternal (src, (NumericType) value.getTargetValue());
            }
            else
            {
                auto n = jmin (numSamples, src.numSamples) * sizeFactor;

                for (size_t i = 0; i < n; ++i)
                {
                    const auto scaler = (NumericType) value.getNextValue();

                    for (size_t ch = 0; ch < numChannels; ++ch)
                        getDataPointer (ch)[i] = scaler * src.getChannelPointer (ch)[i];
                }
            }
        */
    }
    
    #[ALOE_VECTOR_CALLTYPE]
    pub fn add_product_of_internal_with_factor<OtherSampleType: SampleTypeInterface>(&self, 
        src:    AudioBlock<OtherSampleType>,
        factor: <Self as HasNumericType>::Type)  {
    
        todo!();
        /*
            jassert (numChannels == src.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::addWithMultiply (getDataPointer (ch), src.getDataPointer (ch), factor, n);
        */
    }
    
    pub fn add_product_of_internal<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(&self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>)  {
    
        todo!();
        /*
            jassert (numChannels == src1.numChannels && src1.numChannels == src2.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src1.numSamples, src2.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::addWithMultiply (getDataPointer (ch), src1.getDataPointer (ch), src2.getDataPointer (ch), n);
        */
    }
    
    pub fn negate_internal(&self)  {
        
        todo!();
        /*
            multiplyByInternal (static_cast<NumericType> (-1.0));
        */
    }
    
    pub fn replace_with_negative_of_internal<OtherSampleType: SampleTypeInterface>(&self, src: AudioBlock<OtherSampleType>)  {
    
        todo!();
        /*
            jassert (numChannels == src.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::negate (getDataPointer (ch), src.getDataPointer (ch), n);
        */
    }
    
    pub fn replace_with_absolute_value_of_internal<OtherSampleType: SampleTypeInterface>(&self, src: AudioBlock<OtherSampleType>)  {
    
        todo!();
        /*
            jassert (numChannels == src.numChannels);
            auto n = static_cast<int> (jmin (numSamples, src.numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::abs (getDataPointer (ch), src.getDataPointer (ch), n);
        */
    }
    
    pub fn replace_with_min_of_internal<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(&self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>)  {
    
        todo!();
        /*
            jassert (numChannels == src1.numChannels && src1.numChannels == src2.numChannels);
            auto n = static_cast<int> (jmin (src1.numSamples, src2.numSamples, numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::min (getDataPointer (ch), src1.getDataPointer (ch), src2.getDataPointer (ch), n);
        */
    }
    
    pub fn replace_with_max_of_internal<Src1SampleType: SampleTypeInterface, Src2SampleType: SampleTypeInterface>(&self, 
        src1: AudioBlock<Src1SampleType>,
        src2: AudioBlock<Src2SampleType>)  {
    
        todo!();
        /*
            jassert (numChannels == src1.numChannels && src1.numChannels == src2.numChannels);
            auto n = static_cast<int> (jmin (src1.numSamples, src2.numSamples, numSamples) * sizeFactor);

            for (size_t ch = 0; ch < numChannels; ++ch)
                FloatVectorOperations::max (getDataPointer (ch), src1.getDataPointer (ch), src2.getDataPointer (ch), n);
        */
    }
}
