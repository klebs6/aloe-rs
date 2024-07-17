crate::ix!();

#[no_copy]
#[leak_detector]
pub struct RemapperValueSource<'a> {
    base:         ValueSource<'a>,
    source_value: Value<'a>,
    mappings:     Vec<Var>,
}

impl<'a> ValueListener for RemapperValueSource<'a> {

    fn value_changed(&mut self, _0: &mut Value)  {
        
        todo!();
        /*
            sendChangeMessage (true);
        */

    }
}

impl<'a> RemapperValueSource<'a> {

    pub fn new(
        source: &Value,
        map:    &[Var]) -> Self {
    
        todo!();
        /*
        : source_value(source),
        : mappings(map),

            sourceValue.addListener (this);
        */

    }
    
    pub fn get_value(&self) -> Var {
        
        todo!();
        /*
            auto targetValue = sourceValue.getValue();

            for (auto& map : mappings)
                if (map.equalsWithSameType (targetValue))
                    return mappings.indexOf (map) + 1;

            return mappings.indexOf (targetValue) + 1;
        */

    }
    
    pub fn set_value(&mut self, new_value: &Var)  {
        
        todo!();
        /*
            auto remappedVal = mappings [static_cast<int> (newValue) - 1];

            if (! remappedVal.equalsWithSameType (sourceValue))
                sourceValue = remappedVal;
        */

    }
    
}
