crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCBundle.h]

/**
  | An OSC bundle.
  | 
  | An OSCBundle contains an OSCTimeTag
  | and zero or more OSCBundle Elements.
  | 
  | The elements of a bundle can be OSC messages
  | or other OSC bundles (this means that
  | OSC bundles can be nested).
  | 
  | This is an advanced OSC structure useful
  | to bundle OSC messages together whose
  | effects must occur simultaneously
  | at some given time. For most use cases
  | it is probably enough to send and receive
  | plain OSC messages.
  | 
  | @tags{OSC}
  |
  */
pub struct OSCBundle {
    elements: Vec<OSCBundleElement>,
    time_tag: OSCTimeTag,
}

impl Default for OSCBundle {
    
    /**
      | Constructs an OSCBundle with no content
      | and a default time tag ("immediately").
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl IndexMut<i32> for OSCBundle {
    
    /**
      | Returns a reference to the OSCBundle
      | element at index i in this bundle.
      | 
      | This method does not check the range
      | and results in undefined behaviour
      | in case i < 0 or i >= size().
      |
      */
    #[inline] fn index_mut(&mut self, i: i32) -> &mut Self::Output {
        todo!();
        /*
            return elements.getReference (i);
        */
    }
}

impl Index<i32> for OSCBundle {

    type Output = OSCBundleElement;
    
    #[inline] fn index(&self, i: i32) -> &Self::Output {
        todo!();
        /*
            return elements.getReference (i);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_osc/osc/aloe_OSCBundle.cpp]
    
impl From<OSCTimeTag> for OSCBundle {

    /**
      | Constructs an OSCBundle with no content
      | and a given time tag.
      |
      */
    fn from(t: OSCTimeTag) -> Self {
    
        todo!();
        /*
        : time_tag(t),

        
        */
    }
}

impl OSCBundle {

    /**
      | Sets the OSCBundle's OSC time tag.
      |
      */
    pub fn set_time_tag(&mut self, new_time_tag: OSCTimeTag)  {
        
        todo!();
        /*
            timeTag = newTimeTag;
        */
    }

    /**
      | Returns the OSCBundle's OSC time tag.
      |
      */
    pub fn get_time_tag(&self) -> OSCTimeTag {
        
        todo!();
        /*
            return timeTag;
        */
    }
    
    /**
      | Returns the number of elements contained
      | in the bundle.
      |
      */
    pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return elements.size();
        */
    }

    /**
      | Returns true if the bundle contains
      | no elements; false otherwise.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return elements.isEmpty();
        */
    }

    /**
      | Adds an OSCBundleElement to the OSCBundle's
      | content. s
      |
      */
    pub fn add_element(&mut self, element: &OSCBundleElement)  {
        
        todo!();
        /*
            elements.add (element);
        */
    }

    /**
      | Returns a pointer to the first element
      | of the OSCBundle.
      |
      */
    pub fn begin_mut(&mut self) -> *mut OSCBundleElement {
        
        todo!();
        /*
            return elements.begin();
        */
    }

    /**
      | Returns a pointer to the first element
      | of the OSCBundle.
      |
      */
    pub fn begin(&self) -> *const OSCBundleElement {
        
        todo!();
        /*
            return elements.begin();
        */
    }

    /**
      | Returns a pointer past the last element
      | of the OSCBundle.
      |
      */
    pub fn end_mut(&mut self) -> *mut OSCBundleElement {
        
        todo!();
        /*
            return elements.end();
        */
    }

    /**
      | Returns a pointer past the last element
      | of the OSCBundle.
      |
      */
    pub fn end(&self) -> *const OSCBundleElement {
        
        todo!();
        /*
            return elements.end();
        */
    }
}
