crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/values/aloe_CachedValue.h]

/**
  | This class acts as a typed wrapper around
  | a property inside a ValueTree.
  | 
  | A CachedValue provides an easy way to
  | read and write a ValueTree property
  | with a chosen type. So for example a CachedValue<int>
  | allows you to read or write the property
  | as an int, and a CachedValue<String>
  | lets you work with it as a String.
  | 
  | It also allows efficient access to the
  | value, by caching a copy of it in the type
  | that is being used.
  | 
  | You can give the CachedValue an optional
  | UndoManager which it will use when writing
  | to the underlying ValueTree.
  | 
  | If the property inside the ValueTree
  | is missing, the CachedValue will automatically
  | return an optional default value, which
  | can be specified when initialising
  | the CachedValue.
  | 
  | To create one, you can either use the
  | constructor to attach the CachedValue
  | to a ValueTree, or can create an uninitialised
  | CachedValue with its default constructor
  | and then attach it later with the referTo()
  | methods.
  | 
  | Common types like String, int, double
  | which can be easily converted to a var
  | should work out-of-the-box, but if
  | you want to use more complex custom types,
  | you may need to implement some template
  | specialisations of VariantConverter
  | which this class uses to convert between
  | the type and the ValueTree's internal
  | var.
  | 
  | @tags{DataStructures}
  |
  */
#[no_copy]
#[weak_referenceable]
pub struct CachedValue<'a,Type> {
    target_tree:     ValueTree,
    target_property: Identifier,
    undo_manager:    *mut UndoManager<'a>, // default = nullptr
    default_value:   Type,
    cached_value:    Type,
}

impl<'a,Type> ValueTreeListener for CachedValue<'a,Type> {

}

impl<'a,Type> Default for CachedValue<'a,Type> {
    
    /**
      | Default constructor.
      | 
      | Creates a default CachedValue not referring
      | to any property. To initialise the object,
      | call one of the referTo() methods.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<'a,Type> Deref for CachedValue<'a,Type> {

    type Target = Type;
    
    /**
      | Dereference operator. Provides direct
      | access to the property.
      |
      */
    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return cachedValue;
        */
    }
}

impl<'a,Type,OtherType> PartialEq<OtherType> for CachedValue<'a,Type> {
    
    /**
      | Returns true if the current value of
      | the property (or the fallback value)
      | is equal to other.
      |
      */
    #[inline] fn eq(&self, other: &OtherType) -> bool {
        todo!();
        /*
            return cachedValue == other;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/values/aloe_CachedValue.cpp]
impl<'a,Type> CachedValue<'a,Type> {
    
    #[inline] pub fn into_cached(self) -> Type {
        self.cached_value
    }

    /**
      | Returns the current value of the property.
      | If the property does not exist, returns
      | the fallback default value.
      |
      */
    pub fn get(&self) -> Type {
        
        todo!();
        /*
            return cachedValue;
        */
    }
    
    /**
      | Returns the current fallback default
      | value.
      |
      */
    pub fn get_default(&self) -> Type {
        
        todo!();
        /*
            return defaultValue;
        */
    }

    /**
      | Resets the fallback default value.
      |
      */
    pub fn set_default(&mut self, value: &Type)  {
        
        todo!();
        /*
            defaultValue = value;
        */
    }

    /**
      | Returns a reference to the ValueTree
      | containing the referenced property.
      |
      */
    pub fn get_value_tree(&mut self) -> &mut ValueTree {
        
        todo!();
        /*
            return targetTree;
        */
    }

    /**
      | Returns the property ID of the referenced
      | property.
      |
      */
    pub fn get_propertyid(&self) -> &Identifier {
        
        todo!();
        /*
            return targetProperty;
        */
    }

    /**
      | Returns the UndoManager that is being
      | used.
      |
      */
    pub fn get_undo_manager(&mut self) -> *mut UndoManager {
        
        todo!();
        /*
            return undoManager;
        */
    }
    
    /**
      | Constructor.
      | 
      | Creates a CachedValue referring to
      | a Value property inside a ValueTree.
      | If you use this constructor, the fallback
      | value will be a default-constructed
      | instance of Type.
      | 
      | -----------
      | @param tree
      | 
      | The ValueTree containing the property
      | ----------
      | @param propertyID
      | 
      | The identifier of the property
      | ----------
      | @param undoManager
      | 
      | The UndoManager to use when writing
      | to the property
      |
      */
    pub fn new(
        v:  &mut ValueTree,
        i:  &Identifier,
        um: *mut UndoManager) -> Self {
    
        todo!();
        /*
        : target_tree(v),
        : target_property(i),
        : undo_manager(um),
        : default_value(),
        : cached_value(getTypedValue()),

            targetTree.addListener (this);
        */
    }
    
