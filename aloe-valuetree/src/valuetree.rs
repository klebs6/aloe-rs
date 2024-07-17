crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/values/aloe_ValueTree.h]

/**
  | A powerful tree structure that can be
  | used to hold free-form data, and which
  | can handle its own undo and redo behaviour.
  | 
  | A ValueTree contains a list of named
  | properties as var objects, and also
  | holds any number of sub-trees.
  | 
  | Create ValueTree objects on the stack,
  | and don't be afraid to copy them around,
  | as they're simply a lightweight reference
  | to a shared data container. Creating
  | a copy of another ValueTree simply creates
  | a new reference to the same underlying
  | object - to make a separate, deep copy
  | of a tree you should explicitly call
  | createCopy().
  | 
  | Each ValueTree has a type name, in much
  | the same way as an XmlElement has a tag
  | name, and much of the structure of a ValueTree
  | is similar to an XmlElement tree. You
  | can convert a ValueTree to and from an
  | XmlElement, and as long as the XML doesn't
  | contain text elements, the conversion
  | works well and makes a good serialisation
  | format. They can also be serialised
  | to a binary format, which is very fast
  | and compact.
  | 
  | All the methods that change data take
  | an optional UndoManager, which will
  | be used to track any changes to the object.
  | For this to work, you have to be careful
  | to consistently always use the same
  | UndoManager for all operations to any
  | sub-tree inside the tree.
  | 
  | A ValueTree can only be a child of one
  | parent at a time, so if you're moving
  | one from one tree to another, be careful
  | to always remove it first, before adding
  | it. This could also mess up your undo/redo
  | chain, so be wary! In a debug build you
  | should hit assertions if you try to do
  | anything dangerous, but there are still
  | plenty of ways it could go wrong.
  | 
  | Note that although the children in a
  | tree have a fixed order, the properties
  | are not guaranteed to be stored in any
  | particular order, so don't expect that
  | a property's index will correspond
  | to the order in which the property was
  | added, or that it will remain constant
  | when other properties are added or removed.
  | 
  | Listeners can be added to a ValueTree
  | to be told when properties change and
  | when sub-trees are added or removed.
  | 
  | @see var, XmlElement
  | 
  | @tags{DataStructures}
  |
  */
pub struct ValueTree {
    object:    ReferenceCountedObjectPtr<ValueTreeSharedObject>,
    listeners: ListenerList<Box<dyn ValueTreeListener>>,
}

impl Default for ValueTree {
    
    /**
      | Creates an empty, invalid ValueTree.
      | 
      | A ValueTree that is created with this
      | constructor can't actually be used
      | for anything, it's just a default 'null'
      | ValueTree that can be returned to indicate
      | some sort of failure. To create a real
      | one, use the constructor that takes
      | a string.
      |
      */
    fn default() -> Self {
        todo!();
        /*

        
        */
    }
}

impl PartialEq<ValueTree> for ValueTree {
    
    /**
      | Returns true if both this and the other
      | tree refer to the same underlying structure.
      | 
      | -----------
      | @note
      | 
      | this isn't a value comparison - two independently-created
      | trees which contain identical data
      | are NOT considered equal.
      |
      */
    #[inline] fn eq(&self, other: &ValueTree) -> bool {
        todo!();
        /*
            return object == other.object;
        */
    }
}

impl Eq for ValueTree {}

impl Index<&Identifier> for ValueTree {

    type Output = Var;
    
    /**
      | Returns the value of a named property.
      | 
      | If no such property has been set, this
      | will return a void variant. This is the
      | same as calling getProperty(). @see
      | getProperty
      |
      */
    #[inline] fn index(&self, name: &Identifier) -> &Self::Output {
        todo!();
        /*
            return object == nullptr ? getNullVarRef() : object->properties[name];
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/values/aloe_ValueTree.cpp]
impl Drop for ValueTree {

    fn drop(&mut self) {
        todo!();
        /* 
        if (! listeners.isEmpty() && object != nullptr)
            object->valueTreesWithListeners.removeValue (this);
     */
    }
}


pub fn get_null_var_ref() -> &'static Var {
    
    todo!();
    /*
        static var nullVar;
        return nullVar;
    */
}

impl ValueTree {

