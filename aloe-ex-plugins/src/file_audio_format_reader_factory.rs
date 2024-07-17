crate::ix!();

pub struct FileAudioFormatReaderFactory {
    file: File,
}

impl AudioFormatReaderFactory for FileAudioFormatReaderFactory {

    fn make(&self, manager: &mut AudioFormatManager) -> Box<AudioFormatReader> {
        
        todo!();
        /*
            return makeAudioFormatReader (manager, file);
        */
    }
    
    fn clone(&self) -> Box<dyn AudioFormatReaderFactory> {
        
        todo!();
        /*
            return std::unique_ptr<AudioFormatReaderFactory> (new FileAudioFormatReaderFactory (*this));
        */
    }
}

impl FileAudioFormatReaderFactory {

    pub fn new(file_in: File) -> Self {
    
        todo!();
        /*


            : file (std::move (fileIn))
        */
    }
}
