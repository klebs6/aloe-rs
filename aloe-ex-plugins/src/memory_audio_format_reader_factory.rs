crate::ix!();

pub struct MemoryAudioFormatReaderFactory {
    sample_data: *const c_void,
    data_size:   usize,
}

impl AudioFormatReaderFactory for MemoryAudioFormatReaderFactory {

    fn make(&self, manager: &mut AudioFormatManager) -> Box<AudioFormatReader> {
        
        todo!();
        /*
            return makeAudioFormatReader (manager, sampleData, dataSize);
        */
    }
    
    fn clone(&self) -> Box<dyn AudioFormatReaderFactory> {
        
        todo!();
        /*
            return std::unique_ptr<AudioFormatReaderFactory> (new MemoryAudioFormatReaderFactory (*this));
        */
    }
}

impl MemoryAudioFormatReaderFactory {

    pub fn new(
        sample_data_in: *const c_void,
        data_size_in:   usize) -> Self {
    
        todo!();
        /*
        : sample_data(sampleDataIn),
        : data_size(dataSizeIn),

        
        */
    }
}