    /** 
     | Creates a value tree from nested lists of
     | properties and ValueTrees.
     |
     |    This code,
     |
     |    @code
     |    ValueTree groups
     |    { "ParameterGroups", {},
     |      {
     |        { "Group", {{ "name", "Tone Controls" }},
     |          {
     |            { "Parameter", {{ "id", "distortion" }, { "value", 0.5 }}},
     |            { "Parameter", {{ "id", "reverb" },     { "value", 0.5 }}}
     |          }
     |        },
     |        { "Group", {{ "name", "Other Controls" }},
     |          {
     |            { "Parameter", {{ "id", "drywet" }, { "value", 0.5 }}},
     |            { "Parameter", {{ "id", "gain" },   { "value", 0.5 }}}
     |          }
     |        }
     |      }
     |    };
     |    @endcode
     |
     |    produces this tree:
     |
     |    @verbatim
     |    <ParameterGroups>
     |      <Group name="Tone Controls">
     |        <Parameter id="distortion" value="0.5"/>
     |        <Parameter id="reverb" value="0.5"/>
     |      </Group>
     |      <Group name="Other Controls">
     |        <Parameter id="drywet" value="0.5"/>
     |        <Parameter id="gain" value="0.5"/>
     |      </Group>
     |    </ParameterGroups>
     |    @endverbatim
    */
    pub fn new_from_ty_props_and_subtrees(
        ty:         &Identifier,
        properties: &[named_value_set::NamedValue],
        sub_trees:  &[ValueTree]

    ) -> Self {
    
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if this tree refers to some
      | valid data.
      | 
      | An invalid tree is one that was created
      | with the default constructor.
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return object != nullptr;
        */
    }

    /**
      | Returns a start iterator for the children
      | in this tree.
      |
      */
    pub fn iter(&self) -> std::slice::Iter<'_, ValueTree> {
        todo!();
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, ValueTree> {
        todo!();
    }

    /**
      | This method uses a comparator object
      | to sort the tree's children into order.
      | 
      | The object provided must have a method
      | of the form: @code int compareElements
      | (const ValueTree& first, const ValueTree&
      | second); @endcode
      | 
      | ..and this method must return:
      | 
      | - a value of < 0 if the first comes before
      | the second
      | 
      | - a value of 0 if the two objects are equivalent
      | 
      | - a value of > 0 if the second comes before
      | the first
      | 
      | To improve performance, the compareElements()
      | method can be declared as static or const.
      | 
      | -----------
      | @param comparator
      | 
      | the comparator to use for comparing
      | elements.
      | ----------
      | @param undoManager
      | 
      | optional UndoManager for storing the
      | changes
      | ----------
      | @param retainOrderOfEquivalentItems
      | 
      | if this is true, then items which the
      | comparator says are equivalent will
      | be kept in the order in which they currently
      | appear in the array. This is slower to
      | perform, but may be important in some
      | cases. If it's false, a faster algorithm
      | is used, but equivalent elements may
      | be rearranged.
      |
      */
    pub fn sort<ElementComparator>(&mut self, 
        comparator:                       &mut ElementComparator,
        undo_manager:                     *mut UndoManager,
        retain_order_of_equivalent_items: bool)  {
    
        todo!();
        /*
            if (object != nullptr)
            {
                Vec<Box<ValueTree>> sortedList;
                createListOfChildren (sortedList);
                ValueTreeComparatorAdapter<ElementComparator> adapter (comparator);
                sortedList.sort (adapter, retainOrderOfEquivalentItems);
                reorderChildren (sortedList, undoManager);
            }
        */
    }

    /**
      | Creates an empty ValueTree with the
      | given type name.
      | 
      | Like an XmlElement, each ValueTree
      | has a type, which you can access with
      | getType() and hasType().
      |
      */
    pub fn new_from_id_ref(ty: &Identifier) -> Self {
    
        todo!();
        /*


            : object (new ValueTree::ValueTreeSharedObject (type))

        jassert (type.toString().isNotEmpty()); // All objects must be given a sensible type name!
        */
    }
    
    pub fn new_with_ty_props_and_subtrees(
        ty:         &Identifier,
        properties: &[named_value_set::NamedValue],
        sub_trees:  &[ValueTree]

    ) -> Self {
    
        todo!();
        /*
        : value_tree(type),

            object->properties = NamedValueSet (std::move (properties));

        for (auto& tree : subTrees)
            addChild (tree, -1, nullptr);
        */
    }
    
