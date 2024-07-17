crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_ScopedValueSetter.h]

/**
  | Helper class providing an RAII-based mechanism
  | for temporarily setting and then re-setting
  | a value.
  |
  | E.g. @code
  | int x = 1;
  |
  | {
  |     ScopedValueSetter setter (x, 2);
  |
  |     // x is now 2
  | }
  |
  | // x is now 1 again
  |
  | {
  |     ScopedValueSetter setter (x, 3, 4);
  |
  |     // x is now 3
  | }
  |
  | // x is now 4
  | @endcode
  |
  | @tags{Core}
  */
#[no_copy]
pub struct ScopedValueSetter<'a, ValueType> {
    value:          &'a mut ValueType,
    original_value: ValueType,
}

impl<'a, ValueType> Drop for ScopedValueSetter<'a, ValueType> {

    fn drop(&mut self) {

        todo!();
        /* 
            value = originalValue;
         */
    }
}

impl<'a, ValueType> ScopedValueSetter<'a, ValueType> {

    /**
      | Creates a ScopedValueSetter that will
      | immediately change the specified value
      | to the given new value, and will then
      | reset it to its original value when this
      | object is deleted.
      |
      */
    pub fn new(
        value_to_set: &mut ValueType,
        new_value:    ValueType) -> Self {
    
        todo!();
        /*
        : value(valueToSet),
        : original_value(valueToSet),

            valueToSet = newValue;
        */
    }

    /**
      | Creates a ScopedValueSetter that will
      | immediately change the specified value
      | to the given new value, and will then
      | reset it to be valueWhenDeleted when
      | this object is deleted.
      |
      */
    pub fn new_with_value_when_deleted(
        value_to_set:       &mut ValueType,
        new_value:          ValueType,
        value_when_deleted: ValueType) -> Self {
    
        todo!();
        /*
        : value(valueToSet),
        : original_value(valueWhenDeleted),

            valueToSet = newValue;
        */
    }
}