    /**
      | Constructor.
      | 
      | Creates a default Cached Value referring
      | to a Value property inside a ValueTree,
      | and specifies a fallback value to use
      | if the property does not exist.
      | 
      | -----------
      | @param tree
      | 
      | The ValueTree containing the property
      | ----------
      | @param propertyID
      | 
      | The identifier of the property
      | ----------
      | @param undoManager
      | 
      | The UndoManager to use when writing
      | to the property
      | ----------
      | @param defaultToUse
      | 
      | The fallback default value to use.
      |
      */
    pub fn new_with_default(
        v:              &mut ValueTree,
        i:              &Identifier,
        um:             *mut UndoManager,
        default_to_use: &Type) -> Self {
    
        todo!();
        /*
        : target_tree(v),
        : target_property(i),
        : undo_manager(um),
        : default_value(defaultToUse),
        : cached_value(getTypedValue()),

            targetTree.addListener (this);
        */
    }
    
    /**
      | Returns the current property as a Value
      | object.
      |
      */
    #[inline] pub fn get_property_as_value(&mut self) -> Value {
        
        todo!();
        /*
            return targetTree.getPropertyAsValue (targetProperty, undoManager);
        */
    }
    
    /**
      | Returns true if the current property
      | does not exist and the CachedValue is
      | using the fallback default value instead.
      |
      */
    #[inline] pub fn is_using_default(&self) -> bool {
        
        todo!();
        /*
            return ! targetTree.hasProperty (targetProperty);
        */
    }
    
    /**
      | Sets the property. This will actually
      | modify the property in the referenced
      | ValueTree.
      |
      */
    #[inline] pub fn assign_from(&mut self, new_value: &Type) -> &mut CachedValue<Type> {
        
        todo!();
        /*
            setValue (newValue, undoManager);
        return *this;
        */
    }
    
    /**
      | Sets the property. This will actually
      | modify the property in the referenced
      | ValueTree.
      |
      */
    #[inline] pub fn set_value(&mut self, 
        new_value:           &Type,
        undo_manager_to_use: *mut UndoManager)  {
        
        todo!();
        /*
            if (cachedValue != newValue || isUsingDefault())
        {
            cachedValue = newValue;
            targetTree.setProperty (targetProperty, VariantConverter<Type>::toVar (newValue), undoManagerToUse);
        }
        */
    }
    
    /**
      | Removes the property from the referenced
      | ValueTree and makes the CachedValue
      | return the fallback default value instead.
      |
      */
    #[inline] pub fn reset_to_default(&mut self)  {
        
        todo!();
        /*
            resetToDefault (undoManager);
        */
    }
    
    /**
      | Removes the property from the referenced
      | ValueTree and makes the CachedValue
      | return the fallback default value instead.
      |
      */
    #[inline] pub fn reset_to_default_with_undoman(&mut self, undo_manager_to_use: *mut UndoManager)  {
        
        todo!();
        /*
            targetTree.removeProperty (targetProperty, undoManagerToUse);
        forceUpdateOfCachedValue();
        */
    }
    
    /**
      | Makes the CachedValue refer to the specified
      | property inside the given ValueTree.
      |
      */
    #[inline] pub fn refer_to(
        &mut self, 
        v:  &mut ValueTree,
        i:  &Identifier,
        um: *mut UndoManager

    )  {
        
        todo!();
        /*
            referToWithDefault (v, i, um, Type());
        */
    }
    
    /**
      | Force an update in case the referenced
      | property has been changed from elsewhere.
      | 
      | -----------
      | @note
      | 
      | The CachedValue is a ValueTree::Listener
      | and therefore will be informed of changes
      | of the referenced property anyway (and
      | update itself). But this may happen
      | asynchronously. forceUpdateOfCachedValue()
      | forces an update immediately.
      |
      */
    #[inline] pub fn force_update_of_cached_value(&mut self)  {
        
        todo!();
        /*
            cachedValue = getTypedValue();
        */
    }
    
    /**
      | Makes the CachedValue refer to the specified
      | property inside the given ValueTree,
      | and specifies a fallback value to use
      | if the property does not exist.
      |
      */
    #[inline] pub fn refer_to_with_default(
        &mut self, 
        v:           &mut ValueTree,
        i:           &Identifier,
        um:          *mut UndoManager,
        default_val: &Type

    ) {
        
        todo!();
        /*
            targetTree.removeListener (this);
        targetTree = v;
        targetProperty = i;
        undoManager = um;
        defaultValue = defaultVal;
        cachedValue = getTypedValue();
        targetTree.addListener (this);
        */
    }
    
    #[inline] pub fn get_typed_value(&self) -> Type {
        
        todo!();
        /*
            if (const var* property = targetTree.getPropertyPointer (targetProperty))
            return VariantConverter<Type>::fromVar (*property);

        return defaultValue;
        */
    }
    
    #[inline] pub fn value_tree_property_changed(&mut self, 
        changed_tree:     &mut ValueTree,
        changed_property: &Identifier)  {
        
        todo!();
        /*
            if (changedProperty == targetProperty && targetTree == changedTree)
            forceUpdateOfCachedValue();
        */
    }
}
