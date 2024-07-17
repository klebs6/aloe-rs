crate::ix!();

/**
  | We'll use a custom focus traverser here
  | to make sure focus goes from the text
  | editor to another component rather
  | than back to the label itself.
  |
  */
#[no_copy]
#[leak_detector]
pub struct LabelKeyboardFocusTraverser<'a> {
    base:  KeyboardFocusTraverser,
    owner: &'a mut Label<'a>,
}

impl<'a> LabelKeyboardFocusTraverser<'a> {

    pub fn new(l: &mut Label) -> Self {
    
        todo!();
        /*
        : owner(l),
        */
    }
    
    pub fn get_default_component(&mut self, parent: *mut Component) -> *mut Component {
        
        todo!();
        /*
            auto getContainer = [&]
            {
                if (owner.getCurrentTextEditor() != nullptr && parent == &owner)
                    return owner.findKeyboardFocusContainer();

                return parent;
            };

            if (auto* container = getContainer())
                KeyboardFocusTraverser::getDefaultComponent (container);

            return nullptr;
        */
    }
    
    pub fn get_next_component(&mut self, c: *mut Component) -> *mut Component {
        
        todo!();
        /*
            return KeyboardFocusTraverser::getNextComponent     (getComp (c));
        */
    }
    
    pub fn get_previous_component(&mut self, c: *mut Component) -> *mut Component {
        
        todo!();
        /*
            return KeyboardFocusTraverser::getPreviousComponent (getComp (c));
        */
    }
    
    pub fn get_comp(&self, current: *mut Component) -> *mut Component {
        
        todo!();
        /*
            if (auto* ed = owner.getCurrentTextEditor())
                if (current == ed)
                    return current->getParentComponent();

            return current;
        */
    }
}
