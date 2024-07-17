crate::ix!();

pub trait DynamicObjectInterface {

    /**
      | Returns true if the object has a property
      | with this name.
      | 
      | -----------
      | @note
      | 
      | if the property is actually a method,
      | this will return false.
      |
      */
    fn has_property(&self, property_name: &Identifier) -> bool;

    /**
      | Returns a named property. This returns
      | var() if no such property exists.
      |
      */
    fn get_property(&self, property_name: &Identifier) -> &Var;

    /**
      | Sets a named property.
      |
      */
    fn set_property(
        &mut self, 
        property_name: &Identifier,
        new_value:     &Var
    );

    /**
      | Removes a named property.
      |
      */
    fn remove_property(&mut self, property_name: &Identifier);

    /**
      | Checks whether this object has the specified
      | method.
      | 
      | The default implementation of this
      | just checks whether there's a property
      | with this name that's actually a method,
      | but this can be overridden for building
      | objects with dynamic invocation.
      |
      */
    fn has_method(&self, method_name: &Identifier) -> bool;

    /**
      | Invokes a named method on this object.
      | 
      | The default implementation looks up
      | the named property, and if it's a method
      | call, then it invokes it.
      | 
      | This method is virtual to allow more
      | dynamic invocation to used for objects
      | where the methods may not already be
      | set as properties.
      |
      */
    fn invoke_method(
        &mut self, 
        method_name: Identifier,
        args:        &VarNativeFunctionArgs
    ) -> Var;

    /**
      | Returns a clone of this object.
      | 
      | The default implementation of this
      | method just returns a new DynamicObject
      | with a (deep) copy of all of its properties.
      | Subclasses can override this to implement
      | their own custom copy routines.
      |
      */
    fn clone(&mut self) -> DynamicObjectPtr;

    /**
      | Writes this object to a text stream in
      | JSON format.
      | 
      | This method is used by JSON::toString
      | and JSON::writeToStream, and you should
      | never need to call it directly, but it's
      | virtual so that custom object types
      | can stringify themselves appropriately.
      |
      */
    fn write_asjson<W: Write>(
        &mut self, 
        output_stream:          &mut W,
        indent_level:           i32,
        all_on_one_line:        bool,
        maximum_decimal_places: i32
    );
}
