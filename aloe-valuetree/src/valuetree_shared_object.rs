crate::ix!();

#[leak_detector]
pub struct ValueTreeSharedObject {
    base:                       ReferenceCountedObject,
    ty:                         Identifier,
    properties:                 NamedValueSet,
    children:                   ReferenceCountedArray<ValueTreeSharedObject>,
    value_trees_with_listeners: SortedSet<*mut ValueTree>,
    parent:                     *mut ValueTreeSharedObject, // default = nullptr
}

pub type ValueTreeSharedObjectPtr = ReferenceCountedObjectPtr<ValueTreeSharedObject>;

impl Drop for ValueTreeSharedObject {

    fn drop(&mut self) {
        todo!();
        /* 
                jassert (parent == nullptr); // this should never happen unless something isn't obeying the ref-counting!

                for (auto i = children.size(); --i >= 0;)
                {
                    const Ptr c (children.getObjectPointerUnchecked (i));
                    c->parent = nullptr;
                    children.remove (i);
                    c->sendParentChangeMessage();
                }
             */
    }
}

impl ValueTreeSharedObject {

    pub fn new_from_identifier(t: &Identifier) -> Self {
    
        todo!();
        /*
        : ty(t),

        
        */
    }
    
    pub fn new(other: &ValueTreeSharedObject) -> Self {
    
        todo!();
        /*
        : reference_counted_object(),
        : ty(other.type),
        : properties(other.properties),

            for (auto* c : other.children)
                {
                    auto* child = new ValueTreeSharedObject (*c);
                    child->parent = this;
                    children.add (child);
                }
        */
    }
    
    pub fn get_root(&mut self) -> &mut ValueTreeSharedObject {
        
        todo!();
        /*
            return parent == nullptr ? *this : parent->getRoot();
        */
    }
    
    pub fn call_listeners<Function>(
        &self, 
        listener_to_exclude: *mut dyn ValueTreeListener,
        fn_:                 Function

    ) {
    
        todo!();
        /*
            auto numListeners = valueTreesWithListeners.size();

                if (numListeners == 1)
                {
                    valueTreesWithListeners.getUnchecked (0)->listeners.callExcluding (listenerToExclude, fn);
                }
                else if (numListeners > 0)
                {
                    auto listenersCopy = valueTreesWithListeners;

                    for (int i = 0; i < numListeners; ++i)
                    {
                        auto* v = listenersCopy.getUnchecked (i);

                        if (i == 0 || valueTreesWithListeners.contains (v))
                            v->listeners.callExcluding (listenerToExclude, fn);
                    }
                }
        */
    }
    
    pub fn call_listeners_for_all_parents<Function>(
        &self, 
        listener_to_exclude: *mut dyn ValueTreeListener,
        fn_:                 Function

    ) {
    
        todo!();
        /*
            for (auto* t = this; t != nullptr; t = t->parent)
                    t->callListeners (listenerToExclude, fn);
        */
    }
    
    pub fn send_property_change_message(
        &mut self, 
        property:            &Identifier,
        listener_to_exclude: *mut dyn ValueTreeListener

    ) {

        todo!();
        /*
            ValueTree tree (*this);
                callListenersForAllParents (listenerToExclude, [&] (Listener& l) { l.valueTreePropertyChanged (tree, property); });
        */
    }
    
    pub fn send_child_added_message(&mut self, child: ValueTree)  {
        
        todo!();
        /*
            ValueTree tree (*this);
                callListenersForAllParents (nullptr, [&] (Listener& l) { l.valueTreeChildAdded (tree, child); });
        */
    }
    
    pub fn send_child_removed_message(&mut self, 
        child: ValueTree,
        index: i32)  {
        
        todo!();
        /*
            ValueTree tree (*this);
                callListenersForAllParents (nullptr, [=, &tree, &child] (Listener& l) { l.valueTreeChildRemoved (tree, child, index); });
        */
    }
    
    pub fn send_child_order_changed_message(&mut self, 
        old_index: i32,
        new_index: i32)  {
        
        todo!();
        /*
            ValueTree tree (*this);
                callListenersForAllParents (nullptr, [=, &tree] (Listener& l) { l.valueTreeChildOrderChanged (tree, oldIndex, newIndex); });
        */
    }
    
    pub fn send_parent_change_message(&mut self)  {
        
        todo!();
        /*
            ValueTree tree (*this);

                for (auto j = children.size(); --j >= 0;)
                    if (auto* child = children.getObjectPointer (j))
                        child->sendParentChangeMessage();

                callListeners (nullptr, [&] (Listener& l) { l.valueTreeParentChanged (tree); });
        */
    }
    
