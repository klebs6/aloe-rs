crate::ix!();

pub fn set_impulse_response(
    factory:          &mut ConvolutionEngineFactory,
    source_data:      *const c_void,
    source_data_size: usize,
    stereo:           ConvolutionStereo,
    trim:             ConvolutionTrim,
    size:             usize,
    normalise:        ConvolutionNormalise
) {
    
    todo!();
    /*
        factory.setImpulseResponse (loadStreamToBuffer (std::make_unique<MemoryInputStream> (sourceData, sourceDataSize, false), size),
                                    stereo, trim, normalise);
    */
}

pub fn set_impulse_response_from_file(
    factory:               &mut ConvolutionEngineFactory,
    file_impulse_response: &File,
    stereo:                ConvolutionStereo,
    trim:                  ConvolutionTrim,
    size:                  usize,
    normalise:             ConvolutionNormalise
) {
    
    todo!();
    /*
        factory.setImpulseResponse (loadStreamToBuffer (std::make_unique<FileInputStream> (fileImpulseResponse), size),
                                    stereo, trim, normalise);
    */
}