    pub fn new_from_shared_object(so: ValueTreeSharedObjectPtr) -> Self {
    
        todo!();
        /*
        : object(std::move (so)),

        
        */
    }
    
    pub fn new_from_shared_object_ref(so: &mut ValueTreeSharedObject) -> Self {
    
        todo!();
        /*
        : object(so),

        
        */
    }
    
    /**
      | Creates a reference to another ValueTree.
      |
      */
    pub fn new_from_other_ref(other: &ValueTree) -> Self {
    
        todo!();
        /*
        : object(other.object),

        
        */
    }
    
    /**
      | Changes this object to be a reference
      | to the given tree.
      | 
      | -----------
      | @note
      | 
      | calling this just points this at the
      | new object and invokes the ValueTreeListener::valueTreeRedirected
      | callback, but it's not an undoable operation.
      | If you're trying to replace an entire
      | tree in an undoable way, you probably
      | want to use copyPropertiesAndChildrenFrom()
      | instead.
      |
      */
    pub fn assign_from(&mut self, other: &ValueTree) -> &mut ValueTree {
        
        todo!();
        /*
            if (object != other.object)
        {
            if (listeners.isEmpty())
            {
                object = other.object;
            }
            else
            {
                if (object != nullptr)
                    object->valueTreesWithListeners.removeValue (this);

                if (other.object != nullptr)
                    other.object->valueTreesWithListeners.add (this);

                object = other.object;

                listeners.call ([this] (ValueTreeListener& l) { l.valueTreeRedirected (*this); });
            }
        }

        return *this;
        */
    }
    
    pub fn new_from_other(other: ValueTree) -> Self {
    
        todo!();
        /*
        : object(std::move (other.object)),

            if (object != nullptr)
            object->valueTreesWithListeners.removeValue (&other);
        */
    }
    
    /**
      | Performs a deep comparison between
      | the properties and children of two trees.
      | 
      | If all the properties and children of
      | the two trees are the same (recursively),
      | this returns true.
      | 
      | The normal operator==() only checks
      | whether two trees refer to the same shared
      | data structure, so use this method if
      | you need to do a proper value comparison.
      |
      */
    pub fn is_equivalent_to(&self, other: &ValueTree) -> bool {
        
        todo!();
        /*
            return object == other.object
                || (object != nullptr && other.object != nullptr
                     && object->isEquivalentTo (*other.object));
        */
    }
    
    /**
      | Returns a deep copy of this tree and all
      | its sub-trees.
      |
      */
    pub fn create_copy(&self) -> ValueTree {
        
        todo!();
        /*
            if (object != nullptr)
            return ValueTree (*new ValueTreeSharedObject (*object));

        return {};
        */
    }
    
    /**
      | Overwrites all the properties in this
      | tree with the properties of the source
      | tree.
      | 
      | Any properties that already exist will
      | be updated; and new ones will be added,
      | and any that are not present in the source
      | tree will be removed. @see copyPropertiesAndChildrenFrom
      |
      */
    pub fn copy_properties_from(&mut self, 
        source:       &ValueTree,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            jassert (object != nullptr || source.object == nullptr); // Trying to add properties to a null ValueTree will fail!

        if (source.object == nullptr)
            removeAllProperties (undoManager);
        else if (object != nullptr)
            object->copyPropertiesFrom (*(source.object), undoManager);
        */
    }
    
    /**
      | Replaces all children and properties
      | of this object with copies of those from
      | the source object.
      | 
      | @see copyPropertiesFrom
      |
      */
    pub fn copy_properties_and_children_from(&mut self, 
        source:       &ValueTree,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            jassert (object != nullptr || source.object == nullptr); // Trying to copy to a null ValueTree will fail!

        copyPropertiesFrom (source, undoManager);
        removeAllChildren (undoManager);

        if (object != nullptr && source.object != nullptr)
            for (auto& child : source.object->children)
                object->addChild (createCopyIfNotNull (child), -1, undoManager);
        */
    }
    
    /**
      | Returns true if the tree has this type.
      | 
      | The comparison is case-sensitive.
      | @see getType
      |
      */
    pub fn has_type(&self, type_name: &Identifier) -> bool {
        
        todo!();
        /*
            return object != nullptr && object->type == typeName;
        */
    }
    
