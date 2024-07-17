crate::ix!();

pub struct FirstAndSecondPartBlocks<SampleType: SampleTypeInterface> {
    first:  AudioBlock<SampleType>,
    second: AudioBlock<SampleType>,
}

pub fn get_first_and_second_part_blocks<SampleType: SampleTypeInterface>(
    buffer_dry:       &mut AudioBuffer<SampleType>,
    first_part_start: usize,
    channels_to_copy: usize,
    samples_to_copy:  usize

) -> FirstAndSecondPartBlocks<SampleType> {

    todo!();
    /*
        const auto actualChannelsToCopy = jmin (channelsToCopy, (size_t) bufferDry.getNumChannels());
        const auto firstPartLength = jmin ((size_t) bufferDry.getNumSamples() - firstPartStart, samplesToCopy);
        const auto secondPartLength = samplesToCopy - firstPartLength;

        const auto channelBlock = AudioBlock<SampleType> (bufferDry).getSubsetChannelBlock (0, actualChannelsToCopy);

        return { channelBlock.getSubBlock (firstPartStart, firstPartLength),
                 secondPartLength != 0 ? channelBlock.getSubBlock (0, samplesToCopy - firstPartLength) : AudioBlock<SampleType>() };
    */
}
