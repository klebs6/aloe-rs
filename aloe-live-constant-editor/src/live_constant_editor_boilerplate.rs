/*!
  | You can safely ignore all the stuff in
  | this namespace - it's a bunch of boilerplate
  | code used to implement the ALOE_LIVE_CONSTANT
  | functionality.
  |
  */
crate::ix!();

pub fn live_constant_editor_parse_int(_0: String) -> i64 {
    
    todo!();
        /*
        
        */
}

pub fn live_constant_editor_parse_double(_0: &String) -> f64 {
    
    todo!();
        /*
        
        */
}

pub fn live_constant_editor_int_to_string(
        _0:         i32,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
        
        */
}

pub fn live_constant_editor_int_to_string(
        _0:         i64,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
        
        */
}

pub fn live_constant_editor_set_from_string<Type>(
        v: &mut Type,
        s: &String)  {

    todo!();
        /*
            v = static_cast<Type> (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut u8,
        s: &String)  {
    
    todo!();
        /*
            v = (char)           parseInt (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut u8,
        s: &String)  {
    
    todo!();
        /*
            v = (unsigned char)  parseInt (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut i16,
        s: &String)  {
    
    todo!();
        /*
            v = (short)          parseInt (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut u16,
        s: &String)  {
    
    todo!();
        /*
            v = (unsigned short) parseInt (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut i32,
        s: &String)  {
    
    todo!();
        /*
            v = (int)            parseInt (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut u32,
        s: &String)  {
    
    todo!();
        /*
            v = (unsigned int)   parseInt (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut i64,
        s: &String)  {
    
    todo!();
        /*
            v = (long)           parseInt (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut u64,
        s: &String)  {
    
    todo!();
        /*
            v = (unsigned long)  parseInt (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut i64,
        s: &String)  {
    
    todo!();
        /*
            v = (int64)          parseInt (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut u64,
        s: &String)  {
    
    todo!();
        /*
            v = (uint64)         parseInt (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut f64,
        s: &String)  {
    
    todo!();
        /*
            v = parseDouble (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut f32,
        s: &String)  {
    
    todo!();
        /*
            v = (float) parseDouble (s);
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut bool,
        s: &String)  {
    
    todo!();
        /*
            v = (s == "true");
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut String,
        s: &String)  {
    
    todo!();
        /*
            v = s;
        */
}

#[inline] pub fn live_constant_editor_set_from_string(
        v: &mut Colour,
        s: &String)  {
    
    todo!();
        /*
            v = Colour ((uint32) parseInt (s));
        */
}

#[inline] pub fn live_constant_editor_get_as_string<Type>(
        v:  &Type,
        _1: bool) -> String {

    todo!();
        /*
            return String (v);
        */
}

#[inline] pub fn live_constant_editor_get_as_string(
        v:          u8,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
            return intToString ((int) v, preferHex);
        */
}

#[inline] pub fn live_constant_editor_get_as_string(
        v:          u8,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
            return intToString ((int) v, preferHex);
        */
}

#[inline] pub fn live_constant_editor_get_as_string(
        v:          i16,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
            return intToString ((int) v, preferHex);
        */
}

#[inline] pub fn live_constant_editor_get_as_string(
        v:          u16,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
            return intToString ((int) v, preferHex);
        */
}

#[inline] pub fn live_constant_editor_get_as_string(
        v:          i32,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
            return intToString ((int) v, preferHex);
        */
}

#[inline] pub fn live_constant_editor_get_as_string(
        v:          u32,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
            return intToString ((int) v, preferHex);
        */
}

#[inline] pub fn live_constant_editor_get_as_string(
        v:  bool,
        _1: bool) -> String {
    
    todo!();
        /*
            return v ? "true" : "false";
        */
}

#[inline] pub fn live_constant_editor_get_as_string(
        v:          i64,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
            return intToString ((int64) v, preferHex);
        */
}

#[inline] pub fn live_constant_editor_get_as_string(
        v:          u64,
        prefer_hex: bool) -> String {
    
    todo!();
        /*
            return intToString ((int64) v, preferHex);
        */
}

#[inline] pub fn live_constant_editor_get_as_string(
        v:  Colour,
        _1: bool) -> String {
    
    todo!();
        /*
            return intToString ((int) v.getARGB(), true);
        */
}

lazy_static!{
    /*
    template <typename Type>    struct isStringType              { enum { value = 0 }; };
        template <>                 struct isStringType<String>      { enum { value = 1 }; };
    */
}