    /**
      | Returns the type of this tree.
      | 
      | The type is specified when the ValueTree
      | is created. @see hasType
      |
      */
    pub fn get_type(&self) -> Identifier {
        
        todo!();
        /*
            return object != nullptr ? object->type : Identifier();
        */
    }
    
    /**
      | Returns the parent tree that contains
      | this one.
      | 
      | If the tree has no parent, this will return
      | an invalid object. (You can use isValid()
      | to check whether a tree is valid)
      |
      */
    pub fn get_parent(&self) -> ValueTree {
        
        todo!();
        /*
            if (object != nullptr)
            if (auto p = object->parent)
                return ValueTree (*p);

        return {};
        */
    }
    
    /**
      | Recursively finds the highest-level
      | parent tree that contains this one.
      | 
      | If the tree has no parent, this will return
      | itself.
      |
      */
    pub fn get_root(&self) -> ValueTree {
        
        todo!();
        /*
            if (object != nullptr)
            return ValueTree (object->getRoot());

        return {};
        */
    }
    
    /**
      | Returns one of this tree's siblings
      | in its parent's child list.
      | 
      | The delta specifies how far to move through
      | the list, so a value of 1 would return
      | the tree that follows this one, -1 would
      | return the tree before it, 0 will return
      | this one, etc.
      | 
      | If the requested position is beyond
      | the start or end of the child list, this
      | will return an invalid object.
      |
      */
    pub fn get_sibling(&self, delta: i32) -> ValueTree {
        
        todo!();
        /*
            if (object != nullptr)
            if (auto* p = object->parent)
                if (auto* c = p->children.getObjectPointer (p->indexOf (*this) + delta))
                    return ValueTree (*c);

        return {};
        */
    }
    
    /**
      | Returns the value of a named property.
      | 
      | If no such property has been set, this
      | will return a void variant.
      | 
      | You can also use operator[] to get a property.
      | @see var, setProperty, getPropertyPointer,
      | hasProperty
      |
      */
    pub fn get_property(&self, name: &Identifier) -> &Var {
        
        todo!();
        /*
            return object == nullptr ? getNullVarRef() : object->properties[name];
        */
    }
    
    /**
      | Returns the value of a named property,
      | or the value of defaultReturnValue
      | if the property doesn't exist.
      | 
      | You can also use operator[] and getProperty
      | to get a property.
      | 
      | @see var, getProperty, getPropertyPointer,
      | setProperty, hasProperty
      |
      */
    pub fn get_property_with_default(
        &self, 
        name:                 &Identifier,
        default_return_value: &Var

    ) -> Var {
        
        todo!();
        /*
            return object == nullptr ? defaultReturnValue
                                 : object->properties.getWithDefault (name, defaultReturnValue);
        */
    }

    /**
      | Returns a pointer to the value of a named
      | property, or nullptr if the property
      | doesn't exist.
      | 
      | @see var, getProperty, setProperty,
      | hasProperty
      |
      */
    pub fn get_property_pointer(&self, name: &Identifier) -> *const Var {
        
        todo!();
        /*
            return object == nullptr ? nullptr
                                 : object->properties.getVarPointer (name);
        */
    }

    /**
      | Changes a named property of the tree.
      | 
      | The name identifier must not be an empty
      | string.
      | 
      | If the undoManager parameter is not
      | nullptr, its UndoManager::perform()
      | method will be used, so that this change
      | can be undone. Be very careful not to
      | mix undoable and non-undoable changes!
      | 
      | @see var, getProperty, removeProperty
      | 
      | -----------
      | @return
      | 
      | a reference to the value tree, so that
      | you can daisy-chain calls to this method.
      |
      */
    pub fn set_property(&mut self, 
        name:         &Identifier,
        new_value:    &Var,
        undo_manager: *mut UndoManager) -> &mut ValueTree {
        
        todo!();
        /*
            return setPropertyExcludingListener (nullptr, name, newValue, undoManager);
        */
    }

