crate::ix!();

/**
  | ValueTreeIterator for a ValueTree.
  | 
  | You shouldn't ever need to use this class
  | directly - it's used internally by ValueTree::begin()
  | and ValueTree::end() to allow range-based-for
  | loops on a ValueTree.
  |
  */
pub struct ValueTreeIterator {
    internal: *mut c_void,
}

impl HasDifferenceType   for ValueTreeIterator { type DifferenceType   = libc::ptrdiff_t; }
impl HasValueType        for ValueTreeIterator { type ValueType        = ValueTree; }
impl HasIteratorCategory for ValueTreeIterator { type IteratorCategory = ForwardIteratorTag; }

pub struct ForwardIteratorTag;

impl PartialEq<ValueTreeIterator> for ValueTreeIterator {
    
    #[inline] fn eq(&self, other: &ValueTreeIterator) -> bool {
        todo!();
        /*
            return internal == other.internal;
        */
    }
}

impl Eq for ValueTreeIterator {}

impl Deref for ValueTreeIterator {

    type Target = ValueTree;

    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return ValueTree (ValueTreeSharedObject::Ptr (*static_cast<ValueTreeSharedObject**> (internal)));
        */
    }
}

impl ValueTreeIterator {

    pub fn new(
        v:      &ValueTree,
        is_end: bool) -> Self {
    
        todo!();
        /*


            : internal (v.object != nullptr ? (isEnd ? v.object->children.end() : v.object->children.begin()) : nullptr)
        */
    }
    
    pub fn prefix_increment(&mut self) -> &mut ValueTreeIterator {
        
        todo!();
        /*
            internal = static_cast<ValueTreeSharedObject**> (internal) + 1;
        return *this;
        */
    }
}
