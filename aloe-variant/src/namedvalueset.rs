crate::ix!();

pub fn get_null_var_ref<'a>() -> &'a Var {
    
    todo!();
    /*
        static var nullVar;
        return nullVar;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_NamedValueSet.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_NamedValueSet.cpp]

/**
  | Holds a set of named var objects.
  | 
  | This can be used as a basic structure
  | to hold a set of var object, which can
  | be retrieved by using their identifier.
  | 
  | @tags{Core}
  |
  */
pub struct NamedValueSet {
    values: Vec<named_value_set::NamedValue>,
}

pub mod named_value_set {

    use super::*;

    /**
      | Structure for a named var object
      |
      */
    pub struct NamedValue {
        name:  Identifier,
        value: Var,
    }

    impl PartialEq<NamedValue> for NamedValue {
        
        #[inline] fn eq(&self, other: &NamedValue) -> bool {
            todo!();
            /*
                return name == other.name && value == other.value;
            */
        }
    }

    impl Eq for NamedValue {}

    impl NamedValue {
        
        pub fn new_from_identifier_ref_and_var_ref(
            n: &Identifier,
            v: &Var) -> Self {
        
            todo!();
            /*
            : name(n),
            : value(v),
            */
        }
        
        pub fn new_from_named_value_ref(other: &NamedValue) -> Self {
        
            todo!();
            /*
            : named_value(other.name, other.value),
            */
        }
        
        pub fn new_from_named_value(other: NamedValue) -> Self {
        
            todo!();
            /*
            : NamedValue (std::move (other.name),
                     std::move (other.value))
            */
        }
        
        pub fn new_from_identifier_ref_and_var(
            n: &Identifier,
            v: Var) -> Self {
        
            todo!();
            /*
            : name(n),
            : value(std::move (v)),
            */
        }
        
        pub fn new_from_identifier_and_var(
            n: Identifier,
            v: Var) -> Self {
        
            todo!();
            /*
            : name(std::move (n)),
            : value(std::move (v)),
            */
        }
        
        pub fn assign_from(&mut self, other: NamedValue) -> &mut NamedValue {
            
            todo!();
            /*
            name = std::move (other.name);
            value = std::move (other.value);
            return *this;
            */
        }
    }
}

impl Default for NamedValueSet {
    
    /**
      | Creates an empty set.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl Eq for NamedValueSet {}

impl PartialEq<NamedValueSet> for NamedValueSet {
    
    /**
      | Two NamedValueSets are considered
      | equal if they contain all the same key/value
      | pairs, regardless of the order.
      |
      */
    #[inline] fn eq(&self, other: &NamedValueSet) -> bool {
        todo!();
        /*
            auto num = values.size();

        if (num != other.values.size())
            return false;

        for (int i = 0; i < num; ++i)
        {
            // optimise for the case where the keys are in the same order
            if (values.getReference(i).name == other.values.getReference(i).name)
            {
                if (values.getReference(i).value != other.values.getReference(i).value)
                    return false;
            }
            else
            {
                // if we encounter keys that are in a different order, search remaining items by brute force..
                for (int j = i; j < num; ++j)
                {
                    if (auto* otherVal = other.getVarPointer (values.getReference(j).name))
                        if (values.getReference(j).value == *otherVal)
                            continue;

                    return false;
                }

                return true;
            }
        }

        return true;
        */
    }
}

impl Index<&Identifier> for NamedValueSet {

    type Output = Var;
    
    /**
      | Returns the value of a named item.
      | 
      | If the name isn't found, this will return
      | a void variant.
      |
      */
    #[inline] fn index(&self, name: &Identifier) -> &Self::Output {
        todo!();
        /*
            if (auto* v = getVarPointer (name))
            return *v;

        return getNullVarRef();
        */
    }
}

impl NamedValueSet {

    pub fn begin(&self) -> *const named_value_set::NamedValue {
        
        todo!();
        /*
            return values.begin();
        */
    }
    
    pub fn end(&self) -> *const named_value_set::NamedValue {
        
        todo!();
        /*
            return values.end();
        */
    }
    
    pub fn new_from_other_ref(other: &NamedValueSet) -> Self {
    
        todo!();
        /*
        : values(other.values),

        
        */
    }
    
    pub fn new_from_other(other: NamedValueSet) -> Self {
    
        todo!();
        /*
        : values(std::move (other.values)),

        
        */
    }
    
    /**
      | Creates a NamedValueSet from a list
      | of names and properties.
      |
      */
    pub fn new_from_list_of_names_and_properties(list: &[named_value_set::NamedValue]) -> Self {
    
        todo!();
        /*
        : values(std::move (list)),

        
        */
    }
    
    pub fn assign_from_ref(&mut self, other: &NamedValueSet) -> &mut NamedValueSet {
        
        todo!();
        /*
            clear();
        values = other.values;
        return *this;
        */
    }
    
    pub fn assign_from(&mut self, other: NamedValueSet) -> &mut NamedValueSet {
        
        todo!();
        /*
            other.values.swapWith (values);
        return *this;
        */
    }
    
