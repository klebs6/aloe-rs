crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/values/aloe_ValueWithDefault.h]

/**
  | This class acts as a wrapper around a
  | property inside a ValueTree.
  | 
  | If the property inside the ValueTree
  | is missing the ValueWithDefault will
  | automatically return a default value,
  | which can be specified when initialising
  | the ValueWithDefault.
  | 
  | @tags{DataStructures}
  |
  | Default creates an unitialised ValueWithDefault.
  | Initialise it using one of the referTo()
  | methods.
  |
  */
#[weak_referenceable]
pub struct ValueWithDefault<'a> {

    /**
      | You can assign a lambda to this callback
      | object to have it called when the default
      | value is changed.
      |
      */
    on_default_change: fn() -> (),
    target_tree:       ValueTree,
    target_property:   Identifier,
    undo_manager:      *mut UndoManager<'a>, // default = nullptr
    default_value:     Var,
    delimiter:         String,
}

impl<'a> Default for ValueWithDefault<'a> {

    fn default() -> Self {

        Self {
            on_default_change: no_op,
            target_tree:       ValueTree::default(),
            target_property:   Identifier::default(),
            undo_manager:      null_mut(),
            default_value:     Var::default(),
            delimiter:         String::default(),
        }
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/values/aloe_ValueWithDefault.cpp]
impl<'a> ValueWithDefault<'a> {

    /**
      | Creates an ValueWithDefault object.
      | The default value will be an empty Var.
      |
      */
    pub fn new_with_empty_var(
        tree:       &mut ValueTree,
        propertyid: &Identifier,
        um:         *mut UndoManager) -> Self {
    
        todo!();
        /*
        : target_tree(tree),
        : target_property(propertyID),
        : undo_manager(um),
        : default_value(),

        
        */
    }

    /**
      | Creates an ValueWithDefault object.
      | The default value will be defaultToUse.
      |
      */
    pub fn new_with_default(
        tree:           &mut ValueTree,
        propertyid:     &Identifier,
        um:             *mut UndoManager,
        default_to_use: &Var) -> Self {
    
        todo!();
        /*
        : target_tree(tree),
        : target_property(propertyID),
        : undo_manager(um),
        : default_value(defaultToUse),

        
        */
    }

    /**
      | Creates an ValueWithDefault object.
      | The default value will be defaultToUse.
      | 
      | Use this constructor if the underlying
      | Var object being controlled is an array
      | and it will handle the conversion to/from
      | a delimited String that can be written
      | to XML format.
      |
      */
    pub fn new_with_default_array(
        tree:            &mut ValueTree,
        propertyid:      &Identifier,
        um:              *mut UndoManager,
        default_to_use:  &Var,
        array_delimiter: &str) -> Self {
    
        todo!();
        /*
        : target_tree(tree),
        : target_property(propertyID),
        : undo_manager(um),
        : default_value(defaultToUse),
        : delimiter(arrayDelimiter),

        
        */
    }

    /**
      | Creates a ValueWithDefault object
      | from another ValueWithDefault object.
      |
      */
    pub fn new_from_other(other: &ValueWithDefault) -> Self {
    
        todo!();
        /*
        : target_tree(other.targetTree),
        : target_property(other.targetProperty),
        : undo_manager(other.undoManager),
        : default_value(other.defaultValue),
        : delimiter(other.delimiter),

        
        */
    }
    
    /**
      | Returns the current value of the property.
      | If the property does not exist this returns
      | the default value.
      |
      */
    pub fn get(&self) -> Var {
        
        todo!();
        /*
            if (isUsingDefault())
                return defaultValue;

            if (delimiter.isNotEmpty())
                return delimitedStringToVarArray (targetTree[targetProperty].toString());

            return targetTree[targetProperty];
        */
    }

    /**
      | Returns the current property as a Value
      | object.
      |
      */
    pub fn get_property_as_value(&mut self) -> Value {
        
        todo!();
        /*
            return targetTree.getPropertyAsValue (targetProperty, undoManager);
        */
    }

    /**
      | Returns the current default value.
      |
      */
    pub fn get_default(&self) -> Var {
        
        todo!();
        /*
            return defaultValue;
        */
    }

