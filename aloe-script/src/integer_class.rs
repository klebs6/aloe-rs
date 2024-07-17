crate::ix!();

pub struct IntegerClass {
    base: DynamicObject,
}

impl Default for IntegerClass {
    
    fn default() -> Self {
        todo!();
        /*


            setMethod ("parseInt",  parseInt);
        */
    }
}

impl IntegerClass {
    
    pub fn get_class_name() -> Identifier {
        
        todo!();
        /*
            static const Identifier i ("Integer"); return i;
        */
    }
    
    pub fn parse_int(a: Args) -> Var {
        
        todo!();
        /*
            auto s = getString (a, 0).trim();

            return s[0] == '0' ? (s[1] == 'x' ? s.substring(2).getHexValue64() : getOctalValue (s))
                : s.getLargeIntValue();
        */
    }
}