    pub fn set_property(
        &mut self, 
        name:                &Identifier,
        new_value:           &Var,
        undo_manager:        *mut UndoManager,
        listener_to_exclude: *mut dyn ValueTreeListener

    ) {

        todo!();
        /*
            if (undoManager == nullptr)
                {
                    if (properties.set (name, newValue))
                        sendPropertyChangeMessage (name, listenerToExclude);
                }
                else
                {
                    if (auto* existingValue = properties.getVarPointer (name))
                    {
                        if (*existingValue != newValue)
                            undoManager->perform (new ValueTreeSharedObjectSetPropertyAction (*this, name, newValue, *existingValue,
                                                                         false, false, listenerToExclude));
                    }
                    else
                    {
                        undoManager->perform (new ValueTreeSharedObjectSetPropertyAction (*this, name, newValue, {},
                                                                     true, false, listenerToExclude));
                    }
                }
        */
    }
    
    pub fn has_property(&self, name: &Identifier) -> bool {
        
        todo!();
        /*
            return properties.contains (name);
        */
    }
    
    pub fn remove_property(&mut self, 
        name:         &Identifier,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            if (undoManager == nullptr)
                {
                    if (properties.remove (name))
                        sendPropertyChangeMessage (name);
                }
                else
                {
                    if (properties.contains (name))
                        undoManager->perform (new ValueTreeSharedObjectSetPropertyAction (*this, name, {}, properties[name], false, true));
                }
        */
    }
    
    pub fn remove_all_properties(&mut self, undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            if (undoManager == nullptr)
                {
                    while (properties.size() > 0)
                    {
                        auto name = properties.getName (properties.size() - 1);
                        properties.remove (name);
                        sendPropertyChangeMessage (name);
                    }
                }
                else
                {
                    for (auto i = properties.size(); --i >= 0;)
                        undoManager->perform (new ValueTreeSharedObjectSetPropertyAction (*this, properties.getName (i), {},
                                                                     properties.getValueAt (i), false, true));
                }
        */
    }
    
    pub fn copy_properties_from(&mut self, 
        source:       &ValueTreeSharedObject,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            for (auto i = properties.size(); --i >= 0;)
                    if (! source.properties.contains (properties.getName (i)))
                        removeProperty (properties.getName (i), undoManager);

                for (int i = 0; i < source.properties.size(); ++i)
                    setProperty (source.properties.getName (i), source.properties.getValueAt (i), undoManager);
        */
    }
    
    pub fn get_child_with_name(&self, type_to_match: &Identifier) -> ValueTree {
        
        todo!();
        /*
            for (auto* s : children)
                    if (s->type == typeToMatch)
                        return ValueTree (*s);

                return {};
        */
    }
    
    pub fn get_or_create_child_with_name(&mut self, 
        type_to_match: &Identifier,
        undo_manager:  *mut UndoManager) -> ValueTree {
        
        todo!();
        /*
            for (auto* s : children)
                    if (s->type == typeToMatch)
                        return ValueTree (*s);

                auto newObject = new ValueTreeSharedObject (typeToMatch);
                addChild (newObject, -1, undoManager);
                return ValueTree (*newObject);
        */
    }
    
    pub fn get_child_with_property(&self, 
        property_name:  &Identifier,
        property_value: &Var) -> ValueTree {
        
        todo!();
        /*
            for (auto* s : children)
                    if (s->properties[propertyName] == propertyValue)
                        return ValueTree (*s);

                return {};
        */
    }
    
    pub fn isa_child_of(&self, possible_parent: *const ValueTreeSharedObject) -> bool {
        
        todo!();
        /*
            for (auto* p = parent; p != nullptr; p = p->parent)
                    if (p == possibleParent)
                        return true;

                return false;
        */
    }
    
    pub fn index_of(&self, child: &ValueTree) -> i32 {
        
        todo!();
        /*
            return children.indexOf (child.object);
        */
    }
    
