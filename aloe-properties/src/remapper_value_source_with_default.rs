crate::ix!();

#[no_copy]
#[leak_detector]
pub struct RemapperValueSourceWithDefault<'a> {
    base:               ValueSource<'a>,
    value_with_default: WeakReference<ValueWithDefault<'a>>,
    source_value:       Value<'a>,
    mappings:           Vec<Var>,
}

impl<'a> ValueListener for RemapperValueSourceWithDefault<'a> {

    fn value_changed(&mut self, _0: &mut Value)  {
        
        todo!();
        /*
            sendChangeMessage (true);
        */

    }
}

impl<'a> RemapperValueSourceWithDefault<'a> {
    
    pub fn new(
        vwd: *mut ValueWithDefault,
        map: &[Var]) -> Self {
    
        todo!();
        /*


            : valueWithDefault (vwd),
              sourceValue (valueWithDefault->getPropertyAsValue()),
              mappings (map)

            sourceValue.addListener (this);
        */

    }
    
    pub fn get_value(&self) -> Var {
        
        todo!();
        /*
            if (valueWithDefault != nullptr && ! valueWithDefault->isUsingDefault())
            {
                const auto target = sourceValue.getValue();
                const auto equalsWithSameType = [&target] (const var& map) { return map.equalsWithSameType (target); };

                auto iter = std::find_if (mappings.begin(), mappings.end(), equalsWithSameType);

                if (iter == mappings.end())
                    iter = std::find (mappings.begin(), mappings.end(), target);

                if (iter != mappings.end())
                    return 1 + (int) std::distance (mappings.begin(), iter);
            }

            return -1;
        */

    }
    
    pub fn set_value(&mut self, new_value: &Var)  {
        
        todo!();
        /*
            if (valueWithDefault == nullptr)
                return;

            auto newValueInt = static_cast<int> (newValue);

            if (newValueInt == -1)
            {
                valueWithDefault->resetToDefault();
            }
            else
            {
                auto remappedVal = mappings [newValueInt - 1];

                if (! remappedVal.equalsWithSameType (sourceValue))
                    *valueWithDefault = remappedVal;
            }
        */

    }
    
}
