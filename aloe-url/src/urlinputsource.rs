crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_URLInputSource.h]

/**
  | A type of InputSource that represents
  | a Url.
  | 
  | @see InputSource
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
#[derive(Default)]
pub struct URLInputSource {
    u:    Url,
}

impl InputSource for URLInputSource {

    fn create_input_stream(&mut self) -> Box<dyn Read> {
        
        todo!();
        /*
            return u.createInputStream (false).release();
        */
    }
    
    fn create_input_stream_for(&mut self, related_item_path: &std::string::String) 
        -> Box<dyn Read> 
    {
        todo!();
        /*
            auto sub = u.getSubPath();
        auto parent = sub.containsChar (L'/') ? sub.upToLastOccurrenceOf ("/", false, false)
                                              : String ();

        return u.withNewSubPath (parent).getChildURL (relatedItemPath).createInputStream (false).release();
        */
    }
    
    fn hash_code(&self) -> i64 {
        
        todo!();
        /*
            return u.toString (true).hashCode64();
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/streams/aloe_URLInputSource.cpp]
impl URLInputSource {

    /**
      | Creates a URLInputSource for a url.
      |
      */
    pub fn new_from_ref(url: &Url) -> Self {
    
        todo!();
        /*
        : u(url),

        
        */
    }
    
    /**
      | Move constructor which will move the
      | Url into the InputSource.
      | 
      | This is useful when the url carries any
      | security credentials.
      |
      */
    pub fn new(url: Url) -> Self {
    
        todo!();
        /*
        : u(std::move (url)),

        
        */
    }
}
