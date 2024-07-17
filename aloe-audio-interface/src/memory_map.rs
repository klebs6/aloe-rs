crate::ix!();

pub trait MapEntireFile {

    /**
      | Attempts to map the entire file into
      | memory.
      |
      */
    fn map_entire_file(&mut self) -> bool;
}

pub trait GetMappedSection {

    /**
      | Returns the sample range that's currently
      | memory-mapped and available for reading.
      |
      */
    fn get_mapped_section(&self) -> Range<i64>;
}

pub trait SampleToPointer {

    /**
      | Converts a sample index to a pointer
      | to the mapped file memory.
      |
      */
    fn sample_to_pointer(&self, sample: i64);
}

pub trait GetFile {

    /**
      | Returns the file that is being mapped
      |
      */
    fn get_file(&self) -> &File;
}

pub trait CreateMemoryMappedReader {

    fn create_memory_mapped_reader(&mut self, _0: &File) -> Box<dyn MemoryMappedAudioFormatReaderInterface>;

    fn create_memory_mapped_reader_with_file_input_stream(
        &mut self, 
        fin: *mut FileInputStream
    ) 
        -> Box<dyn MemoryMappedAudioFormatReaderInterface>;
}
