crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/values/aloe_Value.h]

/**
  | Receives callbacks when a Value object
  | changes. @see Value::addListener
  |
  */
pub trait ValueListener {

    /**
      | Called when a Value object is changed.
      | 
      | -----------
      | @note
      | 
      | that the Value object passed as a parameter
      | may not be exactly the same object that
      | you registered the listener with - it
      | might be a copy that refers to the same
      | underlying ValueSource. To find out,
      | you can call Value::refersToSameSourceAs().
      |
      */
    fn value_changed(&mut self, value: &mut Value);
}

/**
  | Represents a shared variant value.
  | 
  | A Value object contains a reference
  | to a Var object, and can get and set its
  | value. Listeners can be attached to
  | be told when the value is changed.
  | 
  | The Value class is a wrapper around a
  | shared, reference-counted underlying
  | data object - this means that multiple
  | Value objects can all refer to the same
  | piece of data, allowing all of them to
  | be notified when any of them changes
  | it.
  | 
  | When you create a Value with its default
  | constructor, it acts as a wrapper around
  | a simple Var object, but by creating
  | a Value that refers to a custom subclass
  | of ValueSource, you can map the Value
  | onto any kind of underlying data.
  | 
  | Important note! The Value class is not
  | thread-safe! If you're accessing one
  | from multiple threads, then you'll
  | need to use your own synchronisation
  | around any code that accesses it.
  | 
  | @tags{DataStructures}
  |
  */
pub struct Value<'a> {
    value:     ReferenceCountedObjectPtr<ValueSource<'a>>,
    listeners: ListenerList<Box<dyn ValueListener>>,
}

impl<'a> fmt::Display for Value<'a> {

    /**
      | Writes a Value to an OutputStream as
      | a UTF8 string.
      |
      */
    fn fmt(
        &self, 
        fmt: &mut fmt::Formatter<'_>

    ) -> Result<(),fmt::Error> {

        todo!();
        /*
            return stream << value.toString();
        */
    }
}

impl<'a> Default for Value<'a> {
    
    /**
      | Creates an empty Value, containing
      | a void Var.
      |
      */
    fn default() -> Self {
        todo!();
        /*
            : value (new SimpleValueSource()
        */
    }
}

impl<'a> Into<Var> for Value<'a> {
    
    /**
      | Returns the current value.
      |
      */
    #[inline] fn into(self) -> Var {
        todo!();
        /*
            return value->getValue();
        */
    }
}

impl<'a> PartialEq<Value<'a>> for Value<'a> {
    
    /**
      | Compares two values.
      | 
      | This is a compare-by-value comparison,
      | so is effectively the same as saying
      | (this->getValue() == other.getValue()).
      |
      */
    #[inline] fn eq(&self, other: &Value<'a>) -> bool {
        todo!();
        /*
            return value == other.value || value->getValue() == other.getValue();
        */
    }
}

impl<'a> Eq for Value<'a> {}

impl<'a> Value<'a> {

    /**
      | Returns the ValueSource that this value
      | is referring to.
      |
      */
    pub fn get_value_source(&mut self) -> &mut ValueSource {
        
        todo!();
        /*
            return *value;
        */
    }

    /**
      | Creates a Value object that uses this
      | valueSource object as its underlying
      | data.
      |
      */
    pub fn new_from_value_source(v: *mut ValueSource) -> Self {
    
        todo!();
        /*
        : value(v),

            jassert (v != nullptr);
        */
    }
    
    /**
      | Creates a Value that is set to the specified
      | value.
      |
      */
    pub fn new_from_initial_value(initial_value: &Var) -> Self {
    
        todo!();
        /*


            : value (new SimpleValueSource (initialValue))
        */
    }
    
    /**
      | Creates a Value that refers to the same
      | value as another one.
      | 
      | -----------
      | @note
      | 
      | this doesn't make a copy of the other
      | value - both this and the other Value
      | will share the same underlying value,
      | so that when either one alters it, both
      | will see it change.
      |
      */
    pub fn new_from_other_ref(other: &Value) -> Self {
    
        todo!();
        /*


            : value (other.value)
        */
    }
    
    pub fn new_from_other(other: Value) -> Self {
    
        todo!();
        /*


            // moving a Value with listeners will lose those listeners, which
        // probably isn't what you wanted to happen!
        jassert (other.listeners.size() == 0);

        other.removeFromListenerList();
        value = std::move (other.value);
        */
    }
    