#[inline] pub fn live_constant_editor_get_as_code<Type>(
        v:          &mut Type,
        prefer_hex: bool) -> String {

    todo!();
        /*
            return getAsString (v, preferHex);
        */
}

#[inline] pub fn live_constant_editor_get_as_code(
        v:  Colour,
        _1: bool) -> String {
    
    todo!();
        /*
            return "Colour (0x" + String::toHexString ((int) v.getARGB()).paddedLeft ('0', 8) + ")";
        */
}

#[inline] pub fn live_constant_editor_get_as_code(
        v:  &String,
        _1: bool) -> String {
    
    todo!();
        /*
            return CppTokeniserFunctions::addEscapeChars(v).quoted();
        */
}

#[inline] pub fn live_constant_editor_get_as_code(
        v:  *const u8,
        _1: bool) -> String {
    
    todo!();
        /*
            return getAsCode (String (v), false);
        */
}

#[inline] pub fn live_constant_editor_cast_to_char_pointer<Type>(_0: &Type) -> *const u8 {

    todo!();
        /*
            return "";
        */
}

#[inline] pub fn live_constant_editor_cast_to_char_pointer(s: &String) -> *const u8 {
    
    todo!();
        /*
            return s.toRawUTF8();
        */
}

pub fn live_constant_editor_create_colour_editor(_0: &mut LivePropertyEditorBase) -> *mut Component {
    
    todo!();
        /*
        
        */
}

pub fn live_constant_editor_create_integer_slider(_0: &mut LivePropertyEditorBase) -> *mut Component {
    
    todo!();
        /*
        
        */
}

pub fn live_constant_editor_create_float_slider(_0: &mut LivePropertyEditorBase) -> *mut Component {
    
    todo!();
        /*
        
        */
}

pub fn live_constant_editor_create_bool_slider(_0: &mut LivePropertyEditorBase) -> *mut Component {
    
    todo!();
        /*
        
        */
}

lazy_static!{
    /*
    template <typename Type> struct CustomEditor    { static Component* create (LivePropertyEditorBase&)   { return nullptr; } };
        template <> struct CustomEditor<char>           { static Component* create (LivePropertyEditorBase& e) { return createIntegerSlider (e); } };
        template <> struct CustomEditor<unsigned char>  { static Component* create (LivePropertyEditorBase& e) { return createIntegerSlider (e); } };
        template <> struct CustomEditor<int>            { static Component* create (LivePropertyEditorBase& e) { return createIntegerSlider (e); } };
        template <> struct CustomEditor<unsigned int>   { static Component* create (LivePropertyEditorBase& e) { return createIntegerSlider (e); } };
        template <> struct CustomEditor<short>          { static Component* create (LivePropertyEditorBase& e) { return createIntegerSlider (e); } };
        template <> struct CustomEditor<unsigned short> { static Component* create (LivePropertyEditorBase& e) { return createIntegerSlider (e); } };
        template <> struct CustomEditor<int64>          { static Component* create (LivePropertyEditorBase& e) { return createIntegerSlider (e); } };
        template <> struct CustomEditor<uint64>         { static Component* create (LivePropertyEditorBase& e) { return createIntegerSlider (e); } };
        template <> struct CustomEditor<float>          { static Component* create (LivePropertyEditorBase& e) { return createFloatSlider (e); } };
        template <> struct CustomEditor<double>         { static Component* create (LivePropertyEditorBase& e) { return createFloatSlider (e); } };
        template <> struct CustomEditor<Colour>         { static Component* create (LivePropertyEditorBase& e) { return createColourEditor (e); } };
        template <> struct CustomEditor<bool>           { static Component* create (LivePropertyEditorBase& e) { return createBoolSlider (e); } };
    */
}

#[inline] pub fn live_constant_editor_get_value<Type>(
        file:          *const u8,
        line:          i32,
        initial_value: &Type) -> &mut LiveValue<Type> {

    todo!();
        /*
            // If you hit this assertion then the __FILE__ macro is providing a
            // relative path instead of an absolute path. On Windows this will be
            // a path relative to the build directory rather than the currently
            // running application. To fix this you must compile with the /FC flag.
            jassert (File::isAbsolutePath (file));

            return ValueList::getInstance()->getValue (file, line, initialValue);
        */
}

#[inline] pub fn live_constant_editor_get_value(
        file:          *const u8,
        line:          i32,
        initial_value: *const u8) -> &mut LiveValue<String> {
    
    todo!();
        /*
            return getValue (file, line, String (initialValue));
        */
}
