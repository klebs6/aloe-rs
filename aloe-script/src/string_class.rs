crate::ix!();

pub struct StringClass {
    base: DynamicObject,
}

impl Default for StringClass {
    
    fn default() -> Self {
        todo!();
        /*


            setMethod ("substring",     substring);
            setMethod ("indexOf",       indexOf);
            setMethod ("charAt",        charAt);
            setMethod ("charCodeAt",    charCodeAt);
            setMethod ("fromCharCode",  fromCharCode);
            setMethod ("split",         split);
        */
    }
}

impl StringClass {
    
    pub fn get_class_name() -> Identifier {
        
        todo!();
        /*
            static const Identifier i ("String"); return i;
        */
    }
    
    pub fn from_char_code(a: Args) -> Var {
        
        todo!();
        /*
            return String::charToString (static_cast<aloe_wchar> (getInt (a, 0)));
        */
    }
    
    pub fn substring(a: Args) -> Var {
        
        todo!();
        /*
            return a.thisObject.toString().substring (getInt (a, 0), getInt (a, 1));
        */
    }
    
    pub fn index_of(a: Args) -> Var {
        
        todo!();
        /*
            return a.thisObject.toString().indexOf (getString (a, 0));
        */
    }
    
    pub fn char_code_at(a: Args) -> Var {
        
        todo!();
        /*
            return (int) a.thisObject.toString() [getInt (a, 0)];
        */
    }
    
    pub fn char_at(a: Args) -> Var {
        
        todo!();
        /*
            int p = getInt (a, 0); return a.thisObject.toString().substring (p, p + 1);
        */
    }
    
    pub fn split(a: Args) -> Var {
        
        todo!();
        /*
            auto str = a.thisObject.toString();
            auto sep = getString (a, 0);
            StringArray strings;

            if (sep.isNotEmpty())
                strings.addTokens (str, sep.substring (0, 1), {});
            else // special-case for empty separator: split all chars separately
                for (auto pos = str.getCharPointer(); ! pos.isEmpty(); ++pos)
                    strings.add (String::charToString (*pos));

            Var array;

            for (auto& s : strings)
                array.append (s);

            return array;
        */
    }
}