    pub fn add_child(&mut self, 
        child:        *mut ValueTreeSharedObject,
        index:        i32,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            if (child != nullptr && child->parent != this)
                {
                    if (child != this && ! isAChildOf (child))
                    {
                        // You should always make sure that a child is removed from its previous parent before
                        // adding it somewhere else - otherwise, it's ambiguous as to whether a different
                        // undomanager should be used when removing it from its current parent..
                        jassert (child->parent == nullptr);

                        if (child->parent != nullptr)
                        {
                            jassert (child->parent->children.indexOf (child) >= 0);
                            child->parent->removeChild (child->parent->children.indexOf (child), undoManager);
                        }

                        if (undoManager == nullptr)
                        {
                            children.insert (index, child);
                            child->parent = this;
                            sendChildAddedMessage (ValueTree (*child));
                            child->sendParentChangeMessage();
                        }
                        else
                        {
                            if (! isPositiveAndBelow (index, children.size()))
                                index = children.size();

                            undoManager->perform (new ValueTreeSharedObjectAddOrRemoveChildAction (*this, index, child));
                        }
                    }
                    else
                    {
                        // You're attempting to create a recursive loop! A node
                        // can't be a child of one of its own children!
                        jassertfalse;
                    }
                }
        */
    }
    
    pub fn remove_child(&mut self, 
        child_index:  i32,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            if (auto child = Ptr (children.getObjectPointer (childIndex)))
                {
                    if (undoManager == nullptr)
                    {
                        children.remove (childIndex);
                        child->parent = nullptr;
                        sendChildRemovedMessage (ValueTree (child), childIndex);
                        child->sendParentChangeMessage();
                    }
                    else
                    {
                        undoManager->perform (new ValueTreeSharedObjectAddOrRemoveChildAction (*this, childIndex, {}));
                    }
                }
        */
    }
    
    pub fn remove_all_children(&mut self, undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            while (children.size() > 0)
                    removeChild (children.size() - 1, undoManager);
        */
    }
    
    pub fn move_child(&mut self, 
        current_index: i32,
        new_index:     i32,
        undo_manager:  *mut UndoManager)  {
        
        todo!();
        /*
            // The source index must be a valid index!
                jassert (isPositiveAndBelow (currentIndex, children.size()));

                if (currentIndex != newIndex
                     && isPositiveAndBelow (currentIndex, children.size()))
                {
                    if (undoManager == nullptr)
                    {
                        children.move (currentIndex, newIndex);
                        sendChildOrderChangedMessage (currentIndex, newIndex);
                    }
                    else
                    {
                        if (! isPositiveAndBelow (newIndex, children.size()))
                            newIndex = children.size() - 1;

                        undoManager->perform (new ValueTreeSharedObjectMoveChildAction (*this, currentIndex, newIndex));
                    }
                }
        */
    }
    
    pub fn reorder_children(&mut self, 
        new_order:    &Vec<ValueTree>,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            jassert (newOrder.size() == children.size());

                for (int i = 0; i < children.size(); ++i)
                {
                    auto* child = newOrder.getUnchecked (i)->object.get();

                    if (children.getObjectPointerUnchecked (i) != child)
                    {
                        auto oldIndex = children.indexOf (child);
                        jassert (oldIndex >= 0);
                        moveChild (oldIndex, i, undoManager);
                    }
                }
        */
    }
    
    pub fn is_equivalent_to(&self, other: &ValueTreeSharedObject) -> bool {
        
        todo!();
        /*
            if (type != other.type
                     || properties.size() != other.properties.size()
                     || children.size() != other.children.size()
                     || properties != other.properties)
                    return false;

                for (int i = 0; i < children.size(); ++i)
                    if (! children.getObjectPointerUnchecked (i)->isEquivalentTo (*other.children.getObjectPointerUnchecked (i)))
                        return false;

                return true;
        */
    }
    
    pub fn create_xml(&self) -> *mut XmlElement {
        
        todo!();
        /*
            auto* xml = new XmlElement (type);
                properties.copyToXmlAttributes (*xml);

                // (NB: it's faster to add nodes to XML elements in reverse order)
                for (auto i = children.size(); --i >= 0;)
                    xml->prependChildElement (children.getObjectPointerUnchecked (i)->createXml());

                return xml;
        */
    }
    
    pub fn write_to_stream<W: Write>(&self, output: &mut W)  {
        
        todo!();
        /*
            output.writeString (type.toString());
                output.writeCompressedInt (properties.size());

                for (int j = 0; j < properties.size(); ++j)
                {
                    output.writeString (properties.getName (j).toString());
                    properties.getValueAt (j).writeToStream (output);
                }

                output.writeCompressedInt (children.size());

                for (auto* c : children)
                    writeObjectToStream (output, c);
        */
    }
    
    pub fn write_object_to_stream<W: Write>(
        output: &mut W,
        object: *const ValueTreeSharedObject

    ) {
        
        todo!();
        /*
            if (object != nullptr)
                {
                    object->writeToStream (output);
                }
                else
                {
                    output.writeString ({});
                    output.writeCompressedInt (0);
                    output.writeCompressedInt (0);
                }
        */
    }
}
