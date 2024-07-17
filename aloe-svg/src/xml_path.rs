crate::ix!();

pub struct SvgStateXmlPath {
    xml:    *const XmlElement,
    parent: *const SvgStateXmlPath,
}

impl Deref for SvgStateXmlPath {

    type Target = XmlElement;

    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return xml;
        */
    }
}

impl SvgStateXmlPath {

    pub fn new(
        e: *const XmlElement,
        p: *const SvgStateXmlPath) -> Self {
    
        todo!();
        /*
        : xml(e),
        : parent(p),

        
        */
    }
    
    pub fn get_child(&self, e: *const XmlElement) -> SvgStateXmlPath {
        
        todo!();
        /*
            return SvgStateXmlPath (e, this);
        */
    }
    
    pub fn apply_operation_to_child_withid<OperationType>(&self, 
        id: &String,
        op: &mut OperationType) -> bool {
    
        todo!();
        /*
            for (auto* e : xml->getChildIterator())
                {
                    SvgStateXmlPath child (e, this);

                    if (e->compareAttribute ("id", id)
                          && ! child->hasTagName ("defs"))
                        return op (child);

                    if (child.applyOperationToChildWithID (id, op))
                        return true;
                }

                return false;
        */
    }
}