    /**
      | Changes a named property of the tree,
      | but will not notify a specified listener
      | of the change. @see setProperty
      |
      */
    pub fn set_property_excluding_listener(
        &mut self, 
        listener_to_exclude: *mut dyn ValueTreeListener,
        name:                &Identifier,
        new_value:           &Var,
        undo_manager:        *mut UndoManager

    ) -> &mut ValueTree {
        
        todo!();
        /*
            jassert (name.toString().isNotEmpty()); // Must have a valid property name!
        jassert (object != nullptr); // Trying to add a property to a null ValueTree will fail!

        if (object != nullptr)
            object->setProperty (name, newValue, undoManager, listenerToExclude);

        return *this;
        */
    }

    /**
      | Returns true if the tree contains a named
      | property.
      |
      */
    pub fn has_property(&self, name: &Identifier) -> bool {
        
        todo!();
        /*
            return object != nullptr && object->hasProperty (name);
        */
    }

    /**
      | Removes a property from the tree.
      | 
      | If the undoManager parameter is not
      | nullptr, its UndoManager::perform()
      | method will be used, so that this change
      | can be undone. Be very careful not to
      | mix undoable and non-undoable changes!
      |
      */
    pub fn remove_property(&mut self, 
        name:         &Identifier,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            if (object != nullptr)
            object->removeProperty (name, undoManager);
        */
    }

    /**
      | Removes all properties from the tree.
      | 
      | If the undoManager parameter is not
      | nullptr, its UndoManager::perform()
      | method will be used, so that this change
      | can be undone. Be very careful not to
      | mix undoable and non-undoable changes!
      |
      */
    pub fn remove_all_properties(&mut self, undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            if (object != nullptr)
            object->removeAllProperties (undoManager);
        */
    }

    /**
      | Returns the total number of properties
      | that the tree contains. @see getProperty.
      |
      */
    pub fn get_num_properties(&self) -> i32 {
        
        todo!();
        /*
            return object == nullptr ? 0 : object->properties.size();
        */
    }

    /**
      | Returns the identifier of the property
      | with a given index.
      | 
      | -----------
      | @note
      | 
      | that properties are not guaranteed
      | to be stored in any particular order,
      | so don't expect that the index will correspond
      | to the order in which the property was
      | added, or that it will remain constant
      | when other properties are added or removed.
      | @see getNumProperties
      |
      */
    pub fn get_property_name(&self, index: i32) -> Identifier {
        
        todo!();
        /*
            return object == nullptr ? Identifier()
                                 : object->properties.getName (index);
        */
    }

    /**
      | Returns the total number of references
      | to the shared underlying data structure
      | that this ValueTree is using.
      |
      */
    pub fn get_reference_count(&self) -> i32 {
        
        todo!();
        /*
            return object != nullptr ? object->getReferenceCount() : 0;
        */
    }
    
    /**
      | Returns a Value object that can be used
      | to control and respond to one of the tree's
      | properties.
      | 
      | The Value object will maintain a reference
      | to this tree, and will use the undo manager
      | when it needs to change the value. Attaching
      | a Value::ValueTreeListener to the value object
      | will provide callbacks whenever the
      | property changes.
      | 
      | If shouldUpdateSynchronously is true
      | the Value::ValueTreeListener will be updated
      | synchronously. @see ValueSource::sendChangeMessage
      | (bool)
      |
      */
    pub fn get_property_as_value(
        &mut self, 
        name:                 &Identifier,
        undo_manager:         *mut UndoManager,
        update_synchronously: Option<bool>

    ) -> Value {

        let update_synchronously: bool = update_synchronously.unwrap_or(false);
        
        todo!();
        /*
            return Value (new ValueTreePropertyValueSource (*this, name, undoManager, updateSynchronously));
        */
    }

    /**
      | Returns the number of child trees inside
      | this one. @see getChild
      |
      */
    pub fn get_num_children(&self) -> i32 {
        
        todo!();
        /*
            return object == nullptr ? 0 : object->children.size();
        */
    }

    /**
      | Returns one of this tree's sub-trees.
      | 
      | If the index is out of range, it'll return
      | an invalid tree. (You can use isValid()
      | to check whether a tree is valid)
      |
      */
    pub fn get_child(&self, index: i32) -> ValueTree {
        
        todo!();
        /*
            if (object != nullptr)
            if (auto* c = object->children.getObjectPointer (index))
                return ValueTree (*c);

        return {};
        */
    }
    