    /**
      | Removes all values.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            values.clear();
        */
    }
    
    /**
      | Returns the total number of values that
      | the set contains.
      |
      */
    pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return values.size();
        */
    }
    
    /**
      | Returns true if the set is empty.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return values.isEmpty();
        */
    }
    
    /**
      | Tries to return the named value, but
      | if no such value is found, this will instead
      | return the supplied default value.
      |
      */
    pub fn get_with_default(&self, 
        name:                 &Identifier,
        default_return_value: &Var) -> Var {
        
        todo!();
        /*
            if (auto* v = getVarPointer (name))
            return *v;

        return defaultReturnValue;
        */
    }
    
    /**
      | Returns a pointer to the var that holds
      | a named value, or null if there is no value
      | with this name.
      | 
      | Do not use this method unless you really
      | need access to the internal var object
      | for some reason - for normal reading
      | and writing always prefer operator[]()
      | and set(). Also note that the pointer
      | returned may become invalid as soon
      | as any subsequent methods are called
      | on the NamedValueSet.
      |
      */
    pub fn get_var_pointer_mut(&mut self, name: &Identifier) -> *mut Var {
        
        todo!();
        /*
            for (auto& i : values)
            if (i.name == name)
                return &(i.value);

        return {};
        */
    }
    
    /**
      | Returns a pointer to the var that holds
      | a named value, or null if there is no value
      | with this name.
      | 
      | Do not use this method unless you really
      | need access to the internal var object
      | for some reason - for normal reading
      | and writing always prefer operator[]()
      | and set(). Also note that the pointer
      | returned may become invalid as soon
      | as any subsequent methods are called
      | on the NamedValueSet.
      |
      */
    pub fn get_var_pointer(&self, name: &Identifier) -> *const Var {
        
        todo!();
        /*
            for (auto& i : values)
            if (i.name == name)
                return &(i.value);

        return {};
        */
    }
    
    /**
      | Changes or adds a named value.
      | 
      | -----------
      | @return
      | 
      | true if a value was changed or added;
      | false if the value was already set the
      | value passed-in.
      |
      */
    pub fn set(
        &mut self, 
        name:      &Identifier,
        new_value: Var) -> bool {
        
        todo!();
        /*
            if (auto* v = getVarPointer (name))
        {
            if (v->equalsWithSameType (newValue))
                return false;

            *v = std::move (newValue);
            return true;
        }

        values.add ({ name, std::move (newValue) });
        return true;
        */
    }
    
    /**
      | Changes or adds a named value.
      | 
      | -----------
      | @return
      | 
      | true if a value was changed or added;
      | false if the value was already set the
      | value passed-in.
      |
      */
    pub fn set_with_ref(
        &mut self, 
        name:      &Identifier,
        new_value: &Var) -> bool {
        
        todo!();
        /*
            if (auto* v = getVarPointer (name))
        {
            if (v->equalsWithSameType (newValue))
                return false;

            *v = newValue;
            return true;
        }

        values.add ({ name, newValue });
        return true;
        */
    }
    
    /**
      | Returns true if the set contains an item
      | with the specified name.
      |
      */
    pub fn contains(&self, name: &Identifier) -> bool {
        
        todo!();
        /*
            return getVarPointer (name) != nullptr;
        */
    }
    
    /**
      | Returns the index of the given name,
      | or -1 if it's not found.
      |
      */
    pub fn index_of(&self, name: &Identifier) -> i32 {
        
        todo!();
        /*
            auto numValues = values.size();

        for (int i = 0; i < numValues; ++i)
            if (values.getReference(i).name == name)
                return i;

        return -1;
        */
    }
    
    /**
      | Removes a value from the set.
      | 
      | -----------
      | @return
      | 
      | true if a value was removed; false if
      | there was no value with the name that
      | was given.
      |
      */
    pub fn remove(&mut self, name: &Identifier) -> bool {
        
        todo!();
        /*
            auto numValues = values.size();

        for (int i = 0; i < numValues; ++i)
        {
            if (values.getReference(i).name == name)
            {
                values.remove (i);
                return true;
            }
        }

        return false;
        */
    }
    
    /**
      | Returns the name of the value at a given
      | index.
      | 
      | The index must be between 0 and size()
      | - 1.
      |
      */
    pub fn get_name(&self, index: i32) -> Identifier {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, values.size()))
            return values.getReference (index).name;

        jassertfalse;
        return {};
        */
    }
    
    /**
      | Returns the value of the item at a given
      | index.
      | 
      | The index must be between 0 and size()
      | - 1.
      |
      */
    pub fn get_value_at(&self, index: i32) -> &Var {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, values.size()))
            return values.getReference (index).value;

        jassertfalse;
        return getNullVarRef();
        */
    }
    
    /**
      | Returns the value of the item at a given
      | index.
      | 
      | The index must be between 0 and size()
      | - 1, or this will return a nullptr
      | 
      | Also note that the pointer returned
      | may become invalid as soon as any subsequent
      | methods are called on the NamedValueSet.
      |
      */
    pub fn get_var_pointer_mut_at(&mut self, index: i32) -> *mut Var {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, values.size()))
            return &(values.getReference (index).value);

        return {};
        */
    }
    
    /**
      | Returns the value of the item at a given
      | index.
      | 
      | The index must be between 0 and size()
      | - 1, or this will return a nullptr
      | 
      | Also note that the pointer returned
      | may become invalid as soon as any subsequent
      | methods are called on the NamedValueSet.
      |
      */
    pub fn get_var_pointer_at(&self, index: i32) -> *const Var {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, values.size()))
            return &(values.getReference (index).value);

        return {};
        */
    }
    
}
