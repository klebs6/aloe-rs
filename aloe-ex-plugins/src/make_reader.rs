crate::ix!();

pub trait AudioFormatReaderFactory {
    fn make(&self, _0: &mut AudioFormatManager) -> Box<AudioFormatReader>;
    fn clone(&self) -> Box<dyn AudioFormatReaderFactory>;
}

#[inline] pub fn make_audio_format_reader(
        manager:     &mut AudioFormatManager,
        sample_data: *const c_void,
        data_size:   usize) -> Box<AudioFormatReader> {
    
    todo!();
    /*
        return std::unique_ptr<AudioFormatReader> (manager.createReaderFor (std::make_unique<MemoryInputStream> (sampleData,
                                                                                                                 dataSize,
                                                                                                                 false)));
    */
}

#[inline] pub fn make_audio_format_reader_from_file<'a>(
    manager: &mut AudioFormatManager,
    file:    &File

) -> Box<AudioFormatReader<'a>> {
    
    todo!();
    /*
        return std::unique_ptr<AudioFormatReader> (manager.createReaderFor (file));
    */
}