    pub fn begin(&self) -> ValueTreeIterator {
        
        todo!();
        /*
            return ValueTreeIterator (*this, false);
        */
    }
    
    pub fn end(&self) -> ValueTreeIterator {
        
        todo!();
        /*
            return ValueTreeIterator (*this, true);
        */
    }

    /**
      | Returns the first sub-tree with the
      | specified type name.
      | 
      | If no such child tree exists, it'll return
      | an invalid tree. (You can use isValid()
      | to check whether a tree is valid) @see
      | getOrCreateChildWithName
      |
      */
    pub fn get_child_with_name(&self, ty: &Identifier) -> ValueTree {
        
        todo!();
        /*
            return object != nullptr ? object->getChildWithName (type) : ValueTree();
        */
    }

    /**
      | Returns the first sub-tree with the
      | specified type name, creating and adding
      | a child with this name if there wasn't
      | already one there.
      | 
      | The only time this will return an invalid
      | object is when the object that you're
      | calling the method on is itself invalid.
      | @see getChildWithName
      |
      */
    pub fn get_or_create_child_with_name(&mut self, 
        ty:           &Identifier,
        undo_manager: *mut UndoManager) -> ValueTree {
        
        todo!();
        /*
            return object != nullptr ? object->getOrCreateChildWithName (type, undoManager) : ValueTree();
        */
    }

    /**
      | Looks for the first sub-tree that has
      | the specified property value.
      | 
      | This will scan the child trees in order,
      | until it finds one that has property
      | that matches the specified value.
      | 
      | If no such tree is found, it'll return
      | an invalid object. (You can use isValid()
      | to check whether a tree is valid)
      |
      */
    pub fn get_child_with_property(&self, 
        property_name:  &Identifier,
        property_value: &Var) -> ValueTree {
        
        todo!();
        /*
            return object != nullptr ? object->getChildWithProperty (propertyName, propertyValue) : ValueTree();
        */
    }

    /**
      | Returns true if this tree is a sub-tree
      | (at any depth) of the given parent.
      | 
      | This searches recursively, so returns
      | true if it's a sub-tree at any level below
      | the parent.
      |
      */
    pub fn isa_child_of(&self, possible_parent: &ValueTree) -> bool {
        
        todo!();
        /*
            return object != nullptr && object->isAChildOf (possibleParent.object.get());
        */
    }

    /**
      | Returns the index of a child item in this
      | parent.
      | 
      | If the child isn't found, this returns
      | -1.
      |
      */
    pub fn index_of(&self, child: &ValueTree) -> i32 {
        
        todo!();
        /*
            return object != nullptr ? object->indexOf (child) : -1;
        */
    }

    /**
      | Adds a child to this tree.
      | 
      | Make sure that the child being added
      | has first been removed from any former
      | parent before calling this, or else
      | you'll hit an assertion.
      | 
      | If the index is < 0 or greater than the
      | current number of sub-trees, the new
      | one will be added at the end of the list.
      | 
      | If the undoManager parameter is not
      | nullptr, its UndoManager::perform()
      | method will be used, so that this change
      | can be undone. Be very careful not to
      | mix undoable and non-undoable changes!
      | 
      | @see appendChild, removeChild
      |
      */
    pub fn add_child(&mut self, 
        child:        &ValueTree,
        index:        i32,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            jassert (object != nullptr); // Trying to add a child to a null ValueTree!

        if (object != nullptr)
            object->addChild (child.object.get(), index, undoManager);
        */
    }

    /**
      | Appends a new child sub-tree to this
      | tree.
      | 
      | This is equivalent to calling addChild()
      | with an index of -1. See addChild() for
      | more details. @see addChild, removeChild
      |
      */
    pub fn append_child(&mut self, 
        child:        &ValueTree,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            addChild (child, -1, undoManager);
        */
    }

    /**
      | Removes a sub-tree from this tree.
      | 
      | If the index is out-of-range, nothing
      | will be changed.
      | 
      | If the undoManager parameter is not
      | nullptr, its UndoManager::perform()
      | method will be used, so that this change
      | can be undone. Be very careful not to
      | mix undoable and non-undoable changes!
      |
      */
    pub fn remove_child(&mut self, 
        child_index:  i32,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            if (object != nullptr)
            object->removeChild (childIndex, undoManager);
        */
    }

