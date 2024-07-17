/*!
  | An abstract interface representing
  | the value of an accessibility element.
  | 
  | Values should be used when information
  | needs to be conveyed which cannot be
  | represented by the accessibility element's
  | label alone. For example, a gain slider
  | with the label "Gain" needs to also provide
  | a value for its position whereas a "Save"
  | button does not.
  | 
  | This class allows for full control over
  | the value text/numeric conversion,
  | ranged, and read-only properties but
  | in most cases you'll want to use one of
  | the derived classes below which handle
  | some of this for you.
  | 
  | @see AccessibilityTextValueInterface,
  | AccessibilityNumericValueInterface,
  | AccessibilityRangedNumericValueInterface
  | 
  | @tags{Accessibility}
  |
  */
crate::ix!();

/**
  | The minimum and maximum values for this
  | range, inclusive.
  |
  */
pub struct MinAndMax  { 
    min: f64,
    max: f64,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/accessibility/interfaces/aloe_AccessibilityValueInterface.h]

pub trait IsReadOnly {

    /**
      | Returns true if the value is read-only
      | and cannot be modified by an accessibility
      | client. @see setValue, setValueAsString
      |
      */
    fn is_read_only(&self) -> bool;
}

pub trait GetCurrentValue {
    /**
      | Returns the current value as a double.
      |
      */
    fn get_current_value(&self) -> f64;

}

pub trait GetCurrentValueAsString {

    /**
      | Returns the current value as a String.
      |
      */
    fn get_current_value_as_string(&self) -> String;
}

pub trait SetValue {

    /**
      | Sets the current value to a new double
      | value.
      |
      */
    fn set_value(&mut self, new_value: f64);
}

pub trait SetValueAsString {

    /**
      | Sets the current value to a new String
      | value.
      |
      */
    fn set_value_as_string(&mut self, new_value: &String);
}

pub trait GetAccessibilityValueRange {

    /**
      | If this is a ranged value, this should
      | return a valid AccessibleValueRange
      | object representing the supported
      | numerical range.
      |
      */
    fn get_range(&self) -> AccessibleValueRange {

        todo!();
        /*
            return {};
        */
    }
}

pub trait AccessibilityValueInterface: 
IsReadOnly 
+ GetCurrentValue 
+ GetCurrentValueAsString
+ SetValue
+ SetValueAsString
+ GetAccessibilityValueRange { }

/**
  | A value interface that represents a
  | text value.
  | 
  | @tags{Accessibility}
  |
  */
pub trait AccessibilityTextValueInterface: 
IsReadOnly
+ GetCurrentValueAsString
+ SetValueAsString 
+ AccessibilityValueInterface
{ }

impl<T> SetValue for T 
where T: AccessibilityTextValueInterface {

    fn set_value(&mut self, new_value: f64)  {
        
        todo!();
        /*
            setValueAsString (String (newValue));
        */
    }
}

impl<T> GetCurrentValue for T 
where T: AccessibilityTextValueInterface {

    fn get_current_value(&self) -> f64 {
        
        todo!();
        /*
            return getCurrentValueAsString().getDoubleValue();
        */
    }
}

/**
  | A value interface that represents a
  | non-ranged numeric value.
  | 
  | @tags{Accessibility}
  |
  */
pub trait AccessibilityNumericValueInterface: 
IsReadOnly 
+ GetCurrentValue 
+ SetValue 
+ AccessibilityValueInterface 
{ }

impl<T> GetCurrentValueAsString for T where T: AccessibilityNumericValueInterface {

    fn get_current_value_as_string(&self) -> String {
        
        todo!();
        /*
            return String (getCurrentValue());
        */
    }
}

impl<T> SetValueAsString for T where T: AccessibilityNumericValueInterface {

    fn set_value_as_string(&mut self, new_value: &String)  {
        
        todo!();
        /*
            setValue (newValue.getDoubleValue());
        */
    }
}

/**
  | A value interface that represents a
  | ranged numeric value.
  | 
  | @tags{Accessibility}
  |
  */
pub trait AccessibilityRangedNumericValueInterface: 
IsReadOnly 
+ GetCurrentValue 
+ SetValue 
+ GetAccessibilityValueRange 
+ AccessibilityValueInterface 
{ }
