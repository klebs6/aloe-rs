crate::ix!();

pub trait CopyXMLToBinary {

    /**
      | Helper function that just converts
      | an xml element into a binary blob.
      | 
      | Use this in your processor's getStateInformation()
      | method if you want to store its state
      | as xml.
      | 
      | Then use getXmlFromBinary() to reverse
      | this operation and retrieve the XML
      | from a binary blob.
      |
      */
    fn copy_xml_to_binary(
        xml:       &XmlElement,
        dest_data: &mut MemoryBlock
    ) where Self: Sized;
}

pub trait GetXMLFromBinary {

    /**
      | Retrieves an XML element that was stored
      | as binary with the copyXmlToBinary()
      | method.
      | 
      | This might return nullptr if the data's
      | unsuitable or corrupted.
      |
      */
    fn get_xml_from_binary(
        data:          *const c_void,
        size_in_bytes: i32) -> Box<XmlElement> where Self: Sized;
}