    /**
      | Move assignment operator
      |
      */
    pub fn assign_from_other(&mut self, other: Value) -> &mut Value {
        
        todo!();
        /*
            // moving a Value with listeners will lose those listeners, which
        // probably isn't what you wanted to happen!
        jassert (other.listeners.size() == 0);

        other.removeFromListenerList();
        value = std::move (other.value);
        return *this;
        */
    }

    pub fn remove_from_listener_list(&mut self)  {
        
        todo!();
        /*
            if (listeners.size() > 0 && value != nullptr) // may be nullptr after a move operation
            value->valuesWithListeners.removeValue (this);
        */
    }

    /**
      | Returns the current value.
      |
      */
    pub fn get_value(&self) -> Var {
        
        todo!();
        /*
            return value->getValue();
        */
    }

    /**
      | Sets the current value.
      | 
      | You can also use operator= to set the
      | value.
      | 
      | If there are any listeners registered,
      | they will be notified of the change asynchronously.
      |
      */
    pub fn set_value(&mut self, new_value: &Var)  {
        
        todo!();
        /*
            value->setValue (newValue);
        */
    }

    /**
      | Returns the value as a string.
      | 
      | This is a shortcut for "myValue.getValue().toString()".
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return value->getValue().toString();
        */
    }
    
    /**
      | Sets the current value.
      | 
      | This is the same as calling setValue().
      | 
      | If there are any listeners registered,
      | they will be notified of the change asynchronously.
      |
      */
    pub fn assign_from_var(&mut self, new_value: &Var) -> &mut Value {
        
        todo!();
        /*
            value->setValue (newValue);
        return *this;
        */
    }

    /**
      | Makes this object refer to the same underlying
      | ValueSource as another one.
      | 
      | Once this object has been connected
      | to another one, changing either one
      | will update the other.
      | 
      | Existing listeners will still be registered
      | after you call this method, and they'll
      | continue to receive messages when the
      | new value changes.
      |
      */
    pub fn refer_to(&mut self, value_to_refer_to: &Value)  {
        
        todo!();
        /*
            if (valueToReferTo.value != value)
        {
            if (listeners.size() > 0)
            {
                value->valuesWithListeners.removeValue (this);
                valueToReferTo.value->valuesWithListeners.add (this);
            }

            value = valueToReferTo.value;
            callListeners();
        }
        */
    }
    
    /**
      | Returns true if this value and the other
      | one are references to the same value.
      |
      */
    pub fn refers_to_same_source_as(&self, other: &Value) -> bool {
        
        todo!();
        /*
            return value == other.value;
        */
    }
    
    /**
      | Adds a listener to receive callbacks
      | when the value changes.
      | 
      | The listener is added to this specific
      | Value object, and not to the shared object
      | that it refers to. When this object is
      | deleted, all the listeners will be lost,
      | even if other references to the same
      | Value still exist. So when you're adding
      | a listener, make sure that you add it
      | to a Value instance that will last for
      | as long as you need the listener. In general,
      | you'd never want to add a listener to
      | a local stack-based Value, but more
      | likely to one that's a member variable.
      | 
      | @see removeListener
      |
      */
    pub fn add_listener(&mut self, listener: *mut dyn ValueListener)  {
        
        todo!();
        /*
            if (listener != nullptr)
        {
            if (listeners.size() == 0)
                value->valuesWithListeners.add (this);

            listeners.add (listener);
        }
        */
    }
    
    /**
      | Removes a listener that was previously
      | added with addListener().
      |
      */
    pub fn remove_listener(&mut self, listener: *mut dyn ValueListener)  {
        
        todo!();
        /*
            listeners.remove (listener);

        if (listeners.size() == 0)
            value->valuesWithListeners.removeValue (this);
        */
    }
    
    pub fn call_listeners(&mut self)  {
        
        todo!();
        /*
            if (listeners.size() > 0)
        {
            Value v (*this); // (create a copy in case this gets deleted by a callback)
            listeners.call ([&] (Value::ValueListener& l) { l.valueChanged (v); });
        }
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/values/aloe_Value.cpp]

impl<'a> Drop for Value<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
        removeFromListenerList();
 */
    }
}
