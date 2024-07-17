crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_FileInputSource.h]

/**
  | A type of InputSource that represents
  | a normal file.
  | 
  | @see InputSource
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
#[derive(Default)]
pub struct FileInputSource {
    file:                             File,
    use_file_time_in_hash_generation: bool,
}

impl InputSource for FileInputSource {

    fn create_input_stream(&mut self) -> Box<dyn Read> {
        
        todo!();
        /*
            return file.createInputStream().release();
        */
    }
    
    fn create_input_stream_for(&mut self, related_item_path: &String) -> Box<dyn Read> {
        
        todo!();
        /*
            return file.getSiblingFile (relatedItemPath).createInputStream().release();
        */
    }
    
    fn hash_code(&self) -> i64 {
        
        todo!();
        /*
            int64 h = file.hashCode();

        if (useFileTimeInHashGeneration)
            h ^= file.getLastModificationTime().toMilliseconds();

        return h;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_FileInputSource.cpp]
impl FileInputSource {

    /**
      | Creates a FileInputSource for a file.
      | 
      | If the useFileTimeInHashGeneration
      | parameter is true, then this object's
      | hashCode() method will incorporate
      | the file time into its hash code; if false,
      | only the file name will be used for the
      | hash.
      |
      */
    pub fn new(
        f:                     &File,
        use_file_time_in_hash: Option<bool>

    ) -> Self {

        let use_file_time_in_hash: bool =
            use_file_time_in_hash.unwrap_or(false);
    
        todo!();
        /*


            : file (f), useFileTimeInHashGeneration (useFileTimeInHash)
        */
    }
}
