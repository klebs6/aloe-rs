crate::ix!();

/**
  | An OSC bundle element.
  | 
  | An OSCBundle OSCBundleElement contains either
  | one OSCMessage or one OSCBundle.
  |
  */
pub struct OSCBundleElement {
    message: Box<OSCMessage>,
    bundle:  Box<OSCBundle>,
}

impl Drop for OSCBundleElement {

    fn drop(&mut self) {
        todo!();
        /*
            bundle = nullptr;
        message = nullptr;
        */
    }
}

impl From<OSCMessage> for OSCBundleElement {
        
    /**
      | Constructs an OSCBundle OSCBundleElement from
      | an OSCMessage.
      |
      | Note: The class invariant of OSCBundle::OSCBundleElement
      | is that at least one of the pointers bundle and
      | message is nullptr and the other one always
      | points to a valid object.
      */
    fn from(m: OSCMessage) -> Self {
    
        todo!();
        /*


            : message (new OSCMessage (m)), bundle (nullptr)
        */
    }
}

impl From<OSCBundle> for OSCBundleElement {
    
    /**
      | Constructs an OSCBundle OSCBundleElement from
      | an OSCBundle.
      |
      */
    fn from(b: OSCBundle) -> Self {
    
        todo!();
        /*


            : message (nullptr), bundle (new OSCBundle (b))
        */
    }
}

impl From<&OSCBundleElement> for OSCBundleElement {
    
    /**
      | Copy constructor.
      |
      */
    fn from(other: &OSCBundleElement) -> Self {
    
        todo!();
        /*


            if (this != &other)
        {
            message = nullptr;
            bundle = nullptr;

            if (other.isMessage())
                message.reset (new OSCMessage (other.getMessage()));
            else
                bundle.reset (new OSCBundle (other.getBundle()));
        }
        */
    }
}
    
impl OSCBundleElement {

    /**
      | Returns true if the OSCBundle element
      | is an OSCMessage.
      |
      */
    pub fn is_message(&self) -> bool {
        
        todo!();
        /*
            return message != nullptr;
        */
    }
    
    /**
      | Returns true if the OSCBundle element
      | is an OSCBundle.
      |
      */
    pub fn is_bundle(&self) -> bool {
        
        todo!();
        /*
            return bundle != nullptr;
        */
    }
    
    /**
      | Returns a reference to the contained
      | OSCMessage.
      | 
      | If the OSCBundle element is not an OSCMessage,
      | behaviour is undefined.
      |
      */
    pub fn get_message(&self) -> &OSCMessage {
        
        todo!();
        /*
            if (message == nullptr)
        {
            // This element is not a bundle! You must check this first before accessing.
            jassertfalse;
            throw OSCInternalError ("Access error in OSC bundle element.");
        }

        return *message;
        */
    }
    
    /**
      | Returns a reference to the contained
      | OSCBundle.
      | 
      | If the OSCBundle element is not an OSCBundle,
      | behaviour is undefined.
      |
      */
    pub fn get_bundle(&self) -> &OSCBundle {
        
        todo!();
        /*
            if (bundle == nullptr)
        {
            // This element is not a bundle! You must check this first before accessing.
            jassertfalse;
            throw OSCInternalError ("Access error in OSC bundle element.");
        }

        return *bundle;
        */
    }
}
