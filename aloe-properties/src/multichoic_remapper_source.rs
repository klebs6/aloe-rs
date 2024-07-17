crate::ix!();

#[no_copy]
#[leak_detector]
pub struct MultiChoiceRemapperSource<'a> {
    base:           ValueSource<'a>,
    source_value:   Value<'a>,
    var_to_control: Var,
    max_choices:    i32,
}

impl<'a> ValueListener for MultiChoiceRemapperSource<'a> {

    fn value_changed(&mut self, _0: &mut Value)  {
        
        todo!();
        /*
            sendChangeMessage (true);
        */

    }
}

impl<'a> MultiChoiceRemapperSource<'a> {

    pub fn new(
        source: &Value,
        v:      Var,
        c:      i32) -> Self {
    
        todo!();
        /*
        : source_value(source),
        : var_to_control(v),
        : max_choices(c),

            sourceValue.addListener (this);
        */

    }
    
    pub fn get_value(&self) -> Var {
        
        todo!();
        /*
            if (auto* arr = sourceValue.getValue().getArray())
                if (arr->contains (varToControl))
                    return true;

            return false;
        */

    }
    
    pub fn set_value(&mut self, new_value: &Var)  {
        
        todo!();
        /*
            if (auto* arr = sourceValue.getValue().getArray())
            {
                auto temp = *arr;

                if (static_cast<bool> (newValue))
                {
                    if (temp.addIfNotAlreadyThere (varToControl) && (maxChoices != -1) && (temp.size() > maxChoices))
                         temp.remove (temp.size() - 2);
                }
                else
                {
                    temp.remove (arr->indexOf (varToControl));
                }

                StringComparator c;
                temp.sort (c);

                sourceValue = temp;
            }
        */

    }
    
}