    /**
      | Removes the specified child from this
      | tree's child-list.
      | 
      | If the undoManager parameter is not
      | nullptr, its UndoManager::perform()
      | method will be used, so that this change
      | can be undone. Be very careful not to
      | mix undoable and non-undoable changes!
      |
      */
    pub fn remove_child_tree(&mut self, 
        child:        &ValueTree,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            if (object != nullptr)
            object->removeChild (object->children.indexOf (child.object), undoManager);
        */
    }

    /**
      | Removes all child-trees.
      | 
      | If the undoManager parameter is not
      | nullptr, its UndoManager::perform()
      | method will be used, so that this change
      | can be undone. Be very careful not to
      | mix undoable and non-undoable changes!
      |
      */
    pub fn remove_all_children(&mut self, undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            if (object != nullptr)
            object->removeAllChildren (undoManager);
        */
    }

    /**
      | Moves one of the sub-trees to a different
      | index.
      | 
      | This will move the child to a specified
      | index, shuffling along any intervening
      | items as required. So for example, if
      | you have a list of { 0, 1, 2, 3, 4, 5 }, then
      | calling move (2, 4) would result in {
      | 0, 1, 3, 4, 2, 5 }.
      | 
      | -----------
      | @param currentIndex
      | 
      | the index of the item to be moved. If this
      | isn't a valid index, then nothing will
      | be done
      | ----------
      | @param newIndex
      | 
      | the index at which you'd like this item
      | to end up. If this is less than zero, the
      | value will be moved to the end of the list
      | ----------
      | @param undoManager
      | 
      | the optional UndoManager to use to store
      | this transaction
      |
      */
    pub fn move_child(&mut self, 
        current_index: i32,
        new_index:     i32,
        undo_manager:  *mut UndoManager)  {
        
        todo!();
        /*
            if (object != nullptr)
            object->moveChild (currentIndex, newIndex, undoManager);
        */
    }

    pub fn create_list_of_children(&self, list: &mut Vec<ValueTree>)  {
        
        todo!();
        /*
            if (object != nullptr)
            for (auto* o : object->children)
                if (o != nullptr)
                    list.add (new ValueTree (*o));
        */
    }

    pub fn reorder_children(&mut self, 
        new_order:    &Vec<ValueTree>,
        undo_manager: *mut UndoManager)  {
        
        todo!();
        /*
            if (object != nullptr)
            object->reorderChildren (newOrder, undoManager);
        */
    }

    /**
      | Adds a listener to receive callbacks
      | when this tree is changed in some way.
      | 
      | The listener is added to this specific
      | ValueTree object, and not to the shared
      | object that it refers to. When this object
      | is deleted, all the listeners will be
      | lost, even if other references to the
      | same ValueTree still exist. And if you
      | use the operator= to make this refer
      | to a different ValueTree, any listeners
      | will begin listening to changes to the
      | new tree instead of the old one.
      | 
      | When you're adding a listener, make
      | sure that you add it to a ValueTree instance
      | that will last for as long as you need
      | the listener. In general, you'd never
      | want to add a listener to a local stack-based
      | ValueTree, and would usually add one
      | to a member variable.
      | 
      | @see removeListener
      |
      */
    pub fn add_listener(&mut self, listener: *mut dyn ValueTreeListener)  {
        
        todo!();
        /*
            if (listener != nullptr)
        {
            if (listeners.isEmpty() && object != nullptr)
                object->valueTreesWithListeners.add (this);

            listeners.add (listener);
        }
        */
    }

    /**
      | Removes a listener that was previously
      | added with addListener().
      |
      */
    pub fn remove_listener(&mut self, listener: *mut dyn ValueTreeListener)  {
        
        todo!();
        /*
            listeners.remove (listener);

        if (listeners.isEmpty() && object != nullptr)
            object->valueTreesWithListeners.removeValue (this);
        */
    }

    /**
      | Causes a property-change callback
      | to be triggered for the specified property,
      | calling any listeners that are registered.
      |
      */
    pub fn send_property_change_message(&mut self, property: &Identifier)  {
        
        todo!();
        /*
            if (object != nullptr)
            object->sendPropertyChangeMessage (property);
        */
    }

