crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_LiveConstantEditor.cpp]

pub fn live_constant_editor_parse_int(s: String) -> i64 {
    
    todo!();
        /*
            s = s.trimStart();

        if (s.startsWithChar ('-'))
            return -parseInt (s.substring (1));

        if (s.startsWith ("0x"))
            return s.substring(2).getHexValue64();

        return s.getLargeIntValue();
        */
}

pub fn live_constant_editor_parse_double(s: &String) -> f64 {
    
    todo!();
        /*
            return s.retainCharacters ("0123456789.eE-").getDoubleValue();
        */
}

pub fn live_constant_editor_int_to_string(
        v:          i32,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
            return preferHex ? "0x" + String::toHexString (v) : String (v);
        */
}

pub fn live_constant_editor_int_to_string_i64(
        v:          i64,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
            return preferHex ? "0x" + String::toHexString (v) : String (v);
        */
}
