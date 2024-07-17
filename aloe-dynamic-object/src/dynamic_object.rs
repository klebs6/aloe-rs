crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_DynamicObject.h]

/**
  | Represents a dynamically implemented
  | object.
  | 
  | This class is primarily intended for
  | wrapping scripting language objects,
  | but could be used for other purposes.
  | 
  | An instance of a DynamicObject can be
  | used to store named properties, and
  | by subclassing hasMethod() and invokeMethod(),
  | you can give your object methods.
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub struct DynamicObject {
    base:       ReferenceCountedObject,
    properties: NamedValueSet,
}

pub type DynamicObjectPtr = ReferenceCountedObjectPtr<DynamicObject>;

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_DynamicObject.cpp]
impl DynamicObject {
   
    /**
      | Returns the NamedValueSet that holds
      | the object's properties.
      |
      */
    pub fn get_properties(&mut self) -> &mut NamedValueSet {
        
        todo!();
        /*
            return properties;
        */
    }

    pub fn new(other: &DynamicObject) -> Self {
    
        todo!();
        /*
        : reference_counted_object(),
        : properties(other.properties),
        */
    }
    
    pub fn has_property(&self, property_name: &Identifier) -> bool {
        
        todo!();
        /*
            const var* const v = properties.getVarPointer (propertyName);
        return v != nullptr && ! v->isMethod();
        */
    }
    
    pub fn get_property(&self, property_name: &Identifier) -> &Var {
        
        todo!();
        /*
            return properties [propertyName];
        */
    }
    
    pub fn set_property(
        &mut self, 
        property_name: &Identifier,
        new_value:     &Var)  {
        
        todo!();
        /*
            properties.set (propertyName, newValue);
        */
    }
    
    pub fn remove_property(&mut self, property_name: &Identifier)  {
        
        todo!();
        /*
            properties.remove (propertyName);
        */
    }
    
    pub fn has_method(&self, method_name: &Identifier) -> bool {
        
        todo!();
        /*
            return getProperty (methodName).isMethod();
        */
    }
    
    pub fn invoke_method(&mut self, 
        method: Identifier,
        args:   &VarNativeFunctionArgs) -> Var {
        
        todo!();
        /*
            if (auto function = properties [method].getNativeFunction())
            return function (args);

        return {};
        */
    }
    
    /**
      | Adds a method to the class.
      | 
      | This is basically the same as calling
      | setProperty (methodName, (VarNativeFunction)
      | myFunction), but helps to avoid accidentally
      | invoking the wrong type of var constructor.
      | It also makes the code easier to read.
      |
      */
    pub fn set_method(&mut self, 
        name:     Identifier,
        function: VarNativeFunction)  {
        
        todo!();
        /*
            properties.set (name, var (function));
        */
    }
    
    /**
      | Removes all properties and methods
      | from the object.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            properties.clear();
        */
    }
    
    /**
      | Calls Varclone() on all the properties
      | that this object contains.
      |
      */
    pub fn clone_all_properties(&mut self)  {
        
        todo!();
        /*
            for (int i = properties.size(); --i >= 0;)
            if (auto* v = properties.getVarPointerAt (i))
                *v = v->clone();
        */
    }
    
    pub fn clone(&mut self) -> DynamicObjectPtr {
        
        todo!();
        /*
            Ptr d (new DynamicObject (*this));
        d->cloneAllProperties();
        return d;
        */
    }
    
    pub fn write_asjson<W: Write>(
        &mut self, 
        out:                    &mut W,
        indent_level:           i32,
        all_on_one_line:        bool,
        maximum_decimal_places: i32)  {
        
        todo!();
        /*
            out << '{';
        if (! allOnOneLine)
            out << newLine;

        const int numValues = properties.size();

        for (int i = 0; i < numValues; ++i)
        {
            if (! allOnOneLine)
                JSONFormatter::writeSpaces (out, indentLevel + JSONFormatter::indentSize);

            out << '"';
            JSONFormatter::writeString (out, properties.getName (i));
            out << "\": ";
            JSONFormatter::write (out, properties.getValueAt (i), indentLevel + JSONFormatter::indentSize, allOnOneLine, maximumDecimalPlaces);

            if (i < numValues - 1)
            {
                if (allOnOneLine)
                    out << ", ";
                else
                    out << ',' << newLine;
            }
            else if (! allOnOneLine)
                out << newLine;
        }

        if (! allOnOneLine)
            JSONFormatter::writeSpaces (out, indentLevel);

        out << '}';
        */
    }
}
