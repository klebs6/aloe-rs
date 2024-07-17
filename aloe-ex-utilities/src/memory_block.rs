crate::ix!();

/**
  | This is a token that's used at both ends
  | of our parent-child processes, to act
  | as a unique token in the command line
  | arguments.
  |
  */
pub const demoCommandLineUID: &'static str = "demoUID";

/**
  | A few quick utility functions to convert
  | between raw data and ValueTrees
  |
  */
pub fn memory_block_to_value_tree(mb: &MemoryBlock) -> ValueTree {
    
    todo!();
    /*
        return ValueTree::readFromData (mb.getData(), mb.getSize());
    */
}

pub fn value_tree_to_memory_block(v: &ValueTree) -> MemoryBlock {
    
    todo!();
    /*
        MemoryOutputStream mo;
        v.writeToStream (mo);

        return mo.getMemoryBlock();
    */
}

pub fn value_tree_to_string(v: &ValueTree) -> String {
    
    todo!();
    /*
        if (auto xml = v.createXml())
            return xml->toString (XmlElement::TextFormat().singleLine().withoutHeader());

        return {};
    */
}
