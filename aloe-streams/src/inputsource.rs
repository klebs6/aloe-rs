crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_InputSource.h]

/**
  | A lightweight object that can create
  | a stream to read some kind of resource.
  | 
  | This may be used to refer to a file, or
  | some other kind of source, allowing
  | a caller to create an input stream that
  | can read from it when required.
  | 
  | @see FileInputSource
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub trait InputSource {

    /**
      | Returns a new InputStream to read this
      | item.
      | 
      | -----------
      | @return
      | 
      | an inputstream that the caller will
      | delete, or nullptr if the filename isn't
      | found.
      |
      */
    fn create_input_stream(&mut self) -> Box<dyn Read>;

    /**
      | Returns a new InputStream to read an
      | item, relative.
      | 
      | -----------
      | @param relatedItemPath
      | 
      | the relative pathname of the resource
      | that is required
      | 
      | -----------
      | @return
      | 
      | an inputstream that the caller will
      | delete, or nullptr if the item isn't
      | found.
      |
      */
    fn create_input_stream_for(&mut self, related_item_path: &String) -> Box<dyn Read>;

    /**
      | Returns a hash code that uniquely represents
      | this item.
      |
      */
    fn hash_code(&self) -> i64;
}