    /**
      | Creates an XmlElement that holds a complete
      | image of this tree and all its children.
      | 
      | If this tree is invalid, this may return
      | nullptr. Otherwise, the XML that is
      | produced can be used to recreate a similar
      | tree by calling ValueTree::fromXml().
      | @see fromXml, toXmlString
      |
      */
    pub fn create_xml(&self) -> Box<XmlElement> {
        
        todo!();
        /*
            return std::unique_ptr<XmlElement> (object != nullptr ? object->createXml() : nullptr);
        */
    }

    /**
      | Tries to recreate a tree from its XML
      | representation.
      | 
      | This isn't designed to cope with random
      | XML data - it should only be fed XML that
      | was created by the createXml() method.
      |
      */
    pub fn from_xml(&mut self, xml: &XmlElement) -> ValueTree {
        
        todo!();
        /*
            if (! xml.isTextElement())
        {
            ValueTree v (xml.getTagName());
            v.object->properties.setFromXmlAttributes (xml);

            for (auto* e : xml.getChildIterator())
                v.appendChild (fromXml (*e), nullptr);

            return v;
        }

        // ValueTrees don't have any equivalent to XML text elements!
        jassertfalse;
        return {};
        */
    }

    /**
      | Tries to recreate a tree from its XML
      | representation.
      | 
      | This isn't designed to cope with random
      | XML data - it should only be fed XML that
      | was created by the createXml() method.
      |
      */
    pub fn from_xml_string(&mut self, xml_text: &String) -> ValueTree {
        
        todo!();
        /*
            if (auto xml = parseXML (xmlText))
            return fromXml (*xml);

        return {};
        */
    }

    /**
      | This returns a string containing an
      | XML representation of the tree.
      | 
      | This is quite handy for debugging purposes,
      | as it provides a quick way to view a tree.
      | @see createXml()
      |
      */
    pub fn to_xml_string(&self, format: &xml_element::TextFormat) -> String {
        
        todo!();
        /*
            if (auto xml = createXml())
            return xml->toString (format);

        return {};
        */
    }

    /**
      | Stores this tree (and all its children)
      | in a binary format.
      | 
      | Once written, the data can be read back
      | with readFromStream().
      | 
      | It's much faster to load/save your tree
      | in binary form than as XML, but obviously
      | isn't human-readable.
      |
      */
    pub fn write_to_stream<W: Write>(&self, output: &mut W)  {
        
        todo!();
        /*
            ValueTreeSharedObject::writeObjectToStream (output, object.get());
        */
    }

    /**
      | Reloads a tree from a stream that was
      | written with writeToStream().
      |
      */
    pub fn read_from_stream<R: Read>(&mut self, input: &mut R) -> ValueTree {
        
        todo!();
        /*
            auto type = input.readString();

        if (type.isEmpty())
            return {};

        ValueTree v (type);

        auto numProps = input.readCompressedInt();

        if (numProps < 0)
        {
            jassertfalse;  // trying to read corrupted data!
            return v;
        }

        for (int i = 0; i < numProps; ++i)
        {
            auto name = input.readString();

            if (name.isNotEmpty())
                v.object->properties.set (name, var::readFromStream (input));
            else
                jassertfalse;  // trying to read corrupted data!
        }

        auto numChildren = input.readCompressedInt();
        v.object->children.ensureStorageAllocated (numChildren);

        for (int i = 0; i < numChildren; ++i)
        {
            auto child = readFromStream (input);

            if (! child.isValid())
                return v;

            v.object->children.add (child.object);
            child.object->parent = v.object.get();
        }

        return v;
        */
    }

    /**
      | Reloads a tree from a data block that
      | was written with writeToStream().
      |
      */
    pub fn read_from_data(&mut self, 
        data:      *const c_void,
        num_bytes: usize) -> ValueTree {
        
        todo!();
        /*
            MemoryInputStream in (data, numBytes, false);
        return readFromStream (in);
        */
    }

    /**
      | Reloads a tree from a data block that
      | was written with writeToStream() and
      | then zipped using GZIPCompressorOutputStream.
      |
      */
    pub fn read_from_gzip_data(&mut self, 
        data:      *const c_void,
        num_bytes: usize) -> ValueTree {
        
        todo!();
        /*
            MemoryInputStream in (data, numBytes, false);
        GZIPDecompressorInputStream gzipStream (in);
        return readFromStream (gzipStream);
        */
    }
}