    /**
      | Sets the default value to a new Var.
      |
      */
    pub fn set_default(&mut self, new_default: &Var)  {
        
        todo!();
        /*
            if (defaultValue != newDefault)
            {
                defaultValue = newDefault;

                if (onDefaultChange != nullptr)
                    onDefaultChange();
            }
        */
    }

    /**
      | Returns true if the property does not
      | exist in the referenced ValueTree.
      |
      */
    pub fn is_using_default(&self) -> bool {
        
        todo!();
        /*
            return ! targetTree.hasProperty (targetProperty);
        */
    }

    /**
      | Removes the property from the referenced
      | ValueTree.
      |
      */
    pub fn reset_to_default(&mut self)  {
        
        todo!();
        /*
            targetTree.removeProperty (targetProperty, nullptr);
        */
    }
    
    /**
      | Sets the property and returns the new
      | ValueWithDefault. This will modify
      | the property in the referenced ValueTree.
      |
      */
    pub fn assign_from_var(&mut self, new_value: &Var) -> &mut ValueWithDefault {
        
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
    pub fn set_value(&mut self, 
        new_value:           &Var,
        undo_manager_to_use: *mut UndoManager)  {
        
        todo!();
        /*
            if (auto* array = newValue.getArray())
                targetTree.setProperty (targetProperty, varArrayToDelimitedString (*array), undoManagerToUse);
            else
                targetTree.setProperty (targetProperty, newValue, undoManagerToUse);
        */
    }
    
    /**
      | Makes the ValueWithDefault refer to
      | the specified property inside the given
      | ValueTree.
      |
      */
    pub fn refer_to(
        &mut self, 
        tree:     &mut ValueTree,
        property: &Identifier,
        um:       *mut UndoManager

    ) {
        
        todo!();
        /*
            referToWithDefault (tree, property, um, Var(), {});
        */
    }

    /**
      | Makes the ValueWithDefault refer to
      | the specified property inside the given
      | ValueTree, and specifies a default
      | value to use.
      |
      */
    pub fn refer_to_with_default(
        &mut self, 
        tree:        &mut ValueTree,
        property:    &Identifier,
        um:          *mut UndoManager,
        default_val: &Var

    ) {
        
        todo!();
        /*
            referToWithDefault (tree, property, um, defaultVal, {});
        */
    }
    
    pub fn refer_to_arr_with_default(
        &mut self, 
        tree:            &mut ValueTree,
        property:        &Identifier,
        um:              *mut UndoManager,
        default_val:     &Var,
        array_delimiter: &str

    ) {
        
        todo!();
        /*
            referToWithDefault (tree, property, um, defaultVal, arrayDelimiter);
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
    pub fn get_propertyid(&mut self) -> &mut Identifier {
        
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
    
    pub fn assign_from(&mut self, other: &ValueWithDefault) -> &mut ValueWithDefault {
        
        todo!();
        /*
            referToWithDefault (other.targetTree, other.targetProperty, other.undoManager,
                                other.defaultValue, other.delimiter);

            return *this;
        */
    }
    
    pub fn refer_to_with_default_and_delimiter(
        &mut self, 
        v:           &ValueTree,
        i:           &Identifier,
        um:          *mut UndoManager,
        default_val: &Var,
        del:         &str

    ) {
        
        todo!();
        /*
            targetTree = v;
            targetProperty = i;
            undoManager = um;
            defaultValue = defaultVal;
            delimiter = del;
        */
    }
    
    pub fn var_array_to_delimited_string(&self, input: &[Var]) -> String {
        
        todo!();
        /*
            // if you are trying to control a Var that is an array then you need to
            // set a delimiter string that will be used when writing to XML!
            jassert (delimiter.isNotEmpty());

            StringArray elements;

            for (auto& v : input)
                elements.add (v.toString());

            return elements.joinIntoString (delimiter);
        */
    }
    
    pub fn delimited_string_to_var_array(&self, input: &str) -> Vec<Var> {
        
        todo!();
        /*
            Vec<Var> arr;

            for (auto t : StringArray::fromTokens (input, delimiter, {}))
                arr.add (t);

            return arr;
        */
    }
}
