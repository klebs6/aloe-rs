crate::ix!();

#[no_copy]
#[leak_detector]
pub struct MultiChoiceRemapperSourceWithDefault<'a> {
    base:               ValueSource<'a>,
    value_with_default: WeakReference<ValueWithDefault<'a>>,
    var_to_control:     Var,
    source_value:       Value<'a>,
    max_choices:        i32,
    button_to_control:  *mut ToggleButton<'a>,
}

impl<'a> ValueListener for MultiChoiceRemapperSourceWithDefault<'a> {

    fn value_changed(&mut self, _0: &mut Value)  {
        
        todo!();
        /*
            sendChangeMessage (true);
        */

    }
}

impl<'a> MultiChoiceRemapperSourceWithDefault<'a> {
    
    pub fn new(
        vwd: *mut ValueWithDefault,
        v:   Var,
        c:   i32,
        b:   *mut ToggleButton) -> Self {
    
        todo!();
        /*


            : valueWithDefault (vwd),
              varToControl (v),
              sourceValue (valueWithDefault->getPropertyAsValue()),
              maxChoices (c),
              buttonToControl (b)
            sourceValue.addListener (this);
        */

    }
    
    pub fn get_value(&self) -> Var {
        
        todo!();
        /*
            if (valueWithDefault == nullptr)
                return {};

            auto v = valueWithDefault->get();

            if (auto* arr = v.getArray())
            {
                if (arr->contains (varToControl))
                {
                    updateButtonTickColour (buttonToControl, valueWithDefault->isUsingDefault());
                    return true;
                }
            }

            return false;
        */

    }
    
    pub fn set_value(&mut self, new_value: &Var)  {
        
        todo!();
        /*
            if (valueWithDefault == nullptr)
                return;

            auto v = valueWithDefault->get();

            OptionalScopedPointer<Vec<var>> arrayToControl;

            if (valueWithDefault->isUsingDefault())
                arrayToControl.set (new Vec<var>(), true); // use an empty array so the default values are overwritten
            else
                arrayToControl.set (v.getArray(), false);

            if (arrayToControl != nullptr)
            {
                auto temp = *arrayToControl;

                bool newState = newValue;

                if (valueWithDefault->isUsingDefault())
                {
                    if (auto* defaultArray = v.getArray())
                    {
                        if (defaultArray->contains (varToControl))
                            newState = true; // force the state as the user is setting it explicitly
                    }
                }

                if (newState)
                {
                    if (temp.addIfNotAlreadyThere (varToControl) && (maxChoices != -1) && (temp.size() > maxChoices))
                        temp.remove (temp.size() - 2);
                }
                else
                {
                    temp.remove (temp.indexOf (varToControl));
                }

                StringComparator c;
                temp.sort (c);

                *valueWithDefault = temp;

                if (temp.size() == 0)
                    valueWithDefault->resetToDefault();
            }
        */

    }
    
}
