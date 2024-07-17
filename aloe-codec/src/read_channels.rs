crate::ix!();

pub fn read_channels<'a>(
    reader:              &mut AudioFormatReader<'a>,
    chans:               *mut *mut i32,
    buffer:              *mut AudioBuffer<f32>,
    start_sample:        i32,
    num_samples:         i32,
    reader_start_sample: i64,
    num_target_channels: i32,
    convert_to_float:    bool

) {
    
    todo!();
        /*
            for (int j = 0; j < numTargetChannels; ++j)
            chans[j] = reinterpret_cast<int*> (buffer->getWritePointer (j, startSample));

        chans[numTargetChannels] = nullptr;
        reader.read (chans, numTargetChannels, readerStartSample, numSamples, true);

        if (convertToFloat)
            convertFixedToFloat (chans, numTargetChannels, numSamples);
        */
}

